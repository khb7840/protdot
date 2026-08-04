use macroquad::prelude::*;
use crate::types::{ColorScheme, RenderMode};
use crate::camera_state::CameraState;
use crate::visualization_state::VisualizationState;
use crate::animation::AnimationState;
use crate::visualization_presets::apply_visualization_preset;

pub fn handle_model_rotation(vis: &mut VisualizationState, cam: &Camera3D) {
    // Calculate screen-space axes based on current camera view
    // These axes are recalculated each frame to always match the screen orientation
    let view_dir = (cam.target - cam.position).normalize();
    let right = view_dir.cross(vec3(0.0, 1.0, 0.0)).normalize();
    let up = right.cross(view_dir).normalize();
    
    // Store for rendering
    vis.camera_right = right;
    vis.camera_up = up;
    vis.camera_forward = view_dir;
    
    // W/S: Rotate around camera's horizontal axis (screen X - up/down)
    if is_key_down(KeyCode::W) {
        let rot = Quat::from_axis_angle(right, -vis.rotation_speed);
        vis.rotation = rot * vis.rotation;
    }
    if is_key_down(KeyCode::S) {
        let rot = Quat::from_axis_angle(right, vis.rotation_speed);
        vis.rotation = rot * vis.rotation;
    }
    
    // A/D: Rotate around camera's vertical axis (screen Y - left/right)
    if is_key_down(KeyCode::A) {
        let rot = Quat::from_axis_angle(up, -vis.rotation_speed);
        vis.rotation = rot * vis.rotation;
    }
    if is_key_down(KeyCode::D) {
        let rot = Quat::from_axis_angle(up, vis.rotation_speed);
        vis.rotation = rot * vis.rotation;
    }
    
    // Q/E: Rotate around camera's view axis (screen Z - roll clockwise/counterclockwise)
    if is_key_down(KeyCode::Q) {
        let rot = Quat::from_axis_angle(view_dir, -vis.rotation_speed);
        vis.rotation = rot * vis.rotation;
    }
    if is_key_down(KeyCode::E) {
        let rot = Quat::from_axis_angle(view_dir, vis.rotation_speed);
        vis.rotation = rot * vis.rotation;
    }
    
    // Normalize quaternion to prevent accumulation of error
    vis.rotation = vis.rotation.normalize();
    
    // Arrow keys: Translation in screen space
    if is_key_down(KeyCode::Up) {
        vis.translation += up * vis.translation_speed;
    }
    if is_key_down(KeyCode::Down) {
        vis.translation -= up * vis.translation_speed;
    }
    if is_key_down(KeyCode::Left) {
        vis.translation -= right * vis.translation_speed;
    }
    if is_key_down(KeyCode::Right) {
        vis.translation += right * vis.translation_speed;
    }
    
    // Rotation speed adjustment with ( and ) keys (Shift+9 and Shift+0)
    if is_key_pressed(KeyCode::Key9) && is_key_down(KeyCode::LeftShift) {  // (
        vis.rotation_speed = (vis.rotation_speed - 0.005).max(0.005);
    }
    if is_key_pressed(KeyCode::Key0) && is_key_down(KeyCode::LeftShift) {  // )
        vis.rotation_speed = (vis.rotation_speed + 0.005).min(0.1);
    }
}

pub fn handle_mode_switches(vis: &mut VisualizationState) {
    // R key: Reset to defaults
    if is_key_pressed(KeyCode::R) {
        vis.reset();
    }
    
    if is_key_pressed(KeyCode::C) {
        vis.color_scheme = match vis.color_scheme {
            ColorScheme::ByElement => ColorScheme::ByAminoAcidGroup,
            ColorScheme::ByAminoAcidGroup => ColorScheme::ByAminoAcidType,
            ColorScheme::ByAminoAcidType => ColorScheme::NToCGradient,
            ColorScheme::NToCGradient => ColorScheme::RandomChain,
            ColorScheme::RandomChain => ColorScheme::Theme,
            ColorScheme::Theme => ColorScheme::ByElement,
        };
    }
    
    if is_key_pressed(KeyCode::B) {
        vis.render_mode = match vis.render_mode {
            RenderMode::PerAtom => RenderMode::PerResidue,
            RenderMode::PerResidue => RenderMode::PerAtom,
        };
    }
    
    // M key: Cycle through themes
    if is_key_pressed(KeyCode::M) {
        if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
            // Shift+M: Previous theme
            vis.color_maps.prev_theme();
        } else {
            // M: Next theme
            vis.color_maps.next_theme();
        }
        // Update background color to match the new theme
        vis.bg_color = vis.color_maps.current_background();
    }
    
    // N key: Cycle through color mappings within current theme
    if is_key_pressed(KeyCode::N) {
        if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
            // Shift+N: Previous mapping
            vis.color_maps.previous_mapping_smart(vis.color_scheme);
        } else {
            // N: Next mapping
            vis.color_maps.next_mapping_smart(vis.color_scheme);
        }
    }
}

pub fn handle_visualization_presets(
    vis: &mut VisualizationState,
    anim: &mut AnimationState,
    color_picker_open: bool,
) {
    if color_picker_open {
        return;
    }

    let preset_index = if is_key_pressed(KeyCode::Key1) {
        Some(0)
    } else if is_key_pressed(KeyCode::Key2) {
        Some(1)
    } else if is_key_pressed(KeyCode::Key3) {
        Some(2)
    } else if is_key_pressed(KeyCode::Key4) {
        Some(3)
    } else if is_key_pressed(KeyCode::Key5) {
        Some(4)
    } else if is_key_pressed(KeyCode::Key6) {
        Some(5)
    } else {
        None
    };

    if let Some(preset_index) = preset_index {
        apply_visualization_preset(preset_index, vis, anim);
    }
}

pub fn handle_radius_scale(vis: &mut VisualizationState) {
    if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::KpAdd) {
        vis.radius_scale += 0.1;
    }
    if is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::KpSubtract) {
        vis.radius_scale = (vis.radius_scale - 0.1).max(0.1);
    }
    if is_key_pressed(KeyCode::Key0) {
        vis.radius_scale = 1.0;
    }
}



#[cfg(not(target_arch = "wasm32"))]
use crate::export::ExportFormat;

#[cfg(not(target_arch = "wasm32"))]
pub fn handle_export() -> Option<ExportFormat> {
    if is_key_pressed(KeyCode::V) {
        Some(ExportFormat::SVG)
    } else if is_key_pressed(KeyCode::X) {
        Some(ExportFormat::PNG)
    } else {
        None
    }
}

pub fn handle_animation_controls(anim: &mut AnimationState) {
    // T key: Toggle animation on/off
    if is_key_pressed(KeyCode::T) {
        anim.toggle();
    }
    
    // Y key: Next animation mode
    if is_key_pressed(KeyCode::Y) {
        anim.next_mode();
        if !anim.enabled {
            anim.enabled = true;
        }
    }
    
    // , key: Decrease animation speed
    if is_key_pressed(KeyCode::Comma) {
        anim.decrease_speed();
    }
    
    // . key: Increase animation speed
    if is_key_pressed(KeyCode::Period) {
        anim.increase_speed();
    }
    
    // U key: Toggle reverse direction
    if is_key_pressed(KeyCode::U) {
        anim.toggle_reverse();
    }
}

pub fn handle_camera_controls(cam: &mut Camera3D, cam_state: &mut CameraState) {
    let (mouse_x, mouse_y) = mouse_position();
    let mouse_down = is_mouse_button_down(MouseButton::Left);
    let middle_mouse_down = is_mouse_button_down(MouseButton::Middle);
    
    // Rotation with left button
    if mouse_down {
        if cam_state.mouse_was_down {
            let dx = mouse_x - cam_state.prev_mouse_pos.0;
            let dy = mouse_y - cam_state.prev_mouse_pos.1;
            cam_state.velocity_x = dx * 0.01;
            cam_state.velocity_y = dy * 0.01;
        } else {
            cam_state.prev_mouse_pos = (mouse_x, mouse_y);
        }
        cam_state.angle_x += cam_state.velocity_x;
        cam_state.angle_y += cam_state.velocity_y;
    } else {
        cam_state.velocity_x *= 0.95;
        cam_state.velocity_y *= 0.95;
        cam_state.angle_x += cam_state.velocity_x;
        cam_state.angle_y += cam_state.velocity_y;
    }
    
    // Panning with middle button
    if middle_mouse_down {
        if cam_state.mouse_was_down {
            let dx = (mouse_x - cam_state.prev_mouse_pos.0) * 0.05;
            let dy = (mouse_y - cam_state.prev_mouse_pos.1) * 0.05;
            
            let right = vec3(cam_state.angle_x.cos(), 0.0, -cam_state.angle_x.sin());
            let up = vec3(0.0, 1.0, 0.0);
            
            cam_state.pan_offset += right * dx - up * dy;
        }
    }
    
    cam_state.prev_mouse_pos = (mouse_x, mouse_y);
    cam_state.mouse_was_down = mouse_down || middle_mouse_down;
    
    // Zoom
    let wheel = mouse_wheel().1;
    if wheel != 0.0 {
        cam_state.radius -= wheel * 2.0;
        cam_state.radius = cam_state.radius.max(5.0);
    }
    
    // Update camera position
    cam_state.angle_y = cam_state.angle_y.clamp(-1.5, 1.5);
    
    let x = cam_state.radius * cam_state.angle_y.cos() * cam_state.angle_x.sin();
    let y = cam_state.radius * cam_state.angle_y.sin();
    let z = cam_state.radius * cam_state.angle_y.cos() * cam_state.angle_x.cos();
    
    cam.position = vec3(x, y, z) + cam_state.pan_offset;
    cam.target = cam_state.pan_offset;
}
