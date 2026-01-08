use macroquad::prelude::*;
use crate::types::{ColorScheme, RenderMode};
use crate::camera_state::CameraState;
use crate::visualization_state::VisualizationState;
use crate::animation::AnimationState;

pub fn handle_mode_switches(vis: &mut VisualizationState) {
    if is_key_pressed(KeyCode::C) {
        vis.color_scheme = match vis.color_scheme {
            ColorScheme::ByElement => ColorScheme::ByAminoAcidGroup,
            ColorScheme::ByAminoAcidGroup => ColorScheme::ByAminoAcidType,
            ColorScheme::ByAminoAcidType => ColorScheme::NToCGradient,
            ColorScheme::NToCGradient => ColorScheme::RandomChain,
            ColorScheme::RandomChain => ColorScheme::ByElement,
        };
    }
    
    if is_key_pressed(KeyCode::B) {
        vis.render_mode = match vis.render_mode {
            RenderMode::PerAtom => RenderMode::PerResidue,
            RenderMode::PerResidue => RenderMode::PerAtom,
        };
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

pub fn handle_alpha(vis: &mut VisualizationState) {
    if is_key_pressed(KeyCode::LeftBracket) {
        vis.alpha = (vis.alpha - 0.1).max(0.0);
    }
    if is_key_pressed(KeyCode::RightBracket) {
        vis.alpha = (vis.alpha + 0.1).min(1.0);
    }
}

pub fn handle_export() -> bool {
    is_key_pressed(KeyCode::E)
}

pub fn handle_animation_controls(anim: &mut AnimationState) {
    // A key: Toggle animation on/off
    if is_key_pressed(KeyCode::A) {
        anim.toggle();
    }
    
    // N key: Next animation mode
    if is_key_pressed(KeyCode::N) {
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
