use macroquad::prelude::*;
use crate::color_maps::ColorMaps;
use crate::types::{ColorScheme, RenderMode};

pub struct VisualizationState {
    pub color_scheme: ColorScheme,
    pub render_mode: RenderMode,
    pub radius_scale: f32,
    pub bg_color: Color,
    pub color_maps: ColorMaps,
    pub rotation: Quat,
    pub rotation_speed: f32,
    pub translation: Vec3,
    pub translation_speed: f32,
    pub camera_right: Vec3,
    pub camera_up: Vec3,
    pub camera_forward: Vec3,
    pub initial_camera_pos: Vec3,
}

impl VisualizationState {
    pub fn default_radius_scale() -> f32 {
        #[cfg(target_arch = "wasm32")]
        let radius_scale = 0.3;
        
        #[cfg(not(target_arch = "wasm32"))]
        let radius_scale = 1.0;

        radius_scale
    }

    pub fn default_render_mode() -> RenderMode {
        #[cfg(target_arch = "wasm32")]
        let render_mode = RenderMode::PerResidue;
        
        #[cfg(not(target_arch = "wasm32"))]
        let render_mode = RenderMode::PerAtom;

        render_mode
    }

    pub fn new() -> Self {
        let radius_scale = Self::default_radius_scale();
        let render_mode = Self::default_render_mode();
        
        let color_maps = ColorMaps::new();
        let bg_color = color_maps.current_background();
        
        Self {
            color_scheme: ColorScheme::ByElement,
            render_mode,
            radius_scale,
            bg_color,
            color_maps,
            rotation: Quat::IDENTITY,
            rotation_speed: 0.02,
            translation: vec3(0.0, 0.0, 0.0),
            translation_speed: 0.5,
            camera_right: vec3(1.0, 0.0, 0.0),
            camera_up: vec3(0.0, 1.0, 0.0),
            camera_forward: vec3(0.0, 0.0, -1.0),
            initial_camera_pos: vec3(0.0, 0.0, 0.0),
        }
    }
    
    pub fn reset(&mut self) {
        let radius_scale = Self::default_radius_scale();
        let render_mode = Self::default_render_mode();
        
        self.color_scheme = ColorScheme::ByElement;
        self.render_mode = render_mode;
        self.radius_scale = radius_scale;
        self.rotation = Quat::IDENTITY;
        self.translation = vec3(0.0, 0.0, 0.0);
        // Keep color_maps as they might have custom palettes loaded
    }
}
