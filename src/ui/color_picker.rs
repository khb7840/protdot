use crate::color_maps::ColorMaps;
use macroquad::prelude::*;

pub struct ColorPickerState {
    pub category: usize,
    pub index: usize,
}

impl ColorPickerState {
    pub fn new() -> Self {
        Self {
            category: 0,
            index: 0,
        }
    }

    pub fn handle_navigation(&mut self, items_len: usize) {
        if is_key_pressed(KeyCode::Tab) {
            self.category = (self.category + 1) % 3;
            self.index = 0;
        }

        // Use J/K for navigation instead of arrow keys (which are used for translation)
        if is_key_pressed(KeyCode::K) && self.index > 0 {
            self.index -= 1;
        }
        if is_key_pressed(KeyCode::J) && self.index < items_len.saturating_sub(1) {
            self.index += 1;
        }
    }

    pub fn get_items(&self) -> Vec<String> {
        match self.category {
            0 => vec!["Background".to_string()],
            1 => vec![
                "C".to_string(),
                "O".to_string(),
                "N".to_string(),
                "S".to_string(),
                "H".to_string(),
                "P".to_string(),
            ],
            2 => vec![
                "Hydrophobic".to_string(),
                "Polar".to_string(),
                "Positive".to_string(),
                "Negative".to_string(),
                "Glycine".to_string(),
            ],
            _ => vec![],
        }
    }

    pub fn get_current_color(
        &self,
        bg_color: Color,
        color_maps: &ColorMaps,
        current_item: &str,
    ) -> Color {
        match self.category {
            0 => bg_color,
            1 => color_maps
                .elements()
                .get(current_item)
                .copied()
                .unwrap_or(GRAY),
            2 => color_maps
                .aa_groups()
                .get(current_item)
                .copied()
                .unwrap_or(GRAY),
            _ => GRAY,
        }
    }

    pub fn handle_color_adjustment(&self, current_color: Color) -> Color {
        let mut new_color = current_color;
        if is_key_pressed(KeyCode::Key1) {
            new_color.r = (new_color.r - 0.1).max(0.0);
        }
        if is_key_pressed(KeyCode::Key2) {
            new_color.r = (new_color.r + 0.1).min(1.0);
        }
        if is_key_pressed(KeyCode::Key3) {
            new_color.g = (new_color.g - 0.1).max(0.0);
        }
        if is_key_pressed(KeyCode::Key4) {
            new_color.g = (new_color.g + 0.1).min(1.0);
        }
        if is_key_pressed(KeyCode::Key5) {
            new_color.b = (new_color.b - 0.1).max(0.0);
        }
        if is_key_pressed(KeyCode::Key6) {
            new_color.b = (new_color.b + 0.1).min(1.0);
        }
        new_color
    }

    pub fn apply_color_change(
        &self,
        new_color: Color,
        bg_color: &mut Color,
        color_maps: &mut ColorMaps,
        current_item: &str,
    ) {
        match self.category {
            0 => {
                *bg_color = new_color;
                // Also update theme background
                if let Some(theme) = color_maps.themes.get_mut(color_maps.current_theme_index) {
                    theme.background = new_color;
                }
            }
            1 => {
                if let Some(theme) = color_maps.themes.get_mut(color_maps.current_theme_index) {
                    theme.elements.insert(current_item.to_string(), new_color);
                }
            }
            2 => {
                if let Some(theme) = color_maps.themes.get_mut(color_maps.current_theme_index) {
                    theme.aa_groups.insert(current_item.to_string(), new_color);
                }
            }
            _ => {}
        }
    }
}

pub fn draw_color_picker(
    picker_state: &mut ColorPickerState,
    bg_color: &mut Color,
    color_maps: &mut ColorMaps,
) {
    let panel_x = screen_width() - 280.0;
    let panel_y = 10.0;
    let panel_w = 270.0;
    let panel_h = 300.0;

    draw_rectangle(
        panel_x,
        panel_y,
        panel_w,
        panel_h,
        Color::new(0.2, 0.2, 0.2, 0.9),
    );
    draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, 2.0, WHITE);

    draw_text(
        "Color Settings (P to close)",
        panel_x + 10.0,
        panel_y + 25.0,
        20.0,
        WHITE,
    );

    let categories = ["Background", "Elements", "AA Groups"];
    let category_name = categories[picker_state.category];
    draw_text(
        &format!("Category (Tab): {}", category_name),
        panel_x + 10.0,
        panel_y + 55.0,
        18.0,
        YELLOW,
    );

    let items = picker_state.get_items();
    picker_state.handle_navigation(items.len());

    let current_item = &items[picker_state.index];
    draw_text(
        &format!("Item (J/K): {}", current_item),
        panel_x + 10.0,
        panel_y + 80.0,
        18.0,
        WHITE,
    );

    let current_color = picker_state.get_current_color(*bg_color, color_maps, current_item);

    draw_text("R (1/2)", panel_x + 10.0, panel_y + 115.0, 16.0, LIGHTGRAY);
    draw_text("G (3/4)", panel_x + 10.0, panel_y + 145.0, 16.0, LIGHTGRAY);
    draw_text("B (5/6)", panel_x + 10.0, panel_y + 175.0, 16.0, LIGHTGRAY);

    let bar_x = panel_x + 80.0;
    let bar_w = 170.0;
    draw_rectangle(bar_x, panel_y + 105.0, bar_w * current_color.r, 15.0, RED);
    draw_rectangle_lines(bar_x, panel_y + 105.0, bar_w, 15.0, 1.0, WHITE);

    draw_rectangle(bar_x, panel_y + 135.0, bar_w * current_color.g, 15.0, GREEN);
    draw_rectangle_lines(bar_x, panel_y + 135.0, bar_w, 15.0, 1.0, WHITE);

    draw_rectangle(bar_x, panel_y + 165.0, bar_w * current_color.b, 15.0, BLUE);
    draw_rectangle_lines(bar_x, panel_y + 165.0, bar_w, 15.0, 1.0, WHITE);

    draw_rectangle(panel_x + 10.0, panel_y + 205.0, 80.0, 40.0, current_color);
    draw_rectangle_lines(panel_x + 10.0, panel_y + 205.0, 80.0, 40.0, 2.0, WHITE);
    draw_text("Preview", panel_x + 100.0, panel_y + 228.0, 18.0, LIGHTGRAY);

    draw_text(
        "Tab: change category",
        panel_x + 10.0,
        panel_y + 265.0,
        14.0,
        DARKGRAY,
    );
    draw_text(
        "J/K: select item",
        panel_x + 10.0,
        panel_y + 280.0,
        14.0,
        DARKGRAY,
    );

    let new_color = picker_state.handle_color_adjustment(current_color);

    if new_color != current_color {
        picker_state.apply_color_change(new_color, bg_color, color_maps, current_item);
    }
}
