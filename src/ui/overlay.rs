use macroquad::prelude::*;

use crate::{
    animation::AnimationState,
    structure_summary::StructureSummary,
    visualization_presets::{VISUALIZATION_PRESETS, active_visualization_preset},
    visualization_state::VisualizationState,
};

pub fn draw_info_overlay(
    structure_summary: &StructureSummary,
    vis_state: &VisualizationState,
    anim_state: &AnimationState,
) {
    let margin = 14.0;
    let column_gap = 14.0;
    let total_width = (screen_width() - margin * 3.0).max(580.0);
    let left_width = (total_width * 0.56).clamp(320.0, 560.0);
    let right_width = (total_width - left_width).max(240.0);
    let left_x = margin;
    let right_x = left_x + left_width + column_gap;
    let mut left_y = margin;

    draw_summary_panel(left_x, left_y, left_width, structure_summary);
    left_y += 118.0;

    draw_current_view_panel(
        left_x,
        left_y,
        left_width,
        structure_summary,
        vis_state,
        anim_state,
    );
    left_y += 170.0;

    draw_composition_panel(left_x, left_y, left_width, structure_summary);

    draw_preset_panel(right_x, margin, right_width, vis_state, anim_state);
    draw_footer_hint(left_x, screen_height() - 18.0);
}

fn draw_summary_panel(x: f32, y: f32, width: f32, summary: &StructureSummary) {
    draw_panel(
        x,
        y,
        width,
        104.0,
        "Scientific summary",
        "Instant structure context",
    );

    draw_metric(x + 18.0, y + 56.0, "Atoms", summary.atom_count);
    draw_metric(x + 150.0, y + 56.0, "Residues", summary.residue_count);
    draw_metric(x + 302.0, y + 56.0, "Chains", summary.chain_summaries.len());

    let residue_span = if summary.min_residue_num == summary.max_residue_num {
        format!("Residue span: {}", summary.min_residue_num)
    } else {
        format!(
            "Residue span: {} → {}",
            summary.min_residue_num, summary.max_residue_num
        )
    };
    draw_text(&residue_span, x + 18.0, y + 88.0, 20.0, GRAY);
}

fn draw_current_view_panel(
    x: f32,
    y: f32,
    width: f32,
    summary: &StructureSummary,
    vis_state: &VisualizationState,
    anim_state: &AnimationState,
) {
    draw_panel(
        x,
        y,
        width,
        156.0,
        "Current visualization",
        "Useful for screenshots and fast scientific review",
    );

    let animation_label = if anim_state.enabled {
        if anim_state.reverse {
            format!(
                "{} · {:.1}x · reverse",
                anim_state.mode_name(),
                anim_state.speed
            )
        } else {
            format!("{} · {:.1}x", anim_state.mode_name(), anim_state.speed)
        }
    } else {
        "Off".to_string()
    };

    let preset_label = active_visualization_preset(vis_state, anim_state)
        .and_then(|index| VISUALIZATION_PRESETS.get(index))
        .map(|preset| preset.name)
        .unwrap_or("Custom");

    draw_text(
        &format!("Preset: {}", preset_label),
        x + 18.0,
        y + 56.0,
        24.0,
        WHITE,
    );
    draw_text(
        &format!(
            "Color: {}   Mode: {}   Radius: {:.2}x",
            vis_state.color_scheme.label(),
            vis_state.render_mode.label(),
            vis_state.radius_scale
        ),
        x + 18.0,
        y + 84.0,
        20.0,
        GRAY,
    );
    draw_text(
        &format!(
            "Theme: {}   Mapping: {}   Animation: {}",
            vis_state.color_maps.current_theme_name(),
            vis_state.color_maps.mapping_index,
            animation_label
        ),
        x + 18.0,
        y + 110.0,
        20.0,
        GRAY,
    );

    let chain_focus = summary
        .chain_summaries
        .iter()
        .take(3)
        .map(|chain| format!("{}:{} atoms", display_chain_id(&chain.id), chain.atom_count))
        .collect::<Vec<_>>()
        .join("   ");
    draw_text(
        &format!("Largest chains: {}", chain_focus),
        x + 18.0,
        y + 136.0,
        18.0,
        LIGHTGRAY,
    );

    draw_palette_preview(x + 18.0, y + 148.0, vis_state);
}

fn draw_composition_panel(x: f32, y: f32, width: f32, summary: &StructureSummary) {
    draw_panel(
        x,
        y,
        width,
        170.0,
        "Composition cues",
        "Top residues and palette-driving chemistry",
    );

    let chains = summary
        .chain_summaries
        .iter()
        .take(4)
        .map(|chain| {
            format!(
                "{} ({} res, {} atoms)",
                display_chain_id(&chain.id),
                chain.residue_count,
                chain.atom_count
            )
        })
        .collect::<Vec<_>>()
        .join("   ");
    draw_wrapped_line("Chains", &chains, x + 18.0, y + 58.0);

    let elements = format_top_counts(&summary.element_counts, 5, "");
    draw_wrapped_line("Elements", &elements, x + 18.0, y + 86.0);

    let groups = format_top_counts(&summary.residue_group_counts, 5, " res");
    draw_wrapped_line("AA groups", &groups, x + 18.0, y + 114.0);

    let residues = format_top_counts(&summary.residue_type_counts, 6, " res");
    draw_wrapped_line("Residues", &residues, x + 18.0, y + 142.0);
}

fn draw_preset_panel(
    x: f32,
    y: f32,
    width: f32,
    vis_state: &VisualizationState,
    anim_state: &AnimationState,
) {
    let panel_height = 68.0 + VISUALIZATION_PRESETS.len() as f32 * 63.0;
    draw_panel(
        x,
        y,
        width,
        panel_height,
        "Visualization examples",
        "Press 1–6 for instant storytelling presets",
    );

    let active_index = active_visualization_preset(vis_state, anim_state);
    let mut card_y = y + 52.0;

    for (index, preset) in VISUALIZATION_PRESETS.iter().enumerate() {
        let is_active = active_index == Some(index);
        draw_preset_card(x + 12.0, card_y, width - 24.0, preset, is_active);
        card_y += 63.0;
    }
}

fn draw_preset_card(
    x: f32,
    y: f32,
    width: f32,
    preset: &crate::visualization_presets::VisualizationPreset,
    is_active: bool,
) {
    let background = if is_active {
        Color::new(0.13, 0.21, 0.32, 0.92)
    } else {
        Color::new(0.08, 0.12, 0.18, 0.78)
    };
    let border = if is_active {
        Color::new(0.39, 0.74, 1.0, 0.95)
    } else {
        Color::new(0.25, 0.34, 0.45, 0.92)
    };
    let badge = if is_active {
        Color::new(0.95, 0.78, 0.25, 1.0)
    } else {
        Color::new(0.28, 0.48, 0.66, 1.0)
    };

    draw_rectangle(x, y, width, 54.0, background);
    draw_rectangle_lines(x, y, width, 54.0, 1.5, border);
    draw_rectangle(x + 10.0, y + 10.0, 26.0, 26.0, badge);
    draw_text(preset.key, x + 18.0, y + 29.0, 20.0, BLACK);
    draw_text(preset.name, x + 48.0, y + 24.0, 21.0, WHITE);
    draw_text(preset.description, x + 48.0, y + 43.0, 16.0, LIGHTGRAY);
}

fn draw_palette_preview(x: f32, y: f32, vis_state: &VisualizationState) {
    let element_colors = vis_state.color_maps.elements();
    let swatches = [
        ("C", element_colors.get("C").copied().unwrap_or(LIGHTGRAY)),
        ("O", element_colors.get("O").copied().unwrap_or(RED)),
        ("N", element_colors.get("N").copied().unwrap_or(SKYBLUE)),
        ("S", element_colors.get("S").copied().unwrap_or(YELLOW)),
    ];

    let mut cursor_x = x;
    for (label, color) in swatches {
        draw_circle(cursor_x, y, 6.0, color);
        draw_text(label, cursor_x + 10.0, y + 5.0, 18.0, LIGHTGRAY);
        cursor_x += 52.0;
    }
}

fn draw_metric(x: f32, y: f32, label: &str, value: usize) {
    draw_text(label, x, y, 18.0, LIGHTGRAY);
    draw_text(&value.to_string(), x, y + 26.0, 30.0, WHITE);
}

fn draw_wrapped_line(label: &str, value: &str, x: f32, y: f32) {
    draw_text(&format!("{label}:"), x, y, 18.0, LIGHTGRAY);
    draw_text(value, x + 92.0, y, 18.0, GRAY);
}

fn draw_footer_hint(x: f32, y: f32) {
    draw_text(
        "C/B/M/N refine view · T/Y/,/. animate · P opens color editor · H hides dashboard",
        x,
        y,
        18.0,
        LIGHTGRAY,
    );
}

fn draw_panel(x: f32, y: f32, width: f32, height: f32, title: &str, subtitle: &str) {
    let background = Color::new(0.03, 0.05, 0.09, 0.82);
    let border = Color::new(0.19, 0.36, 0.56, 0.95);
    let title_bar = Color::new(0.07, 0.11, 0.17, 0.94);

    draw_rectangle(x, y, width, height, background);
    draw_rectangle_lines(x, y, width, height, 2.0, border);
    draw_rectangle(x, y, width, 30.0, title_bar);
    draw_text(title, x + 14.0, y + 20.0, 22.0, WHITE);
    draw_text(subtitle, x + 14.0, y + 44.0, 16.0, LIGHTGRAY);
}

fn format_top_counts(entries: &[(String, usize)], limit: usize, suffix: &str) -> String {
    entries
        .iter()
        .take(limit)
        .map(|(label, count)| format!("{label} {count}{suffix}"))
        .collect::<Vec<_>>()
        .join("   ")
}

fn display_chain_id(id: &str) -> &str {
    if id.trim().is_empty() { "?" } else { id }
}
