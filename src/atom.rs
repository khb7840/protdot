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
        color_maps.elements().get(element)
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
        color_maps.aa_groups().get(group)
            .copied()
            .unwrap_or(Color::from_rgba(180, 180, 180, 255))
    }
    
    pub fn get_color_by_amino_acid_type(residue: &str, color_maps: &ColorMaps) -> Color {
        color_maps.aa_types().get(residue)
            .copied()
            .unwrap_or(Color::from_rgba(200, 180, 150, 255))
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
