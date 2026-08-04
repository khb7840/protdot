use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationMode {
    None,
    RotateY,  // Smooth rotation around Y axis
    RotateX,  // Rotation around X axis
    RotateZ,  // Rotation around Z axis
    RotateXY, // Combined X and Y rotation
    Orbit,    // Orbital motion
    Figure8,  // Figure-8 pattern
    Wobble,   // Oscillating wobble
    Tumble,   // Random tumbling effect
}

pub struct AnimationState {
    pub mode: AnimationMode,
    pub speed: f32,
    pub time: f32,
    pub enabled: bool,
    pub reverse: bool,
}

impl AnimationState {
    pub fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        let enabled = true;

        #[cfg(not(target_arch = "wasm32"))]
        let enabled = false;

        Self {
            mode: AnimationMode::Wobble,
            speed: 2.0,
            time: 0.0,
            enabled,
            reverse: false,
        }
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        rotation: &mut Quat,
        camera_right: Vec3,
        camera_up: Vec3,
        camera_forward: Vec3,
        angle_x: &mut f32,
        angle_y: &mut f32,
    ) {
        if !self.enabled || self.mode == AnimationMode::None {
            return;
        }

        self.time += delta_time * self.speed;
        let dt = delta_time * self.speed;
        let direction = if self.reverse { -1.0 } else { 1.0 };

        match self.mode {
            AnimationMode::None => {}

            AnimationMode::RotateY => {
                // Rotate around screen vertical axis (up)
                let rot = Quat::from_axis_angle(camera_up, dt * 0.5 * direction);
                *rotation = rot * *rotation;
            }

            AnimationMode::RotateX => {
                // Rotate around screen horizontal axis (right)
                let rot = Quat::from_axis_angle(camera_right, dt * 0.3 * direction);
                *rotation = rot * *rotation;
            }

            AnimationMode::RotateZ => {
                // Rotate around screen depth axis (forward) - clockwise
                let rot = Quat::from_axis_angle(camera_forward, dt * 0.4 * direction);
                *rotation = rot * *rotation;
            }

            AnimationMode::RotateXY => {
                // Combined horizontal and vertical rotation
                let rot_h = Quat::from_axis_angle(camera_right, dt * 0.2 * direction);
                let rot_v = Quat::from_axis_angle(camera_up, dt * 0.4 * direction);
                *rotation = rot_v * rot_h * *rotation;
            }

            AnimationMode::Orbit => {
                // Circular orbit pattern
                *angle_x = (self.time * 0.5).cos() * 3.14;
                *angle_y = (self.time * 0.5).sin() * 0.8;
            }

            AnimationMode::Figure8 => {
                // Figure-8 pattern (Lissajous curve)
                *angle_x = (self.time * 0.5).sin() * 3.14;
                *angle_y = (self.time * 1.0).sin() * 0.8;
            }

            AnimationMode::Wobble => {
                // Oscillating wobble effect
                *angle_x += (self.time * 2.0).sin() * 0.02;
                *angle_y = (self.time * 1.5).cos() * 0.5;
            }

            AnimationMode::Tumble => {
                // Chaotic tumbling with multiple frequencies
                *angle_x += ((self.time * 0.7).sin() + (self.time * 1.3).cos() * 0.5) * 0.01;
                *angle_y += ((self.time * 0.9).cos() + (self.time * 1.7).sin() * 0.5) * 0.008;
                *angle_y = angle_y.clamp(-1.5, 1.5);
            }
        }

        // Normalize quaternion to prevent accumulation of error
        *rotation = rotation.normalize();
    }

    pub fn next_mode(&mut self) {
        self.mode = match self.mode {
            AnimationMode::None => AnimationMode::RotateY,
            AnimationMode::RotateY => AnimationMode::RotateX,
            AnimationMode::RotateX => AnimationMode::RotateZ,
            AnimationMode::RotateZ => AnimationMode::RotateXY,
            AnimationMode::RotateXY => AnimationMode::Orbit,
            AnimationMode::Orbit => AnimationMode::Figure8,
            AnimationMode::Figure8 => AnimationMode::Wobble,
            AnimationMode::Wobble => AnimationMode::Tumble,
            AnimationMode::Tumble => AnimationMode::None,
        };
        self.time = 0.0; // Reset time when changing modes
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
        if self.enabled && self.mode == AnimationMode::None {
            self.mode = AnimationMode::RotateY;
        }
    }

    pub fn increase_speed(&mut self) {
        self.speed = (self.speed + 0.2).min(5.0);
    }

    pub fn decrease_speed(&mut self) {
        self.speed = (self.speed - 0.2).max(0.0);
    }

    pub fn toggle_reverse(&mut self) {
        self.reverse = !self.reverse;
    }

    pub fn mode_name(&self) -> &str {
        match self.mode {
            AnimationMode::None => "None",
            AnimationMode::RotateY => "Rotate Y",
            AnimationMode::RotateX => "Rotate X",
            AnimationMode::RotateZ => "Rotate Z",
            AnimationMode::RotateXY => "Rotate XY",
            AnimationMode::Orbit => "Orbit",
            AnimationMode::Figure8 => "Figure-8",
            AnimationMode::Wobble => "Wobble",
            AnimationMode::Tumble => "Tumble",
        }
    }
}
