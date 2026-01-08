use macroquad::prelude::*;
use crate::types::{ColorScheme, RenderMode};
use crate::animation::AnimationState;

pub fn draw_info_overlay(
    atom_count: usize, 
    color_scheme: ColorScheme,
    render_mode: RenderMode,
    radius_scale: f32,
    alpha: f32,
    anim_state: &AnimationState,
) {
    draw_text(&format!("Atoms: {}", atom_count), 10.0, 20.0, 30.0, BLACK);
    draw_text("Left drag: rotate", 10.0, 50.0, 20.0, DARKGRAY);
    draw_text("Middle drag: pan", 10.0, 70.0, 20.0, DARKGRAY);
    draw_text("Scroll: zoom", 10.0, 90.0, 20.0, DARKGRAY);
    
    let color_text = match color_scheme {
        ColorScheme::ByElement => "C: By Element",
        ColorScheme::ByAminoAcidGroup => "C: By AA Group",
        ColorScheme::ByAminoAcidType => "C: By AA Type",
        ColorScheme::NToCGradient => "C: N->C Gradient",
        ColorScheme::RandomChain => "C: Random Chain",
    };
    let render_text = match render_mode {
        RenderMode::PerAtom => "B: Per Atom",
        RenderMode::PerResidue => "B: Per Residue",
    };
    draw_text(color_text, 10.0, 120.0, 20.0, DARKGREEN);
    draw_text(render_text, 10.0, 140.0, 20.0, DARKGREEN);
    draw_text(&format!("+ / - / 0: Radius scale {:.2}x", radius_scale), 10.0, 160.0, 20.0, DARKGREEN);
    draw_text(&format!("[ / ]: Alpha {:.1}", alpha), 10.0, 180.0, 20.0, DARKGREEN);
    
    // Animation info
    let anim_status = if anim_state.enabled { "ON" } else { "OFF" };
    draw_text(&format!("A: Anim {} ({})", anim_state.mode_name(), anim_status), 10.0, 200.0, 20.0, DARKGREEN);
    if anim_state.enabled {
        draw_text(&format!("N: Next | ,/.: Speed {:.1}x", anim_state.speed), 10.0, 220.0, 20.0, DARKGREEN);
        draw_text("H: Hide UI | P: Color Picker", 10.0, 240.0, 20.0, DARKGREEN);
        draw_text("E: Export PNG", 10.0, 260.0, 20.0, DARKGREEN);
    } else {
        draw_text("H: Hide UI | P: Color Picker", 10.0, 220.0, 20.0, DARKGREEN);
        draw_text("E: Export PNG", 10.0, 240.0, 20.0, DARKGREEN);
    }
}
