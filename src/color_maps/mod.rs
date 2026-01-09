use macroquad::prelude::*;
use std::collections::HashMap;

mod themes;
pub use themes::{Theme, create_builtin_themes};

pub struct ColorMaps {
    pub themes: Vec<Theme>,
    pub current_theme_index: usize,
    pub mapping_index: usize,
    pub mapping_rotation_element: usize,
    pub mapping_rotation_aa: usize,
}

impl ColorMaps {
    pub fn new() -> Self {
        let themes = create_builtin_themes();
        
        Self { 
            themes,
            current_theme_index: 0,
            mapping_index: 0,
            mapping_rotation_element: 0,
            mapping_rotation_aa: 0,
        }
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    pub fn merge_with_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        use yaml_rust::YamlLoader;
        
        let content = std::fs::read_to_string(path)?;
        let docs = YamlLoader::load_from_str(&content)?;
        
        if docs.is_empty() {
            return Ok(());
        }
        
        let doc = &docs[0];
        
        // Parse elements into current theme
        if let Some(elements_node) = doc["elements"].as_hash() {
            if let Some(theme) = self.themes.get_mut(self.current_theme_index) {
                for (key, value) in elements_node {
                    if let (Some(name), Some(hex)) = (key.as_str(), value.as_str()) {
                        if let Some(color) = parse_hex_color(hex) {
                            theme.elements.insert(name.to_string(), color);
                        }
                    }
                }
            }
        }
        
        // Parse amino_acid_groups into current theme
        if let Some(aa_groups_node) = doc["amino_acid_groups"].as_hash() {
            if let Some(theme) = self.themes.get_mut(self.current_theme_index) {
                for (key, value) in aa_groups_node {
                    if let (Some(name), Some(hex)) = (key.as_str(), value.as_str()) {
                        if let Some(color) = parse_hex_color(hex) {
                            theme.aa_groups.insert(name.to_string(), color);
                        }
                    }
                }
            }
        }
        
        // Parse amino_acid_types into current theme
        if let Some(aa_types_node) = doc["amino_acid_types"].as_hash() {
            if let Some(theme) = self.themes.get_mut(self.current_theme_index) {
                for (key, value) in aa_types_node {
                    if let (Some(name), Some(hex)) = (key.as_str(), value.as_str()) {
                        if let Some(color) = parse_hex_color(hex) {
                            theme.aa_types.insert(name.to_string(), color);
                        }
                    }
                }
            }
        }
        
        // Parse gradient colors into current theme
        if let Some(gradient_node) = doc["gradient"].as_hash() {
            if let Some(theme) = self.themes.get_mut(self.current_theme_index) {
                if let Some(start_hex) = gradient_node.get(&yaml_rust::Yaml::String("start".to_string())) {
                    if let Some(hex_str) = start_hex.as_str() {
                        if let Some(color) = parse_hex_color(hex_str) {
                            theme.gradient_start = color;
                        }
                    }
                }
                if let Some(end_hex) = gradient_node.get(&yaml_rust::Yaml::String("end".to_string())) {
                    if let Some(hex_str) = end_hex.as_str() {
                        if let Some(color) = parse_hex_color(hex_str) {
                            theme.gradient_end = color;
                        }
                    }
                }
            }
        }
        
        // Parse themes - collect them first to insert before built-in themes
        if let Some(themes_node) = doc["themes"].as_hash() {
            let mut yaml_themes = Vec::new();
            
            for (theme_name_yaml, theme_data) in themes_node {
                if let Some(theme_name) = theme_name_yaml.as_str() {
                    // Start with default theme as base
                    let default_theme = self.themes.get(0)
                        .cloned()
                        .unwrap_or_else(|| create_builtin_themes()[0].clone());
                    
                    let mut theme = Theme {
                        name: theme_name.to_string(),
                        elements: default_theme.elements,
                        aa_groups: default_theme.aa_groups,
                        aa_types: default_theme.aa_types,
                        gradient_start: default_theme.gradient_start,
                        gradient_end: default_theme.gradient_end,
                        background: default_theme.background,
                    };
                    
                    // Parse theme elements
                    if let Some(elements_node) = theme_data["elements"].as_hash() {
                        for (key, value) in elements_node {
                            if let (Some(name), Some(hex)) = (key.as_str(), value.as_str()) {
                                if let Some(color) = parse_hex_color(hex) {
                                    theme.elements.insert(name.to_string(), color);
                                }
                            }
                        }
                    }
                    
                    // Parse theme amino_acid_groups
                    if let Some(aa_groups_node) = theme_data["amino_acid_groups"].as_hash() {
                        for (key, value) in aa_groups_node {
                            if let (Some(name), Some(hex)) = (key.as_str(), value.as_str()) {
                                if let Some(color) = parse_hex_color(hex) {
                                    theme.aa_groups.insert(name.to_string(), color);
                                }
                            }
                        }
                    }
                    
                    // Parse theme amino_acid_types
                    if let Some(aa_types_node) = theme_data["amino_acid_types"].as_hash() {
                        for (key, value) in aa_types_node {
                            if let (Some(name), Some(hex)) = (key.as_str(), value.as_str()) {
                                if let Some(color) = parse_hex_color(hex) {
                                    theme.aa_types.insert(name.to_string(), color);
                                }
                            }
                        }
                    }
                    
                    // Parse theme gradient
                    if let Some(gradient_node) = theme_data["gradient"].as_hash() {
                        if let Some(start_hex) = gradient_node.get(&yaml_rust::Yaml::String("start".to_string())) {
                            if let Some(hex_str) = start_hex.as_str() {
                                if let Some(color) = parse_hex_color(hex_str) {
                                    theme.gradient_start = color;
                                }
                            }
                        }
                        if let Some(end_hex) = gradient_node.get(&yaml_rust::Yaml::String("end".to_string())) {
                            if let Some(hex_str) = end_hex.as_str() {
                                if let Some(color) = parse_hex_color(hex_str) {
                                    theme.gradient_end = color;
                                }
                            }
                        }
                    }
                    
                    // Parse theme background
                    if let Some(bg_hex) = theme_data["background"].as_str() {
                        if let Some(color) = parse_hex_color(bg_hex) {
                            theme.background = color;
                        }
                    }
                    
                    yaml_themes.push(theme);
                }
            }
            
            // Insert all YAML themes at the beginning (before built-in themes)
            for theme in yaml_themes.into_iter().rev() {
                self.themes.insert(0, theme);
            }
        }
        
        Ok(())
    }
    
    pub fn next_theme(&mut self) {
        if !self.themes.is_empty() {
            self.current_theme_index = (self.current_theme_index + 1) % self.themes.len();
            self.mapping_index = 0; // Reset mapping when changing theme
            self.mapping_rotation_element = 0;
            self.mapping_rotation_aa = 0;
        }
    }
    pub fn prev_theme(&mut self) {
        if !self.themes.is_empty() {
            if self.current_theme_index == 0 {
                self.current_theme_index = self.themes.len() - 1;
            } else {
                self.current_theme_index -= 1;
            }
            self.mapping_index = 0; // Reset mapping when changing theme
            self.mapping_rotation_element = 0;
            self.mapping_rotation_aa = 0;
        }
    }
    
    pub fn next_mapping_smart(&mut self, color_scheme: crate::types::ColorScheme) {
        use crate::types::ColorScheme;
        
        // Determine step size based on which rotations affect the current color scheme:
        // - ByElement: element rotation matters, step by 1
        // - ByAminoAcidGroup: aa_group rotation matters, step by 6 to skip element rotations
        // - ByAminoAcidType: only aa_types (no rotation), step by 6 to change aa_group rotation
        // - NToCGradient: gradient direction matters, step by 12
        // - Theme: all rotations matter, step by 1
        let step = match color_scheme {
            ColorScheme::ByAminoAcidGroup => 6, // Skip element rotations
            ColorScheme::ByAminoAcidType => 6,  // Skip element rotations
            ColorScheme::NToCGradient => 12,    // Skip to gradient direction change
            _ => 1,  // Element, RandomChain, Theme all use 1
        };
        
        self.mapping_index = (self.mapping_index + step) % 24;
        self.mapping_rotation_element = self.mapping_index % 6;
        self.mapping_rotation_aa = (self.mapping_index / 6) % 4;
    }
    
    pub fn previous_mapping_smart(&mut self, color_scheme: crate::types::ColorScheme) {
        use crate::types::ColorScheme;
        
        // Same step logic as next_mapping_smart
        let step = match color_scheme {
            ColorScheme::ByAminoAcidGroup => 6,
            ColorScheme::ByAminoAcidType => 6,
            ColorScheme::NToCGradient => 12,
            _ => 1,
        };
        
        // Go backwards with wrapping
        self.mapping_index = if self.mapping_index >= step {
            self.mapping_index - step
        } else {
            24 - (step - self.mapping_index)
        };
        self.mapping_rotation_element = self.mapping_index % 6;
        self.mapping_rotation_aa = (self.mapping_index / 6) % 4;
    }
    
    pub fn current_theme_name(&self) -> &str {
        self.themes.get(self.current_theme_index)
            .map(|t| t.name.as_str())
            .unwrap_or("Default")
    }
    
    pub fn elements(&self) -> HashMap<String, Color> {
        self.get_mapped_elements()
    }
    
    fn get_mapped_elements(&self) -> HashMap<String, Color> {
        let theme = &self.themes[self.current_theme_index];
        let base_elements = &theme.elements;
        
        if self.mapping_index == 0 {
            return base_elements.clone();
        }
        
        // Extract colors from elements in a fixed order
        let keys = ["C", "O", "N", "S", "H", "P"];
        let mut colors: Vec<Color> = Vec::new();
        for key in &keys {
            if let Some(color) = base_elements.get(*key) {
                colors.push(*color);
            }
        }
        
        if colors.len() < 6 {
            return base_elements.clone();
        }
        
        // Rotate colors based on element rotation (mapping_index % 6)
        let element_rotation = self.mapping_index % 6;
        let mut result = HashMap::new();
        for (i, key) in keys.iter().enumerate() {
            let color_idx = (i + element_rotation) % colors.len();
            result.insert(key.to_string(), colors[color_idx]);
        }
        result
    }
    
    pub fn aa_groups(&self) -> HashMap<String, Color> {
        self.get_mapped_aa_groups()
    }
    
    fn get_mapped_aa_groups(&self) -> HashMap<String, Color> {
        let theme = &self.themes[self.current_theme_index];
        let base_groups = &theme.aa_groups;
        
        if self.mapping_index == 0 {
            return base_groups.clone();
        }
        
        // Extract colors from aa_groups in a fixed order (excluding Glycine)
        let keys = ["Hydrophobic", "Polar", "Positive", "Negative"];
        let mut colors: Vec<Color> = Vec::new();
        for key in &keys {
            if let Some(color) = base_groups.get(*key) {
                colors.push(*color);
            }
        }
        
        if colors.is_empty() {
            return base_groups.clone();
        }
        
        // Rotate colors based on aa_group rotation (mapping_index / 6) % 4
        let aa_rotation = (self.mapping_index / 6) % 4;
        let mut result = HashMap::new();
        for (i, key) in keys.iter().enumerate() {
            let color_idx = (i + aa_rotation) % colors.len();
            result.insert(key.to_string(), colors[color_idx]);
        }
        
        // Keep Glycine unchanged
        if let Some(gly_color) = base_groups.get("Glycine") {
            result.insert("Glycine".to_string(), *gly_color);
        }
        
        result
    }
    
    pub fn aa_types(&self) -> &HashMap<String, Color> {
        &self.themes[self.current_theme_index].aa_types
    }
    
    pub fn gradient_start(&self) -> Color {
        let theme = &self.themes[self.current_theme_index];
        // Swap gradient every 12 mappings (mapping_index / 12 determines direction)
        if (self.mapping_index / 12) % 2 == 0 {
            theme.gradient_start
        } else {
            theme.gradient_end
        }
    }
    
    pub fn gradient_end(&self) -> Color {
        let theme = &self.themes[self.current_theme_index];
        // Swap gradient every 12 mappings
        if (self.mapping_index / 12) % 2 == 0 {
            theme.gradient_end
        } else {
            theme.gradient_start
        }
    }
    
    pub fn current_background(&self) -> Color {
        self.themes[self.current_theme_index].background
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_hex_color(hex: &str) -> Option<Color> {
    let hex = hex.trim().trim_start_matches('#');
    
    let (r, g, b, a) = match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            (r, g, b, 255)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            (r, g, b, a)
        }
        _ => return None,
    };
    
    Some(Color::from_rgba(r, g, b, a))
}

// Generate random but consistent color based on seed using theme colors
pub fn seeded_random_color(seed: u32, color_maps: &ColorMaps) -> Color {
    // Get theme colors to base random colors on
    let theme = &color_maps.themes[color_maps.current_theme_index];
    
    // Use gradient colors as base palette
    let start = theme.gradient_start;
    let end = theme.gradient_end;
    
    // Get additional colors from aa_groups for more variety
    let hydrophobic = theme.aa_groups.get("Hydrophobic").copied().unwrap_or(Color::from_rgba(255, 150, 50, 255));
    let polar = theme.aa_groups.get("Polar").copied().unwrap_or(Color::from_rgba(80, 220, 100, 255));
    let positive = theme.aa_groups.get("Positive").copied().unwrap_or(Color::from_rgba(50, 120, 255, 255));
    let negative = theme.aa_groups.get("Negative").copied().unwrap_or(Color::from_rgba(255, 60, 60, 255));
    
    // Create palette from theme colors
    let palette = [
        start, end, hydrophobic, polar, positive, negative,
    ];
    
    // Use seed to select and blend colors from palette
    let idx1 = (seed % 6) as usize;
    let idx2 = ((seed / 6) % 6) as usize;
    let blend_factor = ((seed / 36) % 100) as f32 / 100.0;
    
    let c1 = palette[idx1];
    let c2 = palette[idx2];
    
    // Blend the two selected colors
    Color::from_rgba(
        ((c1.r * (1.0 - blend_factor) + c2.r * blend_factor) * 255.0) as u8,
        ((c1.g * (1.0 - blend_factor) + c2.g * blend_factor) * 255.0) as u8,
        ((c1.b * (1.0 - blend_factor) + c2.b * blend_factor) * 255.0) as u8,
        255
    )
}
