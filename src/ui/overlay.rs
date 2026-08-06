use macroquad::prelude::*;

use crate::{
    animation::AnimationState,
    structure_summary::StructureSummary,
    visualization_presets::{VISUALIZATION_PRESETS, active_visualization_preset},
    visualization_state::VisualizationState,
};

const PANEL_GAP: f32 = 8.0;
const PANEL_PADDING: f32 = 14.0;
const SUMMARY_PANEL_HEIGHT: f32 = 114.0;
const CURRENT_VIEW_PANEL_HEIGHT: f32 = 130.0;
const COMPOSITION_PANEL_HEIGHT: f32 = 144.0;
const BOTTOM_PANEL_HEIGHT: f32 = 176.0;

pub fn draw_info_overlay(
    structure_summary: &StructureSummary,
    vis_state: &VisualizationState,
    anim_state: &AnimationState,
    color_picker_open: bool,
) {
    let margin = 12.0;
    let reserved_right = if color_picker_open { 292.0 } else { 0.0 };
    let available_width = (screen_width() - margin * 2.0 - reserved_right).max(220.0);
    let dashboard_width = (screen_width() * 0.44)
        .clamp(320.0, 460.0)
        .min(available_width);
    let x = margin;
    let mut y = margin;

    draw_summary_panel(x, y, dashboard_width, structure_summary);
    y += SUMMARY_PANEL_HEIGHT + PANEL_GAP;

    draw_current_view_panel(
        x,
        y,
        dashboard_width,
        structure_summary,
        vis_state,
        anim_state,
    );
    y += CURRENT_VIEW_PANEL_HEIGHT + PANEL_GAP;

    draw_composition_panel(x, y, dashboard_width, structure_summary);
    y += COMPOSITION_PANEL_HEIGHT + PANEL_GAP;

    let controls_width = ((dashboard_width - PANEL_GAP) * 0.58).floor();
    let preset_width = dashboard_width - PANEL_GAP - controls_width;

    draw_controls_panel(x, y, controls_width);
    draw_preset_panel(
        x + controls_width + PANEL_GAP,
        y,
        preset_width,
        vis_state,
        anim_state,
    );
}

fn draw_summary_panel(x: f32, y: f32, width: f32, summary: &StructureSummary) {
    draw_panel(
        x,
        y,
        width,
        SUMMARY_PANEL_HEIGHT,
        "Scientific summary",
        "Instant structure context",
    );

    let column_width = (width - PANEL_PADDING * 2.0) / 3.0;
    draw_metric(
        x + PANEL_PADDING,
        y + 58.0,
        column_width,
        "Atoms",
        summary.atom_count,
    );
    draw_metric(
        x + PANEL_PADDING + column_width,
        y + 58.0,
        column_width,
        "Residues",
        summary.residue_count,
    );
    draw_metric(
        x + PANEL_PADDING + column_width * 2.0,
        y + 58.0,
        column_width,
        "Chains",
        summary.chain_summaries.len(),
    );

    let residue_span = if summary.min_residue_num == summary.max_residue_num {
        format!("Residue span: {}", summary.min_residue_num)
    } else {
        format!(
            "Residue span: {} → {}",
            summary.min_residue_num, summary.max_residue_num
        )
    };
    draw_single_line(
        &residue_span,
        x + PANEL_PADDING,
        y + 104.0,
        width - PANEL_PADDING * 2.0,
        16,
        GRAY,
    );
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
        CURRENT_VIEW_PANEL_HEIGHT,
        "Current visualization",
        "Compact state snapshot",
    );

    let animation_label = if anim_state.enabled {
        if anim_state.reverse {
            format!("{} {:.1}x rev", anim_state.mode_name(), anim_state.speed)
        } else {
            format!("{} {:.1}x", anim_state.mode_name(), anim_state.speed)
        }
    } else {
        "Off".to_string()
    };

    let preset_label = active_visualization_preset(vis_state, anim_state)
        .and_then(|index| VISUALIZATION_PRESETS.get(index))
        .map(|preset| preset.name)
        .unwrap_or("Custom");
    let content_width = width - PANEL_PADDING * 2.0;

    draw_single_line(
        &format!("Preset: {}", preset_label),
        x + PANEL_PADDING,
        y + 58.0,
        content_width,
        18,
        WHITE,
    );

    draw_single_line(
        &format!(
            "Mode: {} · Color {} · Radius {:.2}x",
            vis_state.render_mode.label(),
            vis_state.color_scheme.label(),
            vis_state.radius_scale
        ),
        x + PANEL_PADDING,
        y + 78.0,
        content_width,
        16,
        GRAY,
    );

    draw_single_line(
        &format!(
            "Theme: {} · Map {}/24",
            vis_state.color_maps.current_theme_name(),
            vis_state.color_maps.mapping_index + 1
        ),
        x + PANEL_PADDING,
        y + 98.0,
        content_width,
        16,
        GRAY,
    );

    let chain_focus = summary
        .chain_summaries
        .iter()
        .take(2)
        .map(|chain| format!("{} {}a", display_chain_id(&chain.id), chain.atom_count))
        .collect::<Vec<_>>()
        .join(" · ");

    draw_single_line(
        &format!("Anim: {} · Chains {}", animation_label, chain_focus),
        x + PANEL_PADDING,
        y + 118.0,
        content_width,
        16,
        LIGHTGRAY,
    );
}

fn draw_composition_panel(x: f32, y: f32, width: f32, summary: &StructureSummary) {
    draw_panel(
        x,
        y,
        width,
        COMPOSITION_PANEL_HEIGHT,
        "Composition cues",
        "Short chemistry summary",
    );

    let chains = summary
        .chain_summaries
        .iter()
        .take(3)
        .map(|chain| {
            format!(
                "{} {}r {}a",
                display_chain_id(&chain.id),
                chain.residue_count,
                chain.atom_count
            )
        })
        .collect::<Vec<_>>()
        .join(" · ");
    draw_labeled_line(
        "Chains",
        &chains,
        x + PANEL_PADDING,
        y + 58.0,
        width - PANEL_PADDING * 2.0,
    );

    let elements = format_top_counts(&summary.element_counts, 4, "");
    draw_labeled_line(
        "Elements",
        &elements,
        x + PANEL_PADDING,
        y + 80.0,
        width - PANEL_PADDING * 2.0,
    );

    let groups = format_top_counts(&summary.residue_group_counts, 4, "r");
    draw_labeled_line(
        "AA groups",
        &groups,
        x + PANEL_PADDING,
        y + 102.0,
        width - PANEL_PADDING * 2.0,
    );

    let residues = format_top_counts(&summary.residue_type_counts, 4, "r");
    draw_labeled_line(
        "Residues",
        &residues,
        x + PANEL_PADDING,
        y + 124.0,
        width - PANEL_PADDING * 2.0,
    );
}

fn draw_controls_panel(x: f32, y: f32, width: f32) {
    draw_panel(
        x,
        y,
        width,
        BOTTOM_PANEL_HEIGHT,
        "Key map",
        "Visible controls",
    );

    let mut line_y = y + 58.0;
    let content_width = width - PANEL_PADDING * 2.0;

    line_y = draw_labeled_block(
        "Move",
        "Drag orbit · wheel zoom · arrows pan · middle drag pan",
        x + PANEL_PADDING,
        line_y,
        content_width,
        13,
        14.0,
        2,
        GRAY,
    ) + 6.0;

    line_y = draw_labeled_block(
        "Rotate",
        "W/S X · A/D Y · Q/E Z",
        x + PANEL_PADDING,
        line_y,
        content_width,
        13,
        14.0,
        2,
        GRAY,
    ) + 6.0;

    line_y = draw_labeled_block(
        "View",
        "C color · B mode · +/- scale · 0 reset · 1-6 presets",
        x + PANEL_PADDING,
        line_y,
        content_width,
        13,
        14.0,
        2,
        GRAY,
    ) + 6.0;

    line_y = draw_labeled_block(
        "Theme",
        "M/N cycle · Shift+M/N back · T/Y/U/,/. animate",
        x + PANEL_PADDING,
        line_y,
        content_width,
        13,
        14.0,
        2,
        GRAY,
    ) + 6.0;

    draw_labeled_block(
        "UI",
        "H dashboard · P picker · Tab/J/K/1-6 in picker · X/V export",
        x + PANEL_PADDING,
        line_y,
        content_width,
        13,
        14.0,
        2,
        GRAY,
    );
}

fn draw_preset_panel(
    x: f32,
    y: f32,
    width: f32,
    vis_state: &VisualizationState,
    anim_state: &AnimationState,
) {
    let active_index = active_visualization_preset(vis_state, anim_state);
    draw_panel(x, y, width, BOTTOM_PANEL_HEIGHT, "Examples", "Press 1-6");

    let mut row_y = y + 58.0;

    for (index, preset) in VISUALIZATION_PRESETS.iter().enumerate() {
        let is_active = active_index == Some(index);
        draw_preset_row(x + 10.0, row_y, width - 20.0, preset, is_active);
        row_y += 18.0;
    }

    let detail = active_index
        .and_then(|index| VISUALIZATION_PRESETS.get(index))
        .map(|preset| preset.description)
        .unwrap_or("Custom view");
    draw_single_line(detail, x + 10.0, y + 168.0, width - 20.0, 12, LIGHTGRAY);
}

fn draw_preset_row(
    x: f32,
    y: f32,
    width: f32,
    preset: &crate::visualization_presets::VisualizationPreset,
    is_active: bool,
) {
    let background = if is_active {
        Color::new(0.12, 0.20, 0.31, 0.92)
    } else {
        Color::new(0.07, 0.11, 0.17, 0.78)
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

    draw_rectangle(x, y - 12.0, width, 16.0, background);
    draw_rectangle_lines(x, y - 12.0, width, 16.0, 1.0, border);
    draw_rectangle(x + 6.0, y - 10.0, 16.0, 12.0, badge);
    draw_text(preset.key, x + 10.0, y, 13.0, BLACK);
    draw_single_line(
        preset.name,
        x + 28.0,
        y,
        width - 42.0,
        14,
        if is_active { WHITE } else { LIGHTGRAY },
    );
    if is_active {
        draw_circle(x + width - 10.0, y - 4.0, 3.0, badge);
    }
}

fn draw_metric(x: f32, y: f32, width: f32, label: &str, value: usize) {
    let label = truncate_to_width(label, width, 16);
    let value_text = value.to_string();
    let label_dims = measure_text(&label, None, 16, 1.0);
    let value_dims = measure_text(&value_text, None, 24, 1.0);

    draw_text(
        &label,
        x + (width - label_dims.width) * 0.5,
        y,
        16.0,
        LIGHTGRAY,
    );
    draw_text(
        &value_text,
        x + (width - value_dims.width) * 0.5,
        y + 24.0,
        24.0,
        WHITE,
    );
}

fn draw_labeled_line(label: &str, value: &str, x: f32, y: f32, width: f32) {
    let label_width = 64.0;
    draw_text(label, x, y, 15.0, LIGHTGRAY);
    draw_single_line(value, x + label_width, y, width - label_width, 15, GRAY);
}

fn draw_labeled_block(
    label: &str,
    value: &str,
    x: f32,
    y: f32,
    width: f32,
    font_size: u16,
    line_height: f32,
    max_lines: usize,
    value_color: Color,
) -> f32 {
    let label_width = 48.0;
    draw_text(label, x, y, font_size as f32, LIGHTGRAY);
    let mut cursor_y = y;
    for line in wrap_text_lines(value, width - label_width, font_size, max_lines) {
        draw_text(
            &line,
            x + label_width,
            cursor_y,
            font_size as f32,
            value_color,
        );
        cursor_y += line_height;
    }
    cursor_y
}

fn draw_single_line(text: &str, x: f32, y: f32, max_width: f32, font_size: u16, color: Color) {
    let text = truncate_to_width(text, max_width, font_size);
    draw_text(&text, x, y, font_size as f32, color);
}

fn draw_panel(x: f32, y: f32, width: f32, height: f32, title: &str, subtitle: &str) {
    let background = Color::new(0.03, 0.05, 0.09, 0.82);
    let border = Color::new(0.19, 0.36, 0.56, 0.95);
    let title_bar = Color::new(0.07, 0.11, 0.17, 0.94);
    let title = truncate_to_width(title, width - 28.0, 20);
    let subtitle = truncate_to_width(subtitle, width - 28.0, 14);

    draw_rectangle(x, y, width, height, background);
    draw_rectangle_lines(x, y, width, height, 2.0, border);
    draw_rectangle(x, y, width, 28.0, title_bar);
    draw_text(&title, x + 14.0, y + 20.0, 20.0, WHITE);
    draw_text(&subtitle, x + 14.0, y + 42.0, 14.0, LIGHTGRAY);
}

fn format_top_counts(entries: &[(String, usize)], limit: usize, suffix: &str) -> String {
    entries
        .iter()
        .take(limit)
        .map(|(label, count)| {
            if suffix.is_empty() {
                format!("{label} {count}")
            } else {
                format!("{label} {count}{suffix}")
            }
        })
        .collect::<Vec<_>>()
        .join(" · ")
}

fn wrap_text_lines(text: &str, max_width: f32, font_size: u16, max_lines: usize) -> Vec<String> {
    if max_lines == 0 {
        return Vec::new();
    }

    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        let candidate = if current.is_empty() {
            word.to_string()
        } else {
            format!("{current} {word}")
        };

        if current.is_empty() || measure_text(&candidate, None, font_size, 1.0).width <= max_width {
            current = candidate;
        } else {
            lines.push(current);
            current = word.to_string();
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    if lines.len() > max_lines {
        let overflow = lines[max_lines - 1..].join(" ");
        lines.truncate(max_lines);
        lines[max_lines - 1] = truncate_to_width(&overflow, max_width, font_size);
    } else {
        for line in &mut lines {
            *line = truncate_to_width(line, max_width, font_size);
        }
    }

    lines
}

fn truncate_to_width(text: &str, max_width: f32, font_size: u16) -> String {
    if measure_text(text, None, font_size, 1.0).width <= max_width {
        return text.to_string();
    }

    let ellipsis = '…';
    let mut chars: Vec<char> = text.chars().collect();

    while !chars.is_empty() {
        chars.pop();
        let candidate = format!(
            "{}{}",
            chars.iter().collect::<String>().trim_end(),
            ellipsis
        );
        if measure_text(&candidate, None, font_size, 1.0).width <= max_width {
            return candidate;
        }
    }

    ellipsis.to_string()
}

fn display_chain_id(id: &str) -> &str {
    if id.trim().is_empty() { "?" } else { id }
}
