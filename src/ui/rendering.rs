use macroquad::prelude::*;
use std::collections::HashMap;
use crate::atom::Atom;
use crate::color_maps::{ColorMaps, seeded_random_color};
use crate::types::{ColorScheme, RenderMode};

pub fn render_atoms(
    atoms: &[Atom],
    cam: &Camera3D,
    color_scheme: ColorScheme,
    render_mode: RenderMode,
    color_maps: &ColorMaps,
    radius_scale: f32,
    alpha: f32,
) {
    match render_mode {
        RenderMode::PerResidue => render_per_residue(atoms, cam, color_scheme, color_maps, radius_scale, alpha),
        RenderMode::PerAtom => render_per_atom(atoms, cam, color_scheme, color_maps, radius_scale, alpha),
    }
}

fn render_per_residue(
    atoms: &[Atom],
    cam: &Camera3D,
    color_scheme: ColorScheme,
    color_maps: &ColorMaps,
    radius_scale: f32,
    alpha: f32,
) {
    let mut residues: HashMap<i32, Vec<&Atom>> = HashMap::new();
    
    for atom in atoms {
        residues.entry(atom.residue_num).or_insert_with(Vec::new).push(atom);
    }
    
    let min_res = residues.keys().min().copied().unwrap_or(0) as f32;
    let max_res = residues.keys().max().copied().unwrap_or(1) as f32;
    
    let mut residue_data: Vec<(i32, Vec3, f32, Color)> = Vec::new();
    
    for (res_num, res_atoms) in residues.iter() {
        if res_atoms.is_empty() {
            continue;
        }
        
        let center: Vec3 = res_atoms.iter()
            .fold(vec3(0.0, 0.0, 0.0), |acc, a| acc + a.position) 
            / res_atoms.len() as f32;
        
        let sphere_radius = res_atoms.iter()
            .map(|a| (a.position - center).length() + a.radius)
            .fold(0.0f32, |max, dist| max.max(dist));
        
        let color = get_color(color_scheme, res_atoms[0], *res_num, min_res, max_res, color_maps);
        
        residue_data.push((*res_num, center, sphere_radius, color));
    }
    
    if alpha < 1.0 {
        residue_data.sort_by(|a, b| {
            let dist_a = (a.1 - cam.position).length();
            let dist_b = (b.1 - cam.position).length();
            dist_b.partial_cmp(&dist_a).unwrap()
        });
    }
    
    for (_res_num, center, sphere_radius, color) in residue_data {
        let scaled_radius = sphere_radius * radius_scale;
        let color_with_alpha = Color::new(color.r, color.g, color.b, alpha);
        draw_sphere(center, scaled_radius, None, color_with_alpha);
    }
}

fn render_per_atom(
    atoms: &[Atom],
    cam: &Camera3D,
    color_scheme: ColorScheme,
    color_maps: &ColorMaps,
    radius_scale: f32,
    alpha: f32,
) {
    let min_res = atoms.iter().map(|a| a.residue_num).min().unwrap_or(0) as f32;
    let max_res = atoms.iter().map(|a| a.residue_num).max().unwrap_or(1) as f32;
    
    let mut atom_data: Vec<(&Atom, Color)> = atoms.iter().map(|atom| {
        let color = get_color(color_scheme, atom, atom.residue_num, min_res, max_res, color_maps);
        (atom, color)
    }).collect();
    
    if alpha < 1.0 {
        atom_data.sort_by(|a, b| {
            let dist_a = (a.0.position - cam.position).length();
            let dist_b = (b.0.position - cam.position).length();
            dist_b.partial_cmp(&dist_a).unwrap()
        });
    }
    
    for (atom, color) in atom_data {
        let scaled_radius = atom.radius * radius_scale;
        let color_with_alpha = Color::new(color.r, color.g, color.b, alpha);
        draw_sphere(atom.position, scaled_radius, None, color_with_alpha);
    }
}

fn get_color(
    color_scheme: ColorScheme,
    atom: &Atom,
    res_num: i32,
    min_res: f32,
    max_res: f32,
    color_maps: &ColorMaps,
) -> Color {
    match color_scheme {
        ColorScheme::ByElement => Atom::get_color_by_element(&atom.element, color_maps),
        ColorScheme::ByAminoAcidGroup => Atom::get_color_by_amino_acid_group(&atom.residue, color_maps),
        ColorScheme::ByAminoAcidType => Atom::get_color_by_amino_acid_type(&atom.residue),
        ColorScheme::NToCGradient => {
            let t = (res_num as f32 - min_res) / (max_res - min_res).max(1.0);
            Color::from_rgba(
                (50.0 + t * 205.0) as u8,
                (100.0 + t * 155.0) as u8,
                (255.0 - t * 155.0) as u8,
                255
            )
        },
        ColorScheme::RandomChain => seeded_random_color(res_num as u32),
    }
}
