use macroquad::prelude::*;
use crate::color_maps::ColorMaps;
use crate::types::{ColorScheme, RenderMode};

pub struct VisualizationState {
    pub color_scheme: ColorScheme,
    pub render_mode: RenderMode,
    pub radius_scale: f32,
    pub alpha: f32,
    pub bg_color: Color,
    pub color_maps: ColorMaps,
}

impl VisualizationState {
    pub fn new() -> Self {
        Self {
            color_scheme: ColorScheme::ByElement,
            render_mode: RenderMode::PerResidue,
            radius_scale: 0.3,
            alpha: 1.0,
            bg_color: LIGHTGRAY,
            color_maps: ColorMaps::new(),
        }
    }
}
