use macroquad::prelude::*;

// Module declarations
mod animation;
mod atom;
mod camera;
mod camera_state;
mod color_maps;
mod input;
mod pdb_parser;
mod render_cache;
mod types;
mod ui;
mod visualization_state;
mod web_api;

#[cfg(not(target_arch = "wasm32"))]
mod export;

// Use declarations
use animation::{AnimationMode, AnimationState};
use atom::Atom;
use camera::{calculate_bounding_box, calculate_initial_radius};
use camera_state::CameraState;
use input::{
    handle_animation_controls, handle_camera_controls, handle_mode_switches, handle_model_rotation,
    handle_radius_scale,
};
use pdb_parser::load_pdb;
use render_cache::MolecularRenderer;
use ui::{ColorPickerState, UIState, draw_color_picker, draw_info_overlay, render_atoms_optimized};
use visualization_state::VisualizationState;
use web_api::{WebCommand, poll_commands};

#[cfg(not(target_arch = "wasm32"))]
use input::handle_export;

#[cfg(not(target_arch = "wasm32"))]
use export::{ExportFormat, export_svg};

// For native builds, use command line args
#[cfg(not(target_arch = "wasm32"))]
use std::env;

// For WASM builds, embed default PDB data
#[cfg(target_arch = "wasm32")]
const DEFAULT_PDB_DATA: &str = include_str!("../data/default.pdb");

fn count_residues(atoms: &[Atom]) -> usize {
    atoms
        .iter()
        .map(|atom| (atom.chain.as_str(), atom.residue_num))
        .collect::<std::collections::HashSet<_>>()
        .len()
}

fn apply_structure(
    atoms: Vec<Atom>,
    structure_label: &str,
    atoms_state: &mut Vec<Atom>,
    residue_count: &mut usize,
    structure_source: &mut String,
    status_message: &mut String,
    renderer: &mut MolecularRenderer,
    cam: &mut Camera3D,
    cam_state: &mut CameraState,
    vis_state: &mut VisualizationState,
) -> Result<(), String> {
    if atoms.is_empty() {
        return Err("No ATOM records were found in the provided PDB data.".to_string());
    }

    let (min_bound, max_bound) = calculate_bounding_box(&atoms);
    let initial_radius = calculate_initial_radius(min_bound, max_bound);

    *atoms_state = atoms;
    *residue_count = count_residues(atoms_state);
    *structure_source = structure_label.to_string();
    *status_message = format!(
        "Loaded {} atoms across {} residues",
        atoms_state.len(),
        *residue_count
    );
    *renderer = MolecularRenderer::new(atoms_state);

    cam_state.reset(initial_radius);
    *cam = Camera3D {
        position: vec3(0., 0., initial_radius),
        target: vec3(0., 0., 0.),
        up: vec3(0., 1., 0.),
        ..Default::default()
    };
    vis_state.initial_camera_pos = cam.position;
    vis_state.reset_view();

    Ok(())
}

#[macroquad::main("protdot")]
async fn main() {
    // Load the protein
    #[cfg(not(target_arch = "wasm32"))]
    let (initial_atoms, initial_label) = {
        let args: Vec<String> = env::args().collect();
        let pdb_file = if args.len() > 1 {
            &args[1]
        } else {
            "data/1G2F.pdb" // Default file for native
        };
        (load_pdb(pdb_file), pdb_file.to_string())
    };

    #[cfg(target_arch = "wasm32")]
    let (initial_atoms, initial_label) = (
        load_pdb(DEFAULT_PDB_DATA),
        "Embedded default structure".to_string(),
    );

    // Calculate bounding box to fit entire structure
    let (min_bound, max_bound) = calculate_bounding_box(&initial_atoms);
    let initial_radius = calculate_initial_radius(min_bound, max_bound);

    // Setup Camera
    let mut cam = Camera3D {
        position: vec3(0., 0., initial_radius),
        target: vec3(0., 0., 0.),
        up: vec3(0., 1., 0.),
        ..Default::default()
    };

    // State management
    let mut cam_state = CameraState::new(initial_radius);
    let mut vis_state = VisualizationState::new();

    // Try to load custom color palette (CLI only)
    #[cfg(not(target_arch = "wasm32"))]
    if let Err(e) = vis_state.color_maps.merge_with_file("color_palette.yaml") {
        eprintln!(
            "Note: Could not load color_palette.yaml (using defaults): {}",
            e
        );
    }

    vis_state.initial_camera_pos = cam.position;
    let mut ui_state = UIState::new();
    let mut anim_state = AnimationState::new();
    let mut picker_state = ColorPickerState::new();
    let mut atoms = initial_atoms;
    let mut residue_count = count_residues(&atoms);
    let mut structure_source = initial_label;
    let mut status_message = format!(
        "Loaded {} atoms across {} residues",
        atoms.len(),
        residue_count
    );
    let mut pending_structure_label = structure_source.clone();
    let mut renderer = MolecularRenderer::new(&atoms);

    // Track state to detect changes
    let mut last_color_scheme = vis_state.color_scheme;
    let mut last_radius_scale = vis_state.radius_scale;
    let mut last_mapping_index = vis_state.color_maps.mapping_index;
    let mut last_mapping_rotation_element = vis_state.color_maps.mapping_rotation_element;
    let mut last_mapping_rotation_aa = vis_state.color_maps.mapping_rotation_aa;
    let mut last_theme_index = vis_state.color_maps.current_theme_index;

    loop {
        #[cfg(target_arch = "wasm32")]
        for dropped_file in get_dropped_files() {
            if let Some(bytes) = dropped_file.bytes {
                let structure_label = dropped_file
                    .path
                    .as_ref()
                    .and_then(|path| path.file_name())
                    .and_then(|name| name.to_str())
                    .unwrap_or("Dropped PDB file")
                    .to_string();
                let pdb_text = String::from_utf8_lossy(&bytes).to_string();
                let parsed = pdb_parser::parse_pdb_content(&pdb_text);

                if let Err(message) = apply_structure(
                    parsed,
                    &structure_label,
                    &mut atoms,
                    &mut residue_count,
                    &mut structure_source,
                    &mut status_message,
                    &mut renderer,
                    &mut cam,
                    &mut cam_state,
                    &mut vis_state,
                ) {
                    status_message = message;
                }
            }
        }

        for command in poll_commands() {
            match command {
                WebCommand::LoadPdbText(pdb_text) => {
                    let parsed = pdb_parser::parse_pdb_content(&pdb_text);

                    if let Err(message) = apply_structure(
                        parsed,
                        &pending_structure_label,
                        &mut atoms,
                        &mut residue_count,
                        &mut structure_source,
                        &mut status_message,
                        &mut renderer,
                        &mut cam,
                        &mut cam_state,
                        &mut vis_state,
                    ) {
                        status_message = message;
                    }
                }
                WebCommand::SetStructureLabel(label) => {
                    pending_structure_label = label;
                }
                WebCommand::LoadDefaultStructure => {
                    #[cfg(target_arch = "wasm32")]
                    {
                        pending_structure_label = "Embedded default structure".to_string();
                        if let Err(message) = apply_structure(
                            load_pdb(DEFAULT_PDB_DATA),
                            &pending_structure_label,
                            &mut atoms,
                            &mut residue_count,
                            &mut structure_source,
                            &mut status_message,
                            &mut renderer,
                            &mut cam,
                            &mut cam_state,
                            &mut vis_state,
                        ) {
                            status_message = message;
                        }
                    }
                }
                WebCommand::SetColorScheme(color_scheme) => {
                    vis_state.color_scheme = color_scheme;
                }
                WebCommand::SetRenderMode(render_mode) => {
                    vis_state.render_mode = render_mode;
                }
                WebCommand::NextTheme => {
                    vis_state.color_maps.next_theme();
                    vis_state.bg_color = vis_state.color_maps.current_background();
                }
                WebCommand::PreviousTheme => {
                    vis_state.color_maps.prev_theme();
                    vis_state.bg_color = vis_state.color_maps.current_background();
                }
                WebCommand::SetThemeIndex(index) => {
                    vis_state.color_maps.set_theme_index(index);
                    vis_state.bg_color = vis_state.color_maps.current_background();
                }
                WebCommand::SetRadiusScale(radius_scale) => {
                    vis_state.radius_scale = radius_scale.max(0.1);
                }
                WebCommand::ResetView => {
                    cam_state.reset(vis_state.initial_camera_pos.length().max(5.0));
                    cam.position = vis_state.initial_camera_pos;
                    cam.target = vec3(0., 0., 0.);
                    vis_state.reset_view();
                    anim_state.time = 0.0;
                }
                WebCommand::SetAnimationEnabled(enabled) => {
                    anim_state.enabled = enabled;
                    if enabled && anim_state.mode == AnimationMode::None {
                        anim_state.mode = AnimationMode::RotateY;
                    }
                }
                WebCommand::SetAnimationMode(mode) => {
                    anim_state.mode = mode;
                    anim_state.time = 0.0;
                }
                WebCommand::SetAnimationSpeed(speed) => {
                    anim_state.speed = speed.clamp(0.0, 5.0);
                }
                WebCommand::SetUiVisible(visible) => {
                    ui_state.show_ui = visible;
                }
            }
        }

        // Handle all input
        handle_mode_switches(&mut vis_state);
        handle_radius_scale(&mut vis_state);
        handle_animation_controls(&mut anim_state);

        // Mark renderer dirty if visual parameters changed
        if vis_state.color_scheme != last_color_scheme
            || (vis_state.radius_scale - last_radius_scale).abs() > 0.001
            || vis_state.color_maps.mapping_index != last_mapping_index
            || vis_state.color_maps.mapping_rotation_element != last_mapping_rotation_element
            || vis_state.color_maps.mapping_rotation_aa != last_mapping_rotation_aa
            || vis_state.color_maps.current_theme_index != last_theme_index
        {
            renderer.mark_dirty();
            last_color_scheme = vis_state.color_scheme;
            last_radius_scale = vis_state.radius_scale;
            last_mapping_index = vis_state.color_maps.mapping_index;
            last_mapping_rotation_element = vis_state.color_maps.mapping_rotation_element;
            last_mapping_rotation_aa = vis_state.color_maps.mapping_rotation_aa;
            last_theme_index = vis_state.color_maps.current_theme_index;
        }

        // Must update camera controls first to get camera position
        handle_camera_controls(&mut cam, &mut cam_state);

        // Then handle model rotation which calculates screen-space axes
        handle_model_rotation(&mut vis_state, &cam);
        ui_state.handle_toggles();

        #[cfg(not(target_arch = "wasm32"))]
        let export_format = handle_export();

        // Update animation using screen-space axes and camera angles
        let delta_time = get_frame_time();
        anim_state.update(
            delta_time,
            &mut vis_state.rotation,
            vis_state.camera_right,
            vis_state.camera_up,
            vis_state.camera_forward,
            &mut cam_state.angle_x,
            &mut cam_state.angle_y,
        );

        // Clear and render
        clear_background(vis_state.bg_color);
        set_camera(&cam);

        // Render atoms
        render_atoms_optimized(
            &mut renderer,
            &atoms,
            &cam,
            vis_state.color_scheme,
            vis_state.render_mode,
            &vis_state.color_maps,
            vis_state.radius_scale,
            vis_state.rotation,
            vis_state.translation,
            (
                vis_state.camera_right,
                vis_state.camera_up,
                vis_state.camera_forward,
            ),
        );

        // Switch to 2D for UI, &anim_state
        set_default_camera();

        if ui_state.show_ui {
            let theme_name = vis_state.color_maps.current_theme_name();
            draw_info_overlay(
                &structure_source,
                &status_message,
                atoms.len(),
                residue_count,
                vis_state.color_scheme,
                vis_state.render_mode,
                vis_state.radius_scale,
                &anim_state,
                theme_name,
                vis_state.color_maps.mapping_index,
                vis_state.color_maps.mapping_rotation_element,
                vis_state.color_maps.mapping_rotation_aa,
            );
        }

        if ui_state.show_color_picker {
            draw_color_picker(
                &mut picker_state,
                &mut vis_state.bg_color,
                &mut vis_state.color_maps,
            );
        }

        // Export before frame swap (capture current rendered frame)
        // Only available in native builds
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(format) = export_format {
            match format {
                ExportFormat::PNG => {
                    let image = get_screen_data();
                    image.export_png("protein_export.png");
                    println!("✓ Exported to protein_export.png");
                }
                ExportFormat::SVG => {
                    if let Err(e) = export_svg(
                        &atoms,
                        "protein_export.svg",
                        &cam,
                        vis_state.color_scheme,
                        vis_state.render_mode,
                        &vis_state.color_maps,
                        vis_state.radius_scale,
                        screen_width() as f32,
                        screen_height() as f32,
                        vis_state.rotation,
                        vis_state.translation,
                    ) {
                        eprintln!("Failed to export SVG: {}", e);
                    } else {
                        println!("✓ Exported to protein_export.svg");
                    }
                }
            }
        }

        next_frame().await;
    }
}
