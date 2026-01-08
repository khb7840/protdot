use macroquad::prelude::*;
use crate::atom::Atom;

// For native builds: load from file path
#[cfg(not(target_arch = "wasm32"))]
pub fn load_pdb(path: &str) -> Vec<Atom> {
    use std::fs;
    let content = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Could not read file: {}", path));
    parse_pdb_content(&content)
}

// For WASM builds: load from string content
#[cfg(target_arch = "wasm32")]
pub fn load_pdb(content: &str) -> Vec<Atom> {
    parse_pdb_content(content)
}

// Common parsing logic
fn parse_pdb_content(content: &str) -> Vec<Atom> {
    let mut atoms = Vec::new();

    for line in content.lines() {
        if line.starts_with("ATOM") {
            // PDB files are fixed-width, but splitting by whitespace 
            // is often "good enough" for a quick hack.
            // Standard PDB format:
            // Col 0: "ATOM"
            // Col 2: Atom Name (e.g., CA, CB)
            // Col 6-8: X, Y, Z coordinates usually sit around these indices in a split
            
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            // Safety check for malformed lines
            if parts.len() < 8 { continue; }

            // Parse coordinates (Usually indices 6, 7, 8 in whitespace split)
            // Note: In strict PDB parsing, we would slice indices (30..38), etc.
            if let (Ok(x), Ok(y), Ok(z)) = (
                parts[6].parse::<f32>(),
                parts[7].parse::<f32>(),
                parts[8].parse::<f32>(),
            ) {
                // Get atom name (e.g., CA, CB, N, O)
                let atom_name = parts[2].to_string();
                
                // Infer element from Atom Name (e.g., "CA" -> "C")
                let element = atom_name.chars().next().unwrap_or('?').to_string();
                
                // Get residue name (usually at index 3)
                let residue = if parts.len() > 3 {
                    parts[3].to_string()
                } else {
                    "UNK".to_string()
                };
                
                // Get residue number (usually at index 5)
                let residue_num = if parts.len() > 5 {
                    parts[5].parse::<i32>().unwrap_or(0)
                } else {
                    0
                };

                atoms.push(Atom {
                    atom_name,
                    element: element.clone(),
                    residue,
                    residue_num,
                    position: vec3(x, y, z),
                    radius: Atom::get_radius(&element),
                });
            }
        }
    }
    
    // CENTER THE PROTEIN
    // 3D cameras rotate around (0,0,0). If the protein is at (100, 100, 100),
    // it will swing wildly out of view. We must center it.
    let count = atoms.len() as f32;
    let center: Vec3 = atoms.iter().fold(vec3(0.,0.,0.), |acc, a| acc + a.position) / count;
    
    for atom in &mut atoms {
        atom.position -= center;
    }

    atoms
}
