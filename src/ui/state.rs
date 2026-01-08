use macroquad::prelude::*;

pub struct UIState {
    pub show_ui: bool,
    pub show_color_picker: bool,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            show_ui: true,
            show_color_picker: false,
        }
    }
    
    pub fn handle_toggles(&mut self) {
        if is_key_pressed(KeyCode::H) {
            self.show_ui = !self.show_ui;
        }
        if is_key_pressed(KeyCode::P) {
            self.show_color_picker = !self.show_color_picker;
        }
    }
}
