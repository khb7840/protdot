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

// Use declarations
use pdb_parser::load_pdb;
use camera::{calculate_bounding_box, calculate_initial_radius};
use ui::{UIState, ColorPickerState, draw_info_overlay, draw_color_picker, render_atoms};
use input::{handle_mode_switches, handle_radius_scale, handle_alpha, handle_export, handle_camera_controls, handle_animation_controls};
use camera_state::CameraState;
use visualization_state::VisualizationState;
use animation::AnimationState;

// For native builds, use command line args
#[cfg(not(target_arch = "wasm32"))]
use std::env;

// For WASM builds, embed default PDB data
#[cfg(target_arch = "wasm32")]
const DEFAULT_PDB_DATA: &str = include_str!("../data/1G2F.pdb");

#[macroquad::main("Protein Viewer")]
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
    let mut ui_state = UIState::new();
    let mut anim_state = AnimationState::new();
    let mut picker_state = ColorPickerState::new();

    loop {
        // Handle all input
        handle_mode_switches(&mut vis_state);
        handle_radius_scale(&mut vis_state);
        handle_alpha(&mut vis_state);
        handle_animation_controls(&mut anim_state);
        ui_state.handle_toggles();
        let should_export = handle_export();
        
        handle_camera_controls(&mut cam, &mut cam_state);
        
        // Update animation
        let delta_time = get_frame_time();
        anim_state.update(delta_time, &mut cam_state.angle_x, &mut cam_state.angle_y);

        // Clear and render
        clear_background(vis_state.bg_color);
        set_camera(&cam);

        // Render atoms
        render_atoms(&atoms, &cam, vis_state.color_scheme, vis_state.render_mode, 
                    &vis_state.color_maps, vis_state.radius_scale, vis_state.alpha);

        // Switch to 2D for UI, &anim_state
        set_default_camera();
        
        if ui_state.show_ui {
            draw_info_overlay(atoms.len(), vis_state.color_scheme, vis_state.render_mode, 
                            vis_state.radius_scale, vis_state.alpha, &anim_state);
        }
        
        if ui_state.show_color_picker {
            draw_color_picker(&mut picker_state, &mut vis_state.bg_color, &mut vis_state.color_maps);
        }


        // Export PNG before frame swap (capture current rendered frame)
        if should_export {
            let image = get_screen_data();
            image.export_png("protein_export.png");
            println!("Exported to protein_export.png");
        }

        next_frame().await;
    }
}