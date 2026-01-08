use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationMode {
    None,
    RotateY,        // Smooth rotation around Y axis
    RotateX,        // Rotation around X axis
    RotateZ,        // Rotation around Z axis
    RotateXY,       // Combined X and Y rotation
    Orbit,          // Orbital motion
    Figure8,        // Figure-8 pattern
    Wobble,         // Oscillating wobble
    Tumble,         // Random tumbling effect
}

pub struct AnimationState {
    pub mode: AnimationMode,
    pub speed: f32,
    pub time: f32,
    pub enabled: bool,
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
        }
    }

    pub fn update(&mut self, delta_time: f32, angle_x: &mut f32, angle_y: &mut f32) {
        if !self.enabled || self.mode == AnimationMode::None {
            return;
        }

        self.time += delta_time * self.speed;

        match self.mode {
            AnimationMode::None => {},
            
            AnimationMode::RotateY => {
                *angle_x += delta_time * self.speed * 0.5;
            },
            
            AnimationMode::RotateX => {
                *angle_y += delta_time * self.speed * 0.3;
                *angle_y = angle_y.clamp(-1.5, 1.5);
            },
            
            AnimationMode::RotateZ => {
                // Simulate Z rotation by combining X and Y
                let base_x = *angle_x;
                let base_y = *angle_y;
                *angle_x = base_x + (self.time * 0.5).cos() * 0.02;
                *angle_y = base_y + (self.time * 0.5).sin() * 0.02;
            },
            
            AnimationMode::RotateXY => {
                *angle_x += delta_time * self.speed * 0.4;
                *angle_y += delta_time * self.speed * 0.2;
                *angle_y = angle_y.clamp(-1.5, 1.5);
            },
            
            AnimationMode::Orbit => {
                // Circular orbit pattern
                *angle_x = (self.time * 0.5).cos() * 3.14;
                *angle_y = (self.time * 0.5).sin() * 0.8;
            },
            
            AnimationMode::Figure8 => {
                // Figure-8 pattern (Lissajous curve)
                *angle_x = (self.time * 0.5).sin() * 3.14;
                *angle_y = (self.time * 1.0).sin() * 0.8;
            },
            
            AnimationMode::Wobble => {
                // Oscillating wobble effect
                *angle_x += (self.time * 2.0).sin() * 0.02;
                *angle_y = (self.time * 1.5).cos() * 0.5;
            },
            
            AnimationMode::Tumble => {
                // Chaotic tumbling with multiple frequencies
                *angle_x += ((self.time * 0.7).sin() + (self.time * 1.3).cos() * 0.5) * 0.01;
                *angle_y += ((self.time * 0.9).cos() + (self.time * 1.7).sin() * 0.5) * 0.008;
                *angle_y = angle_y.clamp(-1.5, 1.5);
            },
        }
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
        self.speed = (self.speed - 0.2).max(0.1);
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
