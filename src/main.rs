use macroquad::prelude::*;

// Module declarations
mod atom;
mod color_maps;
mod types;
mod pdb_parser;
mod camera;
mod ui;
mod input;
mod camera_state;
mod visualization_state;
mod animation;
mod render_cache;

#[cfg(not(target_arch = "wasm32"))]
mod export;

// Use declarations
use pdb_parser::load_pdb;
use camera::{calculate_bounding_box, calculate_initial_radius};
use ui::{UIState, ColorPickerState, draw_info_overlay, draw_color_picker, render_atoms_optimized};
use input::{handle_mode_switches, handle_radius_scale, handle_camera_controls, handle_animation_controls, handle_model_rotation};
use camera_state::CameraState;
use visualization_state::VisualizationState;
use animation::AnimationState;
use render_cache::MolecularRenderer;

#[cfg(not(target_arch = "wasm32"))]
use input::handle_export;

#[cfg(not(target_arch = "wasm32"))]
use export::{ExportFormat, export_svg};

// For native builds, use command line args
#[cfg(not(target_arch = "wasm32"))]
use std::env;

// For WASM builds, embed default PDB data
#[cfg(target_arch = "wasm32")]
const DEFAULT_PDB_DATA: &str = include_str!("../data/1G2F.pdb");

#[macroquad::main("protdot")]
async fn main() {
    // Load the protein
    #[cfg(not(target_arch = "wasm32"))]
    let atoms = {
        let args: Vec<String> = env::args().collect();
        let pdb_file = if args.len() > 1 {
            &args[1]
        } else {
            "data/1G2F.pdb" // Default file for native
        };
        load_pdb(pdb_file)
    };
    
    #[cfg(target_arch = "wasm32")]
    let atoms = load_pdb(DEFAULT_PDB_DATA);
    
    // Calculate bounding box to fit entire structure
    let (min_bound, max_bound) = calculate_bounding_box(&atoms);
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
        eprintln!("Note: Could not load color_palette.yaml (using defaults): {}", e);
    }
    
    vis_state.initial_camera_pos = cam.position;
    let mut ui_state = UIState::new();
    let mut anim_state = AnimationState::new();
    let mut picker_state = ColorPickerState::new();
    let mut renderer = MolecularRenderer::new(&atoms);
    
    // Track state to detect changes
    let mut last_color_scheme = vis_state.color_scheme;
    let mut last_radius_scale = vis_state.radius_scale;
    let mut last_mapping_index = vis_state.color_maps.mapping_index;
    let mut last_mapping_rotation_element = vis_state.color_maps.mapping_rotation_element;
    let mut last_mapping_rotation_aa = vis_state.color_maps.mapping_rotation_aa;
    let mut last_theme_index = vis_state.color_maps.current_theme_index;

    loop {
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
           || vis_state.color_maps.current_theme_index != last_theme_index {
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
        anim_state.update(delta_time, &mut vis_state.rotation,
                         vis_state.camera_right, vis_state.camera_up, vis_state.camera_forward,
                         &mut cam_state.angle_x, &mut cam_state.angle_y);

        // Clear and render
        clear_background(vis_state.bg_color);
        set_camera(&cam);

        // Render atoms
        render_atoms_optimized(&mut renderer, &atoms, &cam, vis_state.color_scheme, vis_state.render_mode, 
                    &vis_state.color_maps, vis_state.radius_scale,
                    vis_state.rotation,
                    vis_state.translation,
                    (vis_state.camera_right, vis_state.camera_up, vis_state.camera_forward));

        // Switch to 2D for UI, &anim_state
        set_default_camera();
        
        if ui_state.show_ui {
            let theme_name = vis_state.color_maps.current_theme_name();
            let residue_count = atoms.iter()
                .map(|a| a.residue_num)
                .collect::<std::collections::HashSet<_>>()
                .len();
            draw_info_overlay(atoms.len(), residue_count, vis_state.color_scheme, vis_state.render_mode, 
                            vis_state.radius_scale, &anim_state, theme_name, vis_state.color_maps.mapping_index,
                            vis_state.color_maps.mapping_rotation_element, vis_state.color_maps.mapping_rotation_aa);
        }
        
        if ui_state.show_color_picker {
            draw_color_picker(&mut picker_state, &mut vis_state.bg_color, &mut vis_state.color_maps);
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