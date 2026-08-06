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
    let screen_w = screen_width();
    let available_width = (screen_w - margin * 2.0).max(320.0);
    let preferred_left_width = (screen_w * 0.31).clamp(380.0, 520.0).min(available_width);
    let preferred_right_width = (screen_w * 0.24).clamp(300.0, 420.0).min(available_width);
    let split_layout =
        screen_w - margin * 2.0 - preferred_left_width - preferred_right_width >= 260.0;
    let left_width = if split_layout {
        preferred_left_width
    } else {
        available_width
    };
    let right_width = if split_layout {
        preferred_right_width
    } else {
        available_width
    };
    let left_x = margin;
    let right_x = if split_layout {
        screen_w - margin - right_width
    } else {
        left_x
    };
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
    let preset_y = if split_layout { margin } else { left_y + 184.0 };

    draw_preset_panel(right_x, preset_y, right_width, vis_state, anim_state);
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
        .take(2)
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
    let active_index = active_visualization_preset(vis_state, anim_state);
    let theme_detail = active_index
        .and_then(|index| VISUALIZATION_PRESETS.get(index))
        .is_some_and(|preset| preset.color_scheme == crate::types::ColorScheme::Theme);
    let row_spacing = 34.0;
    let detail_height = if theme_detail { 114.0 } else { 72.0 };
    let panel_height = 62.0 + VISUALIZATION_PRESETS.len() as f32 * row_spacing + detail_height;
    draw_panel(
        x,
        y,
        width,
        panel_height,
        "Visualization examples",
        "Press 1–6 for instant storytelling presets",
    );

    let mut card_y = y + 52.0;

    for (index, preset) in VISUALIZATION_PRESETS.iter().enumerate() {
        let is_active = active_index == Some(index);
        draw_preset_card(x + 12.0, card_y, width - 24.0, preset, is_active);
        card_y += row_spacing;
    }

    draw_preset_focus(
        x + 12.0,
        card_y + 2.0,
        width - 24.0,
        vis_state,
        anim_state,
        active_index,
    );
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

    draw_rectangle(x, y, width, 28.0, background);
    draw_rectangle_lines(x, y, width, 28.0, 1.5, border);
    draw_rectangle(x + 8.0, y + 4.0, 22.0, 20.0, badge);
    draw_text(preset.key, x + 14.0, y + 18.0, 18.0, BLACK);
    draw_text(
        preset.name,
        x + 40.0,
        y + 19.0,
        20.0,
        if is_active { WHITE } else { LIGHTGRAY },
    );
    if is_active {
        draw_circle(x + width - 14.0, y + 14.0, 4.0, badge);
    }
}

fn draw_preset_focus(
    x: f32,
    y: f32,
    width: f32,
    vis_state: &VisualizationState,
    anim_state: &AnimationState,
    active_index: Option<usize>,
) {
    let theme_active = active_index
        .and_then(|index| VISUALIZATION_PRESETS.get(index))
        .is_some_and(|preset| preset.color_scheme == crate::types::ColorScheme::Theme);
    let height = if theme_active { 102.0 } else { 60.0 };
    let background = Color::new(0.06, 0.09, 0.14, 0.8);
    let border = Color::new(0.22, 0.33, 0.47, 0.92);

    draw_rectangle(x, y, width, height, background);
    draw_rectangle_lines(x, y, width, height, 1.5, border);
    draw_text("Preset focus", x + 10.0, y + 18.0, 18.0, WHITE);

    let animation_label = animation_summary(anim_state);

    if let Some(index) = active_index {
        let preset = &VISUALIZATION_PRESETS[index];
        draw_text(preset.description, x + 10.0, y + 38.0, 15.0, LIGHTGRAY);

        if preset.color_scheme == crate::types::ColorScheme::Theme {
            draw_text(
                &format!(
                    "Theme: {} · {} palettes · Map {}/24",
                    vis_state.color_maps.current_theme_name(),
                    vis_state.color_maps.themes.len(),
                    vis_state.color_maps.mapping_index + 1
                ),
                x + 10.0,
                y + 58.0,
                15.0,
                SKYBLUE,
            );
            draw_text(
                "M / Shift+M themes · N / Shift+N remixes",
                x + 10.0,
                y + 76.0,
                14.0,
                LIGHTGRAY,
            );
            draw_theme_spotlight_preview(x + 10.0, y + 84.0, vis_state);
        } else {
            draw_text(
                &format!(
                    "{} · {} · {}",
                    vis_state.color_scheme.label(),
                    vis_state.render_mode.label(),
                    animation_label
                ),
                x + 10.0,
                y + 56.0,
                15.0,
                GRAY,
            );
        }
    } else {
        draw_text(
            "Current view is custom. Press 1–6 to restage the structure.",
            x + 10.0,
            y + 38.0,
            15.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!(
                "{} · {} · {}",
                vis_state.color_scheme.label(),
                vis_state.render_mode.label(),
                animation_label
            ),
            x + 10.0,
            y + 56.0,
            15.0,
            GRAY,
        );
    }
}

fn draw_theme_spotlight_preview(x: f32, y: f32, vis_state: &VisualizationState) {
    let swatches = vis_state.color_maps.elements();
    let keys = ["C", "O", "N", "S", "H", "P"];
    let mut cursor_x = x;

    for key in keys {
        let color = swatches.get(key).copied().unwrap_or(LIGHTGRAY);
        draw_rectangle(cursor_x, y, 24.0, 10.0, color);
        draw_rectangle_lines(cursor_x, y, 24.0, 10.0, 1.0, WHITE);
        cursor_x += 28.0;
    }
}

fn animation_summary(anim_state: &AnimationState) -> String {
    if anim_state.enabled {
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
    }
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
