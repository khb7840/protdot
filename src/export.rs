use crate::atom::Atom;
use crate::types::{ColorScheme, RenderMode};
use crate::color_maps::{ColorMaps, seeded_random_color};
use macroquad::prelude::*;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    PNG,
    SVG,
}

/// Export atoms to SVG format (2D projection)
pub fn export_svg(
    atoms: &[Atom],
    filename: &str,
    camera: &Camera3D,
    color_scheme: ColorScheme,
    render_mode: RenderMode,
    color_maps: &ColorMaps,
    radius_scale: f32,
    screen_width: f32,
    screen_height: f32,
    rotation: Quat,
    translation: Vec3,
) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    
    // SVG header
    writeln!(file, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
    writeln!(file, r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#, 
        screen_width, screen_height, screen_width, screen_height)?;
    writeln!(file, r#"  <rect width="100%" height="100%" fill="white"/>"#)?;
    writeln!(file, r#"  <g id="protein">"#)?;
    
    let fovy = 45.0_f32.to_radians();
    let tan_half_fov = (fovy / 2.0).tan();
    
    match render_mode {
        RenderMode::PerAtom => {
            render_svg_per_atom(&mut file, atoms, camera, color_scheme, color_maps, radius_scale, screen_width, screen_height, rotation, translation, tan_half_fov)?;
        }
        RenderMode::PerResidue => {
            render_svg_per_residue(&mut file, atoms, camera, color_scheme, color_maps, radius_scale, screen_width, screen_height, rotation, translation, tan_half_fov)?;
        }
    }
    
    writeln!(file, r#"  </g>"#)?;
    writeln!(file, r#"</svg>"#)?;
    
    Ok(())
}

fn render_svg_per_atom(
    file: &mut File,
    atoms: &[Atom],
    camera: &Camera3D,
    color_scheme: ColorScheme,
    color_maps: &ColorMaps,
    radius_scale: f32,
    screen_width: f32,
    screen_height: f32,
    rotation: Quat,
    translation: Vec3,
    tan_half_fov: f32,
) -> std::io::Result<()> {
    let min_res = atoms.iter().map(|a| a.residue_num).min().unwrap_or(0) as f32;
    let max_res = atoms.iter().map(|a| a.residue_num).max().unwrap_or(1) as f32;
    
    let mut projected_atoms: Vec<_> = atoms.iter()
        .map(|atom| {
            let transformed_pos = apply_transform(atom.position, rotation, translation);
            let screen_pos = project_to_screen(transformed_pos, camera, screen_width, screen_height);
            let depth = (transformed_pos - camera.position).length();
            (atom, screen_pos, depth)
        })
        .collect();
    
    // Sort back to front for proper occlusion
    projected_atoms.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    
    for (atom, screen_pos, depth) in projected_atoms.iter() {
        let color = get_color(color_scheme, atom, atom.residue_num, min_res, max_res, color_maps);
        
        let distance = *depth;
        let scaled_radius = atom.radius * radius_scale;
        
        let radius = if distance > 0.1 {
            (scaled_radius / (distance * tan_half_fov)) * screen_height / 2.0
        } else {
            scaled_radius * 50.0
        };
        
        writeln!(file, r#"    <circle cx="{:.2}" cy="{:.2}" r="{:.2}" fill="rgb({},{},{})" />"#,
            screen_pos.x, screen_pos.y, radius,
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8
        )?;
    }
    
    Ok(())
}

fn render_svg_per_residue(
    file: &mut File,
    atoms: &[Atom],
    camera: &Camera3D,
    color_scheme: ColorScheme,
    color_maps: &ColorMaps,
    radius_scale: f32,
    screen_width: f32,
    screen_height: f32,
    rotation: Quat,
    translation: Vec3,
    tan_half_fov: f32,
) -> std::io::Result<()> {
    let mut residues: HashMap<i32, Vec<&Atom>> = HashMap::new();
    
    for atom in atoms {
        residues.entry(atom.residue_num).or_insert_with(Vec::new).push(atom);
    }
    
    let min_res = residues.keys().min().copied().unwrap_or(0) as f32;
    let max_res = residues.keys().max().copied().unwrap_or(1) as f32;
    
    let mut residue_data: Vec<(i32, Vec2, f32, f32, Color)> = Vec::new();
    
    for (res_num, res_atoms) in residues.iter() {
        if res_atoms.is_empty() {
            continue;
        }
        
        let center: Vec3 = res_atoms.iter()
            .fold(vec3(0.0, 0.0, 0.0), |acc, a| acc + a.position) 
            / res_atoms.len() as f32;
        
        let transformed_center = apply_transform(center, rotation, translation);
        let screen_pos = project_to_screen(transformed_center, camera, screen_width, screen_height);
        let depth = (transformed_center - camera.position).length();
        
        let sphere_radius = res_atoms.iter()
            .map(|a| (a.position - center).length() + a.radius)
            .fold(0.0f32, |max, dist| max.max(dist));
        
        let color = get_color(color_scheme, res_atoms[0], *res_num, min_res, max_res, color_maps);
        
        residue_data.push((*res_num, screen_pos, depth, sphere_radius, color));
    }
    
    // Sort back to front for proper occlusion
    residue_data.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    
    for (_res_num, screen_pos, depth, sphere_radius, color) in residue_data {
        let scaled_radius = sphere_radius * radius_scale;
        
        let radius = if depth > 0.1 {
            (scaled_radius / (depth * tan_half_fov)) * screen_height / 2.0
        } else {
            scaled_radius * 50.0
        };
        
        writeln!(file, r#"    <circle cx="{:.2}" cy="{:.2}" r="{:.2}" fill="rgb({},{},{})" />"#,
            screen_pos.x, screen_pos.y, radius,
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8
        )?;
    }
    
    Ok(())
}

fn apply_transform(pos: Vec3, rotation: Quat, translation: Vec3) -> Vec3 {
    (rotation * pos) + translation
}

fn project_to_screen(
    pos: Vec3,
    camera: &Camera3D,
    screen_width: f32,
    screen_height: f32,
) -> Vec2 {
    // Match macroquad's default perspective projection
    // Default FOV is 45 degrees in macroquad
    let fovy = 45.0_f32.to_radians();
    let aspect = screen_width / screen_height;
    
    let view_dir = (camera.target - camera.position).normalize();
    let right = view_dir.cross(camera.up).normalize();
    let up = right.cross(view_dir).normalize();
    
    let relative_pos = pos - camera.position;
    let distance = relative_pos.dot(view_dir);
    
    if distance < 0.1 {
        return vec2(screen_width / 2.0, screen_height / 2.0);
    }
    
    // Calculate screen space coordinates using perspective projection
    let x = relative_pos.dot(right);
    let y = relative_pos.dot(up);
    
    // Apply perspective division with FOV
    let tan_half_fov = (fovy / 2.0).tan();
    let proj_x = x / (distance * tan_half_fov * aspect);
    let proj_y = y / (distance * tan_half_fov);
    
    vec2(
        screen_width / 2.0 + proj_x * screen_width / 2.0,
        screen_height / 2.0 - proj_y * screen_height / 2.0
    )
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
        ColorScheme::ByAminoAcidType => Atom::get_color_by_amino_acid_type(&atom.residue, color_maps),
        ColorScheme::NToCGradient => {
            let t = (res_num as f32 - min_res) / (max_res - min_res).max(1.0);
            let start = color_maps.gradient_start();
            let end = color_maps.gradient_end();
            Color::from_rgba(
                (start.r * (1.0 - t) + end.r * t * 255.0) as u8,
                (start.g * (1.0 - t) + end.g * t * 255.0) as u8,
                (start.b * (1.0 - t) + end.b * t * 255.0) as u8,
                255
            )
        },
        ColorScheme::RandomChain => seeded_random_color(res_num as u32, color_maps),
        ColorScheme::Theme => {
            // Theme uses the same logic as ByElement but with themed colors
            Atom::get_color_by_element(&atom.element, color_maps)
        }
    }
}
