use macroquad::prelude::*;

pub fn calculate_bounding_box(atoms: &[crate::atom::Atom]) -> (Vec3, Vec3) {
    if atoms.is_empty() {
        return (vec3(0., 0., 0.), vec3(0., 0., 0.));
    }
    
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut min_z = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut max_z = f32::NEG_INFINITY;
    
    for atom in atoms {
        min_x = min_x.min(atom.position.x);
        min_y = min_y.min(atom.position.y);
        min_z = min_z.min(atom.position.z);
        max_x = max_x.max(atom.position.x);
        max_y = max_y.max(atom.position.y);
        max_z = max_z.max(atom.position.z);
    }
    
    (vec3(min_x, min_y, min_z), vec3(max_x, max_y, max_z))
}

pub fn calculate_initial_radius(min_bound: Vec3, max_bound: Vec3) -> f32 {
    let structure_size = (max_bound - min_bound).length();
    (structure_size * 1.1).max(40.0) // 1.1x for a slightly farther view, minimum 40
}
