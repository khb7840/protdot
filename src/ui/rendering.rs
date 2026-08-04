use macroquad::prelude::*;
use crate::atom::Atom;
use crate::color_maps::{ColorMaps, color_for_chain};
use crate::types::{ColorScheme, RenderMode};
use crate::render_cache::{RenderCache, MolecularRenderer};

// New optimized render function using batched meshes
pub fn render_atoms_optimized(
    renderer: &mut MolecularRenderer,
    atoms: &[Atom],
    _cam: &Camera3D,
    color_scheme: ColorScheme,
    render_mode: RenderMode,
    color_maps: &ColorMaps,
    radius_scale: f32,
    rotation: Quat,
    translation: Vec3,
    _camera_axes: (Vec3, Vec3, Vec3),
) {
    // Update meshes if needed
    renderer.update_meshes(atoms, color_scheme, color_maps, radius_scale);

    // Batched mesh rendering (works with transparency but no depth sorting)
    // Note: Alpha < 1.0 will have visual artifacts (incorrect blend order) but renders fast
    unsafe {
        let mut transform = Mat4::from_translation(translation);
        let (axis, angle) = rotation.to_axis_angle();
        transform = transform * Mat4::from_axis_angle(axis, angle);
        
        get_internal_gl().quad_gl.push_model_matrix(transform);
    }
    
    // Render appropriate mesh (transparency handled via vertex alpha)
    match render_mode {
        RenderMode::PerAtom => {
            for mesh in renderer.atom_meshes() {
                draw_mesh(mesh);
            }
        }
        RenderMode::PerResidue => {
            for mesh in renderer.residue_meshes() {
                draw_mesh(mesh);
            }
        }
    }

    unsafe {
        get_internal_gl().quad_gl.pop_model_matrix();
    }
}

// Legacy render function for backwards compatibility
pub fn render_atoms(
    atoms: &[Atom],
    cam: &Camera3D,
    color_scheme: ColorScheme,
    render_mode: RenderMode,
    color_maps: &ColorMaps,
    radius_scale: f32,
    alpha: f32,
    rotation: Quat,
    translation: Vec3,
    camera_axes: (Vec3, Vec3, Vec3),
    cache: &mut RenderCache,
) {
    cache.clear();
    match render_mode {
        RenderMode::PerResidue => render_per_residue(atoms, cam, color_scheme, color_maps, radius_scale, alpha, rotation, translation, camera_axes, cache),
        RenderMode::PerAtom => render_per_atom(atoms, cam, color_scheme, color_maps, radius_scale, alpha, rotation, translation, camera_axes, cache),
    }
}

// Apply rotation transformation to a position
fn rotate_position(pos: Vec3, rotation: Quat, translation: Vec3, _camera_axes: (Vec3, Vec3, Vec3)) -> Vec3 {
    // Apply accumulated rotation and then translation
    // In gl-rs/glam/macroquad, rot * vec applies the rotation to the vector
    (rotation * pos) + translation
}

// Rotate a point around an arbitrary axis using Rodrigues' rotation formula
// Keep strictly for non-standard use cases if any, otherwise unused
fn rotate_around_axis(p: Vec3, axis: Vec3, angle: f32) -> Vec3 {
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    let axis = axis.normalize();
    
    p * cos_a + axis.cross(p) * sin_a + axis * axis.dot(p) * (1.0 - cos_a)
}

fn render_per_residue(
    atoms: &[Atom],
    cam: &Camera3D,
    color_scheme: ColorScheme,
    color_maps: &ColorMaps,
    radius_scale: f32,
    alpha: f32,
    rotation: Quat,
    translation: Vec3,
    camera_axes: (Vec3, Vec3, Vec3),
    cache: &mut RenderCache,
) {
    // Group atoms by residue using pre-allocated map
    for (idx, atom) in atoms.iter().enumerate() {
        cache.residue_map.entry(atom.residue_num)
            .or_insert_with(Vec::new)
            .push(idx);
    }
    
    let min_res = cache.residue_map.keys().min().copied().unwrap_or(0) as f32;
    let max_res = cache.residue_map.keys().max().copied().unwrap_or(1) as f32;
    
    // Calculate residue centers and radii using pre-allocated buffer
    for (res_num, atom_indices) in cache.residue_map.iter() {
        if atom_indices.is_empty() {
            continue;
        }
        
        let mut center = vec3(0.0, 0.0, 0.0);
        for &idx in atom_indices {
            center += atoms[idx].position;
        }
        center /= atom_indices.len() as f32;
        
        let rotated_center = rotate_position(center, rotation, translation, camera_axes);
        
        let mut sphere_radius = 0.0f32;
        for &idx in atom_indices {
            let dist = (atoms[idx].position - center).length() + atoms[idx].radius;
            sphere_radius = sphere_radius.max(dist);
        }
        
        let color = get_color(color_scheme, &atoms[atom_indices[0]], *res_num, min_res, max_res, color_maps);
        
        cache.residue_data.push((*res_num, rotated_center, sphere_radius, color));
    }
    
    if alpha < 1.0 {
        cache.residue_data.sort_unstable_by(|a, b| {
            let dist_a = (a.1 - cam.position).length_squared();
            let dist_b = (b.1 - cam.position).length_squared();
            dist_b.partial_cmp(&dist_a).unwrap_or(std::cmp::Ordering::Equal)
        });
    }
    
    for (_res_num, center, sphere_radius, color) in &cache.residue_data {
        let scaled_radius = sphere_radius * radius_scale;
        let color_with_alpha = Color::new(color.r, color.g, color.b, alpha);
        draw_sphere(*center, scaled_radius, None, color_with_alpha);
    }
}

fn render_per_atom(
    atoms: &[Atom],
    cam: &Camera3D,
    color_scheme: ColorScheme,
    color_maps: &ColorMaps,
    radius_scale: f32,
    alpha: f32,
    rotation: Quat,
    translation: Vec3,
    camera_axes: (Vec3, Vec3, Vec3),
    cache: &mut RenderCache,
) {
    let min_res = atoms.iter().map(|a| a.residue_num).min().unwrap_or(0) as f32;
    let max_res = atoms.iter().map(|a| a.residue_num).max().unwrap_or(1) as f32;
    
    // Fast path: when alpha == 1.0, no sorting needed, render directly without allocation
    if alpha >= 1.0 {
        for atom in atoms {
            let color = get_color(color_scheme, atom, atom.residue_num, min_res, max_res, color_maps);
            let rotated_pos = rotate_position(atom.position, rotation, translation, camera_axes);
            let scaled_radius = atom.radius * radius_scale;
            draw_sphere(rotated_pos, scaled_radius, None, color);
        }
        return;
    }
    
    // Transparent path: use bucketed rendering (much faster than sorting)
    // Find min/max distances for bucket range
    let mut min_dist_sq = f32::MAX;
    let mut max_dist_sq = 0.0f32;
    
    for atom in atoms {
        let rotated_pos = rotate_position(atom.position, rotation, translation, camera_axes);
        let dist_sq = (rotated_pos - cam.position).length_squared();
        min_dist_sq = min_dist_sq.min(dist_sq);
        max_dist_sq = max_dist_sq.max(dist_sq);
    }
        
    // Distribute atoms into distance buckets
    for atom in atoms {
        let color = get_color(color_scheme, atom, atom.residue_num, min_res, max_res, color_maps);
        let rotated_pos = rotate_position(atom.position, rotation, translation, camera_axes);
        cache.atom_data.push((rotated_pos, atom.radius, color));
    }
    
    
    for (pos, radius, color) in &cache.atom_data {
        let scaled_radius = radius * radius_scale;
        let color_with_alpha = Color::new(color.r, color.g, color.b, alpha);
        draw_sphere(*pos, scaled_radius, None, color_with_alpha);
    }
}


pub fn get_color(
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
        ColorScheme::RandomChain => color_for_chain(&atom.chain, color_maps),
        ColorScheme::Theme => {
            // Theme uses the same logic as ByElement but with themed colors
            Atom::get_color_by_element(&atom.element, color_maps)
        }
    }
}
