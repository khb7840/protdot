use macroquad::prelude::*;

// Default PDB data embedded in the binary
const DEFAULT_PDB_DATA: &str = include_str!("../data/1G2F.pdb");

// 1. Define our Atom
#[derive(Debug, Clone)]
struct Atom {
    element: String,
    residue: String,
    position: Vec3,
    radius: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ColorMode {
    ByElement,
    ByResidue,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RenderMode {
    Spheres,
    BackboneRibbon,
}

impl Atom {
    // Modern aesthetic CPK coloring scheme
    fn get_color_by_element(element: &str) -> Color {
        match element {
            "C" => Color::from_rgba(64, 64, 64, 255),     // Dark gray carbon
            "O" => Color::from_rgba(240, 80, 80, 255),    // Soft red oxygen
            "N" => Color::from_rgba(80, 120, 240, 255),   // Soft blue nitrogen
            "S" => Color::from_rgba(255, 200, 50, 255),   // Golden sulfur
            "H" => Color::from_rgba(220, 220, 220, 255),  // Light gray hydrogen
            "P" => Color::from_rgba(255, 128, 0, 255),    // Orange phosphorus
            _ => Color::from_rgba(255, 105, 180, 255),    // Pink unknown
        }
    }
    
    fn get_color_by_residue(residue: &str) -> Color {
        match residue {
            // Hydrophobic (beige/tan shades)
            "ALA" | "VAL" | "LEU" | "ILE" | "MET" | "PHE" | "TRP" | "PRO" => 
                Color::from_rgba(210, 180, 140, 255),
            // Polar (green shades)
            "SER" | "THR" | "CYS" | "TYR" | "ASN" | "GLN" => 
                Color::from_rgba(144, 238, 144, 255),
            // Positive (blue shades)
            "LYS" | "ARG" | "HIS" => 
                Color::from_rgba(100, 149, 237, 255),
            // Negative (red shades)
            "ASP" | "GLU" => 
                Color::from_rgba(240, 128, 128, 255),
            // Glycine (white)
            "GLY" => Color::from_rgba(255, 255, 255, 255),
            _ => Color::from_rgba(192, 192, 192, 255), // Default gray
        }
    }
    
    fn get_radius(element: &str) -> f32 {
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

// 2. The Parsing Logic
fn load_pdb(content: &str) -> Vec<Atom> {
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
                // Infer element from Atom Name (e.g., "CA" -> "C")
                let atom_name = parts[2]; 
                let element = atom_name.chars().next().unwrap_or('?').to_string();
                
                // Get residue name (usually at index 3)
                let residue = if parts.len() > 3 {
                    parts[3].to_string()
                } else {
                    "UNK".to_string()
                };

                atoms.push(Atom {
                    element: element.clone(),
                    residue,
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

#[macroquad::main("Protein Viewer")]
async fn main() {
    // Load the protein from embedded data
    let atoms = load_pdb(DEFAULT_PDB_DATA);
    
    // Setup Camera
    let mut cam = Camera3D {
        position: vec3(0., 0., 40.), // Pull back to see the protein
        target: vec3(0., 0., 0.),
        up: vec3(0., 1., 0.),
        ..Default::default()
    };

    let mut angle_x = 0.0_f32;
    let mut angle_y = 0.0_f32;
    let mut velocity_x = 0.0_f32; // Rotation velocity for smooth motion
    let mut velocity_y = 0.0_f32;
    let mut prev_mouse_pos = mouse_position();
    let mut radius = 40.0_f32; // Camera distance (for zoom)
    let mut pan_offset = vec3(0.0, 0.0, 0.0); // Translation offset
    
    // Visualization options
    let mut color_mode = ColorMode::ByElement;
    let mut render_mode = RenderMode::Spheres;

    loop {
        // Keyboard input for mode switching
        if is_key_pressed(KeyCode::C) {
            color_mode = match color_mode {
                ColorMode::ByElement => ColorMode::ByResidue,
                ColorMode::ByResidue => ColorMode::ByElement,
            };
        }
        if is_key_pressed(KeyCode::B) {
            render_mode = match render_mode {
                RenderMode::Spheres => RenderMode::BackboneRibbon,
                RenderMode::BackboneRibbon => RenderMode::Spheres,
            };
        }
        
        let should_export = is_key_pressed(KeyCode::E);
        let current_mouse_pos = mouse_position();
        let delta_x = current_mouse_pos.0 - prev_mouse_pos.0;
        let delta_y = current_mouse_pos.1 - prev_mouse_pos.1;
        
        // Input: Left Mouse Drag to Rotate
        if is_mouse_button_down(MouseButton::Left) {
            velocity_x = -delta_x * 0.01;
            velocity_y = delta_y * 0.01;
        } else {
            // Apply velocity for smooth rotation
            velocity_x *= 0.9; // Damping factor
            velocity_y *= 0.9;
        }
        
        // Apply velocities to angles
        angle_x += velocity_x;
        angle_y += velocity_y;
        
        // Input: Middle Mouse Drag to Translate (Pan)
        if is_mouse_button_down(MouseButton::Middle) {
            // Pan relative to camera orientation
            let pan_speed = 0.05;
            pan_offset.x -= delta_x * pan_speed;
            pan_offset.y += delta_y * pan_speed;
        }
        
        prev_mouse_pos = current_mouse_pos;
        
        // Input: Mouse Scroll to Zoom
        let (_wheel_x, wheel_y) = mouse_wheel();
        radius -= wheel_y * 2.0; // Zoom speed
        radius = radius.clamp(5.0, 200.0); // Limit zoom range
        
        // Orbital Math: Rotate camera around the center
        cam.position.x = radius * angle_y.cos() * angle_x.sin();
        cam.position.y = radius * angle_y.sin();
        cam.position.z = radius * angle_y.cos() * angle_x.cos();
        cam.target = pan_offset;

        clear_background(LIGHTGRAY);

        set_camera(&cam);

        // Render based on mode
        if render_mode == RenderMode::BackboneRibbon {
            // Extract CA (alpha carbon) atoms for backbone trace
            let ca_atoms: Vec<&Atom> = atoms.iter()
                .filter(|a| a.element == "C" || a.element == "N" || a.element == "O")
                .collect();
            
            // Draw cylinders connecting sequential atoms for surface visualization
            let cylinder_radius = 0.5;
            for i in 0..(ca_atoms.len() - 1) {
                let start = ca_atoms[i].position;
                let end = ca_atoms[i + 1].position;
                let dist = start.distance(end);
                
                // Only connect nearby atoms (likely in same chain)
                if dist < 4.0 {
                    let color = match color_mode {
                        ColorMode::ByElement => Atom::get_color_by_element(&ca_atoms[i].element),
                        ColorMode::ByResidue => Atom::get_color_by_residue(&ca_atoms[i].residue),
                    };
                    
                    // Draw thick line to simulate cylinder
                    let steps = (dist * 5.0) as usize;
                    for step in 0..steps {
                        let t = step as f32 / steps as f32;
                        let pos = start.lerp(end, t);
                        draw_sphere(pos, cylinder_radius, None, color);
                    }
                }
            }
        } else {
            // Render Atoms as spheres
            for atom in &atoms {
                let color = match color_mode {
                    ColorMode::ByElement => Atom::get_color_by_element(&atom.element),
                    ColorMode::ByResidue => Atom::get_color_by_residue(&atom.residue),
                };
                draw_sphere(atom.position, atom.radius, None, color);
            }
        }

        set_default_camera();
        
        // GUI / Stats
        draw_text(&format!("Atoms: {}", atoms.len()), 10.0, 20.0, 30.0, BLACK);
        draw_text("Left drag: rotate", 10.0, 50.0, 20.0, DARKGRAY);
        draw_text("Middle drag: pan", 10.0, 70.0, 20.0, DARKGRAY);
        draw_text("Scroll: zoom", 10.0, 90.0, 20.0, DARKGRAY);
        
        let color_text = match color_mode {
            ColorMode::ByElement => "C: Color by Element",
            ColorMode::ByResidue => "C: Color by Residue",
        };
        let render_text = match render_mode {
            RenderMode::Spheres => "B: Backbone Ribbon",
            RenderMode::BackboneRibbon => "B: Sphere Mode",
        };
        draw_text(color_text, 10.0, 120.0, 20.0, DARKGREEN);
        draw_text(render_text, 10.0, 140.0, 20.0, DARKGREEN);
        draw_text("E: Export PNG", 10.0, 160.0, 20.0, DARKGREEN);

        // Export PNG before frame swap (capture current rendered frame)
        if should_export {
            let image = get_screen_data();
            image.export_png("protein_export.png");
            println!("Exported to protein_export.png");
        }

        next_frame().await;
    }
}