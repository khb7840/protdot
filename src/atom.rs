use macroquad::prelude::*;
use crate::color_maps::ColorMaps;

#[derive(Debug, Clone)]
pub struct Atom {
    pub atom_name: String,
    pub element: String,
    pub residue: String,
    pub residue_num: i32,
    pub position: Vec3,
    pub radius: f32,
}

impl Atom {
    pub fn get_color_by_element(element: &str, color_maps: &ColorMaps) -> Color {
        color_maps.elements.get(element)
            .copied()
            .unwrap_or(Color::from_rgba(255, 105, 180, 255))
    }
    
    pub fn get_color_by_amino_acid_group(residue: &str, color_maps: &ColorMaps) -> Color {
        let group = match residue {
            "ALA" | "VAL" | "LEU" | "ILE" | "MET" | "PHE" | "TRP" | "PRO" => "Hydrophobic",
            "SER" | "THR" | "CYS" | "TYR" | "ASN" | "GLN" => "Polar",
            "LYS" | "ARG" | "HIS" => "Positive",
            "ASP" | "GLU" => "Negative",
            "GLY" => "Glycine",
            _ => return Color::from_rgba(180, 180, 180, 255),
        };
        color_maps.aa_groups.get(group)
            .copied()
            .unwrap_or(Color::from_rgba(180, 180, 180, 255))
    }
    
    pub fn get_color_by_amino_acid_type(residue: &str) -> Color {
        match residue {
            "ALA" => Color::from_rgba(120, 180, 255, 255),  // Sky blue
            "ARG" => Color::from_rgba(255, 50, 100, 255),   // Bright pink-red
            "ASN" => Color::from_rgba(0, 220, 180, 255),    // Turquoise
            "ASP" => Color::from_rgba(255, 80, 50, 255),    // Coral red
            "CYS" => Color::from_rgba(255, 230, 0, 255),    // Pure yellow
            "GLN" => Color::from_rgba(80, 200, 200, 255),   // Aqua
            "GLU" => Color::from_rgba(255, 120, 0, 255),    // Pure orange
            "GLY" => Color::from_rgba(240, 240, 240, 255),  // Almost white
            "HIS" => Color::from_rgba(100, 150, 255, 255),  // Medium blue
            "ILE" => Color::from_rgba(50, 200, 50, 255),    // Bright green
            "LEU" => Color::from_rgba(150, 255, 50, 255),   // Yellow-green
            "LYS" => Color::from_rgba(0, 100, 255, 255),    // Deep blue
            "MET" => Color::from_rgba(255, 200, 50, 255),   // Golden
            "PHE" => Color::from_rgba(150, 80, 220, 255),   // Purple
            "PRO" => Color::from_rgba(255, 170, 120, 255),  // Salmon
            "SER" => Color::from_rgba(255, 150, 200, 255),  // Light pink
            "THR" => Color::from_rgba(255, 120, 180, 255),  // Rose pink
            "TRP" => Color::from_rgba(200, 50, 200, 255),   // Magenta
            "TYR" => Color::from_rgba(180, 120, 255, 255),  // Lavender
            "VAL" => Color::from_rgba(0, 180, 120, 255),    // Sea green
            _ => Color::from_rgba(200, 180, 150, 255),
        }
    }
    
    pub fn get_radius(element: &str) -> f32 {
        match element {
            "C" => 0.77,
            "O" => 0.73,
            "N" => 0.75,
            "S" => 1.02,
            "H" => 0.37,
            _ => 0.5,
        }
    }
}
