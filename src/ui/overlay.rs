use macroquad::prelude::*;
use crate::types::{ColorScheme, RenderMode};
use crate::animation::AnimationState;

pub fn draw_info_overlay(
    atom_count: usize, 
    residue_count: usize,
    color_scheme: ColorScheme,
    render_mode: RenderMode,
    radius_scale: f32,
    anim_state: &AnimationState,
    theme_name: &str,
    mapping_index: usize,
    mapping_rotation_element: usize,
    mapping_rotation_aa: usize,
) {
    let fps = get_fps();
    draw_text(&format!("Atoms: {} | Residues: {} | Theme: {} (Map: {} E:{} A:{}) | FPS: {}", atom_count, residue_count, theme_name, mapping_index, mapping_rotation_element, mapping_rotation_aa, fps), 10.0, 20.0, 25.0, BLACK);
    draw_text("WS: X-Axis | AD: Y-Axis | QE: Z-Axis", 10.0, 50.0, 20.0, DARKGRAY);
    draw_text("Arrows: Move | Mouse drag: camera | R: Reset", 10.0, 70.0, 20.0, DARKGRAY);
    draw_text("Scroll: zoom | Shift+(): Rot speed", 10.0, 90.0, 20.0, DARKGRAY);
    
    let color_text = match color_scheme {
        ColorScheme::ByElement => "C: By Element".to_string(),
        ColorScheme::ByAminoAcidGroup => "C: By AA Group".to_string(),
        ColorScheme::ByAminoAcidType => "C: By AA Type".to_string(),
        ColorScheme::NToCGradient => "C: N->C Gradient".to_string(),
        ColorScheme::RandomChain => "C: Random Chain".to_string(),
        ColorScheme::Theme => format!("C: Theme ({})", theme_name),
    };
    let render_text = match render_mode {
        RenderMode::PerAtom => "B: Per Atom",
        RenderMode::PerResidue => "B: Per Residue",
    };
    draw_text(&color_text, 10.0, 120.0, 20.0, GRAY);
    draw_text(render_text, 10.0, 140.0, 20.0, GRAY);
    draw_text(&format!("+ / - / 0: Radius scale {:.2}x", radius_scale), 10.0, 160.0, 20.0, GRAY);
    
    // Animation info
    let anim_status = if anim_state.enabled { "ON" } else { "OFF" };
    let reverse_indicator = if anim_state.reverse { " REV" } else { "" };
    draw_text(&format!("T: Anim {} ({}{})", anim_state.mode_name(), anim_status, reverse_indicator), 10.0, 200.0, 20.0, GRAY);
    if anim_state.enabled {
        draw_text(&format!("Y: Next | ,/.: Speed {:.1}x | U: Reverse", anim_state.speed), 10.0, 220.0, 20.0, GRAY);
        draw_text("H: Hide UI | P: Color Picker", 10.0, 240.0, 20.0, GRAY);
        #[cfg(not(target_arch = "wasm32"))]
        draw_text("X: PNG | V: SVG", 10.0, 260.0, 20.0, GRAY);
        #[cfg(target_arch = "wasm32")]
        draw_text("M: Theme", 10.0, 260.0, 20.0, GRAY);
    } else {
        draw_text("H: Hide UI | P: Color Picker", 10.0, 220.0, 20.0, GRAY);
        #[cfg(not(target_arch = "wasm32"))]
        draw_text("X: PNG | V: SVG | M: Theme | N: Mapping", 10.0, 240.0, 20.0, GRAY);
        #[cfg(target_arch = "wasm32")]
        draw_text("M: Theme | N: Mapping", 10.0, 240.0, 20.0, GRAY);
    }
}
