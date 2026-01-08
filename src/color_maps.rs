use macroquad::prelude::*;
use std::collections::HashMap;

pub struct ColorMaps {
    pub elements: HashMap<String, Color>,
    pub aa_groups: HashMap<String, Color>,
}

impl ColorMaps {
    pub fn new() -> Self {
        let mut elements = HashMap::new();
        elements.insert("C".to_string(), Color::from_rgba(64, 64, 64, 255));
        elements.insert("O".to_string(), Color::from_rgba(240, 80, 80, 255));
        elements.insert("N".to_string(), Color::from_rgba(80, 120, 240, 255));
        elements.insert("S".to_string(), Color::from_rgba(255, 200, 50, 255));
        elements.insert("H".to_string(), Color::from_rgba(220, 220, 220, 255));
        elements.insert("P".to_string(), Color::from_rgba(255, 128, 0, 255));
        
        let mut aa_groups = HashMap::new();
        aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(255, 150, 50, 255));
        aa_groups.insert("Polar".to_string(), Color::from_rgba(80, 220, 100, 255));
        aa_groups.insert("Positive".to_string(), Color::from_rgba(50, 120, 255, 255));
        aa_groups.insert("Negative".to_string(), Color::from_rgba(255, 60, 60, 255));
        aa_groups.insert("Glycine".to_string(), Color::from_rgba(230, 230, 230, 255));
        
        Self { elements, aa_groups }
    }
}

// Generate random but consistent color based on seed with improved vibrancy
pub fn seeded_random_color(seed: u32) -> Color {
    // Use different multipliers for each channel to get more variety
    let r = ((seed.wrapping_mul(2654435761)) % 200 + 55) as u8;
    let g = ((seed.wrapping_mul(1597463007)) % 200 + 55) as u8;
    let b = ((seed.wrapping_mul(3812015801)) % 200 + 55) as u8;
    // Ensure colors are vibrant (range 55-255 instead of 0-255)
    Color::from_rgba(r, g, b, 255)
}
