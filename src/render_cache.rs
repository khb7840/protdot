use macroquad::prelude::*;
use std::collections::HashMap;
use crate::atom::Atom;
use crate::color_maps::ColorMaps;
use crate::types::ColorScheme;

// Cache for legacy fallback rendering (transparency)
pub struct RenderCache {
    pub atom_data: Vec<(Vec3, f32, Color)>,
    pub residue_map: HashMap<i32, Vec<usize>>,
    pub residue_data: Vec<(i32, Vec3, f32, Color)>,
}

impl RenderCache {
    pub fn new() -> Self {
        Self {
            atom_data: Vec::with_capacity(65536),
            residue_map: HashMap::with_capacity(3200),
            residue_data: Vec::with_capacity(3200),
        }
    }

    pub fn clear(&mut self) {
        self.atom_data.clear();
        self.residue_map.clear();
        self.residue_data.clear();
    }
}

// Optimized batched renderer
pub struct MolecularRenderer {
    atom_meshes: Vec<Mesh>,
    residue_meshes: Vec<Mesh>,
    residue_proxies: Vec<(Vec3, f32, i32, usize)>,
    last_color_scheme: Option<ColorScheme>,
    last_radius_scale: f32,
    is_dirty: bool,
    cache: RenderCache,
    // Mesh quality and batching configuration
    pub rings: usize,
    pub slices: usize,
    pub max_vertices_per_chunk: usize,
}

impl MolecularRenderer {
    pub fn new(atoms: &[Atom]) -> Self {
        let residue_proxies = precompute_residues(atoms);
        
        Self {
            atom_meshes: vec![],
            residue_meshes: vec![],
            residue_proxies,
            last_color_scheme: None,
            last_radius_scale: -1.0,
            is_dirty: true,
            cache: RenderCache::new(),
            rings: 10,  // Default: smooth spheres
            slices: 10,
            max_vertices_per_chunk: 1000,  // ~16 spheres at 10x10 quality
        }
    }

    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }

    pub fn cache_mut(&mut self) -> &mut RenderCache {
        &mut self.cache
    }

    pub fn update_meshes(&mut self, atoms: &[Atom], color_scheme: ColorScheme, color_maps: &ColorMaps, radius_scale: f32) {
        if !self.is_dirty 
           && self.last_color_scheme == Some(color_scheme) 
           && (self.last_radius_scale - radius_scale).abs() < 0.001 {
            return;
        }

        let min_res = atoms.iter().map(|a| a.residue_num).min().unwrap_or(0) as f32;
        let max_res = atoms.iter().map(|a| a.residue_num).max().unwrap_or(1) as f32;

        self.atom_meshes = build_batched_meshes(
            atoms.iter().map(|a| (a.position, a.radius, a)).collect(),
            color_scheme,
            color_maps,
            radius_scale,
            min_res,
            max_res,
            self.rings,
            self.slices,
            self.max_vertices_per_chunk,
        );

        let residue_data: Vec<_> = self.residue_proxies.iter()
            .map(|(pos, rad, _res_num, atom_idx)| (*pos, *rad, &atoms[*atom_idx]))
            .collect();
        
        self.residue_meshes = build_batched_meshes(
            residue_data,
            color_scheme,
            color_maps,
            radius_scale,
            min_res,
            max_res,
            self.rings,
            self.slices,
            self.max_vertices_per_chunk,
        );

        self.last_color_scheme = Some(color_scheme);
        self.last_radius_scale = radius_scale;
        self.is_dirty = false;
    }

    pub fn atom_meshes(&self) -> &[Mesh] {
        &self.atom_meshes
    }

    pub fn residue_meshes(&self) -> &[Mesh] {
        &self.residue_meshes
    }
}

fn precompute_residues(atoms: &[Atom]) -> Vec<(Vec3, f32, i32, usize)> {
    let mut temp_map: HashMap<i32, Vec<usize>> = HashMap::new();
    
    for (i, atom) in atoms.iter().enumerate() {
        temp_map.entry(atom.residue_num).or_default().push(i);
    }

    temp_map.into_iter().map(|(res_num, indices)| {
        let center = indices.iter()
            .fold(vec3(0.0, 0.0, 0.0), |acc, &i| acc + atoms[i].position) 
            / indices.len() as f32;
        
        let radius = indices.iter()
            .map(|&i| (atoms[i].position - center).length() + atoms[i].radius)
            .fold(0.0f32, |max, d| max.max(d));

        (center, radius, res_num, indices[0])
    }).collect()
}

fn build_batched_meshes(
    items: Vec<(Vec3, f32, &Atom)>, 
    color_scheme: ColorScheme, 
    color_maps: &ColorMaps, 
    scale: f32,
    min_res: f32,
    max_res: f32,
    rings: usize,
    slices: usize,
    max_vertices_per_chunk: usize,
) -> Vec<Mesh> {
    let verts_per_sphere = (rings + 1) * (slices + 1);
    let max_spheres_per_chunk = max_vertices_per_chunk / verts_per_sphere;
    
    let mut meshes = Vec::new();
    
    for chunk in items.chunks(max_spheres_per_chunk) {
        let mut vertices = Vec::with_capacity(chunk.len() * verts_per_sphere);
        let mut indices = Vec::with_capacity(chunk.len() * rings * slices * 6);

        for (pos, radius, atom) in chunk {
            let r = radius * scale;
            let color = crate::ui::rendering::get_color(color_scheme, atom, atom.residue_num, min_res, max_res, color_maps);
            
            let start_index = vertices.len() as u16;

            for i in 0..=rings {
                let v = i as f32 / rings as f32;
                let phi = v * std::f32::consts::PI;
                
                for j in 0..=slices {
                    let u = j as f32 / slices as f32;
                    let theta = u * std::f32::consts::PI * 2.0;

                    let x = r * phi.sin() * theta.cos();
                    let y = r * phi.cos();
                    let z = r * phi.sin() * theta.sin();

                    let normal_vec3 = vec3(x, y, z).normalize();
                    let normal = vec4(normal_vec3.x, normal_vec3.y, normal_vec3.z, 0.0);

                    vertices.push(Vertex {
                        position: vec3(pos.x + x, pos.y + y, pos.z + z),
                        uv: vec2(u, v),
                        color: [
                            (color.r * 255.0) as u8,
                            (color.g * 255.0) as u8,
                            (color.b * 255.0) as u8,
                            (color.a * 255.0) as u8,
                        ],
                        normal,
                    });
                }
            }

            for i in 0..rings {
                for j in 0..slices {
                    let next = j + 1;
                    let next_ring = i + 1;
                    let stride = slices + 1;

                    let v0 = start_index + (i * stride + j) as u16;
                    let v1 = start_index + (i * stride + next) as u16;
                    let v2 = start_index + (next_ring * stride + j) as u16;
                    let v3 = start_index + (next_ring * stride + next) as u16;

                    indices.extend_from_slice(&[v0, v2, v1, v1, v2, v3]);
                }
            }
        }
        
        meshes.push(Mesh { vertices, indices, texture: None });
    }
    
    meshes
}
