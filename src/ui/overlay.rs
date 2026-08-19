use crate::animation::AnimationState;
use crate::types::{ColorScheme, RenderMode};
use macroquad::prelude::*;

pub fn draw_info_overlay(
    structure_label: &str,
    status_message: &str,
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
    draw_text(
        &format!("Source: {}", structure_label),
        10.0,
        20.0,
        22.0,
        BLACK,
    );
    if !status_message.is_empty() {
        draw_text(status_message, 10.0, 42.0, 18.0, DARKGRAY);
    }
    draw_text(
        &format!(
            "Atoms: {} | Residues: {} | Theme: {} (Map: {} E:{} A:{}) | FPS: {}",
            atom_count,
            residue_count,
            theme_name,
            mapping_index,
            mapping_rotation_element,
            mapping_rotation_aa,
            fps
        ),
        10.0,
        68.0,
        25.0,
        BLACK,
    );
    draw_text(
        "WS: X-Axis | AD: Y-Axis | QE: Z-Axis",
        10.0,
        98.0,
        20.0,
        DARKGRAY,
    );
    draw_text(
        "Arrows: Move | Mouse drag: camera | R: Reset",
        10.0,
        118.0,
        20.0,
        DARKGRAY,
    );
    draw_text(
        "Scroll: zoom | Shift+(): Rot speed | Drop PDB on page",
        10.0,
        138.0,
        20.0,
        DARKGRAY,
    );

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
    draw_text(&color_text, 10.0, 168.0, 20.0, GRAY);
    draw_text(render_text, 10.0, 188.0, 20.0, GRAY);
    draw_text(
        &format!("+ / - / 0: Radius scale {:.2}x", radius_scale),
        10.0,
        208.0,
        20.0,
        GRAY,
    );

    // Animation info
    let anim_status = if anim_state.enabled { "ON" } else { "OFF" };
    let reverse_indicator = if anim_state.reverse { " REV" } else { "" };
    draw_text(
        &format!(
            "T: Anim {} ({}{})",
            anim_state.mode_name(),
            anim_status,
            reverse_indicator
        ),
        10.0,
        248.0,
        20.0,
        GRAY,
    );
    if anim_state.enabled {
        draw_text(
            &format!("Y: Next | ,/.: Speed {:.1}x | U: Reverse", anim_state.speed),
            10.0,
            268.0,
            20.0,
            GRAY,
        );
        draw_text("H: Hide UI | P: Color Picker", 10.0, 288.0, 20.0, GRAY);
        #[cfg(not(target_arch = "wasm32"))]
        draw_text("X: PNG | V: SVG", 10.0, 308.0, 20.0, GRAY);
        #[cfg(target_arch = "wasm32")]
        draw_text("M: Theme", 10.0, 308.0, 20.0, GRAY);
    } else {
        draw_text("H: Hide UI | P: Color Picker", 10.0, 268.0, 20.0, GRAY);
        #[cfg(not(target_arch = "wasm32"))]
        draw_text(
            "X: PNG | V: SVG | M: Theme | N: Mapping",
            10.0,
            288.0,
            20.0,
            GRAY,
        );
        #[cfg(target_arch = "wasm32")]
        draw_text("M: Theme | N: Mapping", 10.0, 288.0, 20.0, GRAY);
    }
}
