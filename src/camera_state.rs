use macroquad::prelude::*;

pub struct CameraState {
    pub angle_x: f32,
    pub angle_y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub radius: f32,
    pub pan_offset: Vec3,
    pub prev_mouse_pos: (f32, f32),
    pub mouse_was_down: bool,
}

impl CameraState {
    pub fn new(initial_radius: f32) -> Self {
        Self {
            angle_x: 0.0,
            angle_y: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            radius: initial_radius,
            pan_offset: vec3(0.0, 0.0, 0.0),
            prev_mouse_pos: (0.0, 0.0),
            mouse_was_down: false,
        }
    }
}
