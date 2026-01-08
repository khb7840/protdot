use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Theme {
    pub name: String,
    pub elements: HashMap<String, Color>,
    pub aa_groups: HashMap<String, Color>,
    pub aa_types: HashMap<String, Color>,
    pub gradient_start: Color,
    pub gradient_end: Color,
}

pub fn create_builtin_themes() -> Vec<Theme> {
    let mut themes = Vec::new();
    
    // Default theme
    let mut default_elements = HashMap::new();
    default_elements.insert("C".to_string(), Color::from_rgba(64, 64, 64, 255));
    default_elements.insert("O".to_string(), Color::from_rgba(240, 80, 80, 255));
    default_elements.insert("N".to_string(), Color::from_rgba(80, 120, 240, 255));
    default_elements.insert("S".to_string(), Color::from_rgba(255, 200, 50, 255));
    default_elements.insert("H".to_string(), Color::from_rgba(220, 220, 220, 255));
    default_elements.insert("P".to_string(), Color::from_rgba(255, 128, 0, 255));
    
    let mut default_aa_groups = HashMap::new();
    default_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(255, 150, 50, 255));
    default_aa_groups.insert("Polar".to_string(), Color::from_rgba(80, 220, 100, 255));
    default_aa_groups.insert("Positive".to_string(), Color::from_rgba(50, 120, 255, 255));
    default_aa_groups.insert("Negative".to_string(), Color::from_rgba(255, 60, 60, 255));
    default_aa_groups.insert("Glycine".to_string(), Color::from_rgba(230, 230, 230, 255));
    
    let default_aa_types = create_default_aa_types();
    
    themes.push(Theme {
        name: "Default".to_string(),
        elements: default_elements.clone(),
        aa_groups: default_aa_groups.clone(),
        aa_types: default_aa_types.clone(),
        gradient_start: Color::from_rgba(50, 100, 255, 255),
        gradient_end: Color::from_rgba(255, 100, 100, 255),
    });
    
    // Myth-Bursting theme - Space Cadet, Keppel, Orange-Yellow, Yellow Orange, Razzmatazz
    let mut myth_elements = HashMap::new();
    myth_elements.insert("C".to_string(), Color::from_rgba(38, 37, 84, 255));  // Space Cadet
    ocean_elements.insert("O".to_string(), Color::from_rgba(255, 107, 107, 255));
    ocean_elements.insert("N".to_string(), Color::from_rgba(78, 205, 196, 255));
    ocean_elements.insert("S".to_string(), Color::from_rgba(255, 230, 109, 255));
    ocean_elements.insert("H".to_string(), Color::from_rgba(168, 218, 220, 255));
    ocean_elements.insert("P".to_string(), Color::from_rgba(244, 162, 97, 255));
    
    let mut ocean_aa_groups = HashMap::new();
    ocean_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(38, 70, 83, 255));
    ocean_aa_groups.insert("Polar".to_string(), Color::from_rgba(42, 157, 143, 255));
    ocean_aa_groups.insert("Positive".to_string(), Color::from_rgba(69, 123, 157, 255));
    ocean_aa_groups.insert("Negative".to_string(), Color::from_rgba(230, 57, 70, 255));
    ocean_aa_groups.insert("Glycine".to_string(), Color::from_rgba(241, 250, 238, 255));
    
    let ocean_gradient_start = Color::from_rgba(6, 174, 213, 255);
    let ocean_gradient_end = Color::from_rgba(221, 28, 26, 255);
    
    themes.push(Theme {
        name: "Ocean".to_string(),
        elements: ocean_elements.clone(),
        aa_groups: ocean_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&ocean_elements, &ocean_aa_groups, ocean_gradient_start, ocean_gradient_end),
        gradient_start: ocean_gradient_start,
        gradient_end: ocean_gradient_end,
    });
    
    // Sunset theme
    let mut sunset_elements = HashMap::new();
    sunset_elements.insert("C".to_string(), Color::from_rgba(106, 76, 147, 255));
    sunset_elements.insert("O".to_string(), Color::from_rgba(255, 107, 157, 255));
    sunset_elements.insert("N".to_string(), Color::from_rgba(201, 173, 167, 255));
    sunset_elements.insert("S".to_string(), Color::from_rgba(242, 204, 143, 255));
    sunset_elements.insert("H".to_string(), Color::from_rgba(232, 223, 208, 255));
    sunset_elements.insert("P".to_string(), Color::from_rgba(224, 122, 95, 255));
    
    let mut sunset_aa_groups = HashMap::new();
    sunset_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(141, 91, 76, 255));
    sunset_aa_groups.insert("Polar".to_string(), Color::from_rgba(242, 204, 143, 255));
    sunset_aa_groups.insert("Positive".to_string(), Color::from_rgba(106, 76, 147, 255));
    sunset_aa_groups.insert("Negative".to_string(), Color::from_rgba(224, 122, 95, 255));
    sunset_aa_groups.insert("Glycine".to_string(), Color::from_rgba(244, 241, 222, 255));
    
    let sunset_gradient_start = Color::from_rgba(201, 173, 167, 255);
    let sunset_gradient_end = Color::from_rgba(224, 122, 95, 255);
    
    themes.push(Theme {
        name: "Sunset".to_string(),
        elements: sunset_elements.clone(),
        aa_groups: sunset_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&sunset_elements, &sunset_aa_groups, sunset_gradient_start, sunset_gradient_end),
        gradient_start: sunset_gradient_start,
        gradient_end: sunset_gradient_end,
    });
    
    // Neon theme - toned down for better visibility
    let mut neon_elements = HashMap::new();
    neon_elements.insert("C".to_string(), Color::from_rgba(102, 255, 178, 255));
    neon_elements.insert("O".to_string(), Color::from_rgba(255, 102, 178, 255));
    neon_elements.insert("N".to_string(), Color::from_rgba(102, 204, 255, 255));
    neon_elements.insert("S".to_string(), Color::from_rgba(255, 230, 102, 255));
    neon_elements.insert("H".to_string(), Color::from_rgba(204, 153, 255, 255));
    neon_elements.insert("P".to_string(), Color::from_rgba(255, 153, 51, 255));
    
    let mut neon_aa_groups = HashMap::new();
    neon_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(255, 204, 102, 255));
    neon_aa_groups.insert("Polar".to_string(), Color::from_rgba(102, 255, 178, 255));
    neon_aa_groups.insert("Positive".to_string(), Color::from_rgba(102, 204, 255, 255));
    neon_aa_groups.insert("Negative".to_string(), Color::from_rgba(255, 102, 204, 255));
    neon_aa_groups.insert("Glycine".to_string(), Color::from_rgba(230, 230, 255, 255));
    
    let neon_gradient_start = Color::from_rgba(204, 102, 255, 255);
    let neon_gradient_end = Color::from_rgba(102, 255, 204, 255);
    
    themes.push(Theme {
        name: "Neon".to_string(),
        elements: neon_elements.clone(),
        aa_groups: neon_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&neon_elements, &neon_aa_groups, neon_gradient_start, neon_gradient_end),
        gradient_start: neon_gradient_start,
        gradient_end: neon_gradient_end,
    });
    
    // Forest theme
    let mut forest_elements = HashMap::new();
    forest_elements.insert("C".to_string(), Color::from_rgba(45, 79, 60, 255));
    forest_elements.insert("O".to_string(), Color::from_rgba(193, 102, 107, 255));
    forest_elements.insert("N".to_string(), Color::from_rgba(74, 124, 89, 255));
    forest_elements.insert("S".to_string(), Color::from_rgba(212, 165, 116, 255));
    forest_elements.insert("H".to_string(), Color::from_rgba(143, 188, 143, 255));
    forest_elements.insert("P".to_string(), Color::from_rgba(180, 132, 108, 255));
    
    let mut forest_aa_groups = HashMap::new();
    forest_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(95, 133, 117, 255));
    forest_aa_groups.insert("Polar".to_string(), Color::from_rgba(159, 184, 173, 255));
    forest_aa_groups.insert("Positive".to_string(), Color::from_rgba(74, 124, 89, 255));
    forest_aa_groups.insert("Negative".to_string(), Color::from_rgba(139, 99, 92, 255));
    forest_aa_groups.insert("Glycine".to_string(), Color::from_rgba(232, 232, 206, 255));
    
    let forest_gradient_start = Color::from_rgba(53, 94, 59, 255);
    let forest_gradient_end = Color::from_rgba(143, 188, 143, 255);
    
    themes.push(Theme {
        name: "Forest".to_string(),
        elements: forest_elements.clone(),
        aa_groups: forest_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&forest_elements, &forest_aa_groups, forest_gradient_start, forest_gradient_end),
        gradient_start: forest_gradient_start,
        gradient_end: forest_gradient_end,
    });
    
    // Cyberpunk theme
    let mut cyberpunk_elements = HashMap::new();
    cyberpunk_elements.insert("C".to_string(), Color::from_rgba(255, 0, 110, 255));
    cyberpunk_elements.insert("O".to_string(), Color::from_rgba(131, 56, 236, 255));
    cyberpunk_elements.insert("N".to_string(), Color::from_rgba(58, 134, 255, 255));
    cyberpunk_elements.insert("S".to_string(), Color::from_rgba(255, 190, 11, 255));
    cyberpunk_elements.insert("H".to_string(), Color::from_rgba(251, 86, 7, 255));
    cyberpunk_elements.insert("P".to_string(), Color::from_rgba(6, 255, 165, 255));
    
    let mut cyberpunk_aa_groups = HashMap::new();
    cyberpunk_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(255, 0, 110, 255));
    cyberpunk_aa_groups.insert("Polar".to_string(), Color::from_rgba(58, 134, 255, 255));
    cyberpunk_aa_groups.insert("Positive".to_string(), Color::from_rgba(131, 56, 236, 255));
    cyberpunk_aa_groups.insert("Negative".to_string(), Color::from_rgba(251, 86, 7, 255));
    cyberpunk_aa_groups.insert("Glycine".to_string(), Color::from_rgba(255, 255, 255, 255));
    
    let cyberpunk_gradient_start = Color::from_rgba(131, 56, 236, 255);
    let cyberpunk_gradient_end = Color::from_rgba(255, 0, 110, 255);
    
    themes.push(Theme {
        name: "Cyberpunk".to_string(),
        elements: cyberpunk_elements.clone(),
        aa_groups: cyberpunk_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&cyberpunk_elements, &cyberpunk_aa_groups, cyberpunk_gradient_start, cyberpunk_gradient_end),
        gradient_start: cyberpunk_gradient_start,
        gradient_end: cyberpunk_gradient_end,
    });
    
    // Material Design theme - Refined Google Material Design palette
    let mut material_elements = HashMap::new();
    material_elements.insert("C".to_string(), Color::from_rgba(69, 90, 100, 255));
    material_elements.insert("O".to_string(), Color::from_rgba(229, 115, 115, 255));
    material_elements.insert("N".to_string(), Color::from_rgba(100, 181, 246, 255));
    material_elements.insert("S".to_string(), Color::from_rgba(255, 213, 79, 255));
    material_elements.insert("H".to_string(), Color::from_rgba(189, 189, 189, 255));
    material_elements.insert("P".to_string(), Color::from_rgba(255, 167, 38, 255));
    
    let mut material_aa_groups = HashMap::new();
    material_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(255, 183, 77, 255));
    material_aa_groups.insert("Polar".to_string(), Color::from_rgba(129, 199, 132, 255));
    material_aa_groups.insert("Positive".to_string(), Color::from_rgba(79, 195, 247, 255));
    material_aa_groups.insert("Negative".to_string(), Color::from_rgba(240, 98, 146, 255));
    material_aa_groups.insert("Glycine".to_string(), Color::from_rgba(238, 238, 238, 255));
    
    let material_gradient_start = Color::from_rgba(103, 58, 183, 255);
    let material_gradient_end = Color::from_rgba(0, 150, 136, 255);
    
    themes.push(Theme {
        name: "Material".to_string(),
        elements: material_elements.clone(),
        aa_groups: material_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&material_elements, &material_aa_groups, material_gradient_start, material_gradient_end),
        gradient_start: material_gradient_start,
        gradient_end: material_gradient_end,
    });
    
    // Viridis theme - Perceptually uniform scientific colormap
    let mut viridis_elements = HashMap::new();
    viridis_elements.insert("C".to_string(), Color::from_rgba(68, 1, 84, 255));
    viridis_elements.insert("O".to_string(), Color::from_rgba(253, 231, 37, 255));
    viridis_elements.insert("N".to_string(), Color::from_rgba(49, 104, 142, 255));
    viridis_elements.insert("S".to_string(), Color::from_rgba(181, 222, 43, 255));
    viridis_elements.insert("H".to_string(), Color::from_rgba(122, 209, 81, 255));
    viridis_elements.insert("P".to_string(), Color::from_rgba(254, 153, 41, 255));
    
    let mut viridis_aa_groups = HashMap::new();
    viridis_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(68, 1, 84, 255));
    viridis_aa_groups.insert("Polar".to_string(), Color::from_rgba(59, 82, 139, 255));
    viridis_aa_groups.insert("Positive".to_string(), Color::from_rgba(33, 145, 140, 255));
    viridis_aa_groups.insert("Negative".to_string(), Color::from_rgba(253, 231, 37, 255));
    viridis_aa_groups.insert("Glycine".to_string(), Color::from_rgba(204, 235, 197, 255));
    
    let viridis_gradient_start = Color::from_rgba(68, 1, 84, 255);
    let viridis_gradient_end = Color::from_rgba(253, 231, 37, 255);
    
    themes.push(Theme {
        name: "Viridis".to_string(),
        elements: viridis_elements.clone(),
        aa_groups: viridis_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&viridis_elements, &viridis_aa_groups, viridis_gradient_start, viridis_gradient_end),
        gradient_start: viridis_gradient_start,
        gradient_end: viridis_gradient_end,
    });
    
    // Nord theme - Popular Arctic-inspired palette
    let mut nord_elements = HashMap::new();
    nord_elements.insert("C".to_string(), Color::from_rgba(76, 86, 106, 255));
    nord_elements.insert("O".to_string(), Color::from_rgba(191, 97, 106, 255));
    nord_elements.insert("N".to_string(), Color::from_rgba(129, 161, 193, 255));
    nord_elements.insert("S".to_string(), Color::from_rgba(235, 203, 139, 255));
    nord_elements.insert("H".to_string(), Color::from_rgba(216, 222, 233, 255));
    nord_elements.insert("P".to_string(), Color::from_rgba(208, 135, 112, 255));
    
    let mut nord_aa_groups = HashMap::new();
    nord_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(235, 203, 139, 255));
    nord_aa_groups.insert("Polar".to_string(), Color::from_rgba(163, 190, 140, 255));
    nord_aa_groups.insert("Positive".to_string(), Color::from_rgba(136, 192, 208, 255));
    nord_aa_groups.insert("Negative".to_string(), Color::from_rgba(191, 97, 106, 255));
    nord_aa_groups.insert("Glycine".to_string(), Color::from_rgba(236, 239, 244, 255));
    
    let nord_gradient_start = Color::from_rgba(143, 188, 187, 255);
    let nord_gradient_end = Color::from_rgba(191, 97, 106, 255);
    
    themes.push(Theme {
        name: "Nord".to_string(),
        elements: nord_elements.clone(),
        aa_groups: nord_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&nord_elements, &nord_aa_groups, nord_gradient_start, nord_gradient_end),
        gradient_start: nord_gradient_start,
        gradient_end: nord_gradient_end,
    });
    
    // Tableau Classic theme - Standard data visualization palette
    let mut tableau_elements = HashMap::new();
    tableau_elements.insert("C".to_string(), Color::from_rgba(78, 121, 167, 255));
    tableau_elements.insert("O".to_string(), Color::from_rgba(242, 142, 43, 255));
    tableau_elements.insert("N".to_string(), Color::from_rgba(89, 161, 79, 255));
    tableau_elements.insert("S".to_string(), Color::from_rgba(225, 87, 89, 255));
    tableau_elements.insert("H".to_string(), Color::from_rgba(176, 122, 161, 255));
    tableau_elements.insert("P".to_string(), Color::from_rgba(237, 201, 72, 255));
    
    let mut tableau_aa_groups = HashMap::new();
    tableau_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(242, 142, 43, 255));
    tableau_aa_groups.insert("Polar".to_string(), Color::from_rgba(89, 161, 79, 255));
    tableau_aa_groups.insert("Positive".to_string(), Color::from_rgba(78, 121, 167, 255));
    tableau_aa_groups.insert("Negative".to_string(), Color::from_rgba(225, 87, 89, 255));
    tableau_aa_groups.insert("Glycine".to_string(), Color::from_rgba(230, 230, 230, 255));
    
    let tableau_gradient_start = Color::from_rgba(78, 121, 167, 255);
    let tableau_gradient_end = Color::from_rgba(225, 87, 89, 255);
    
    themes.push(Theme {
        name: "Tableau".to_string(),
        elements: tableau_elements.clone(),
        aa_groups: tableau_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&tableau_elements, &tableau_aa_groups, tableau_gradient_start, tableau_gradient_end),
        gradient_start: tableau_gradient_start,
        gradient_end: tableau_gradient_end,
    });
    
    // Solarized theme - Ethan Schoonover's precision color scheme
    let mut solarized_elements = HashMap::new();
    solarized_elements.insert("C".to_string(), Color::from_rgba(88, 110, 117, 255));
    solarized_elements.insert("O".to_string(), Color::from_rgba(220, 50, 47, 255));
    solarized_elements.insert("N".to_string(), Color::from_rgba(38, 139, 210, 255));
    solarized_elements.insert("S".to_string(), Color::from_rgba(181, 137, 0, 255));
    solarized_elements.insert("H".to_string(), Color::from_rgba(147, 161, 161, 255));
    solarized_elements.insert("P".to_string(), Color::from_rgba(203, 75, 22, 255));
    
    let mut solarized_aa_groups = HashMap::new();
    solarized_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(181, 137, 0, 255));
    solarized_aa_groups.insert("Polar".to_string(), Color::from_rgba(133, 153, 0, 255));
    solarized_aa_groups.insert("Positive".to_string(), Color::from_rgba(42, 161, 152, 255));
    solarized_aa_groups.insert("Negative".to_string(), Color::from_rgba(211, 54, 130, 255));
    solarized_aa_groups.insert("Glycine".to_string(), Color::from_rgba(238, 232, 213, 255));
    
    let solarized_gradient_start = Color::from_rgba(38, 139, 210, 255);
    let solarized_gradient_end = Color::from_rgba(220, 50, 47, 255);
    
    themes.push(Theme {
        name: "Solarized".to_string(),
        elements: solarized_elements.clone(),
        aa_groups: solarized_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&solarized_elements, &solarized_aa_groups, solarized_gradient_start, solarized_gradient_end),
        gradient_start: solarized_gradient_start,
        gradient_end: solarized_gradient_end,
    });
    
    // Dracula theme - Dark, vampire-inspired palette
    let mut dracula_elements = HashMap::new();
    dracula_elements.insert("C".to_string(), Color::from_rgba(98, 114, 164, 255));
    dracula_elements.insert("O".to_string(), Color::from_rgba(255, 85, 85, 255));
    dracula_elements.insert("N".to_string(), Color::from_rgba(139, 233, 253, 255));
    dracula_elements.insert("S".to_string(), Color::from_rgba(241, 250, 140, 255));
    dracula_elements.insert("H".to_string(), Color::from_rgba(189, 147, 249, 255));
    dracula_elements.insert("P".to_string(), Color::from_rgba(255, 184, 108, 255));
    
    let mut dracula_aa_groups = HashMap::new();
    dracula_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(255, 184, 108, 255));
    dracula_aa_groups.insert("Polar".to_string(), Color::from_rgba(80, 250, 123, 255));
    dracula_aa_groups.insert("Positive".to_string(), Color::from_rgba(139, 233, 253, 255));
    dracula_aa_groups.insert("Negative".to_string(), Color::from_rgba(255, 121, 198, 255));
    dracula_aa_groups.insert("Glycine".to_string(), Color::from_rgba(248, 248, 242, 255));
    
    let dracula_gradient_start = Color::from_rgba(189, 147, 249, 255);
    let dracula_gradient_end = Color::from_rgba(255, 85, 85, 255);
    
    themes.push(Theme {
        name: "Dracula".to_string(),
        elements: dracula_elements.clone(),
        aa_groups: dracula_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&dracula_elements, &dracula_aa_groups, dracula_gradient_start, dracula_gradient_end),
        gradient_start: dracula_gradient_start,
        gradient_end: dracula_gradient_end,
    });
    
    // Gruvbox theme - Retro groove palette
    let mut gruvbox_elements = HashMap::new();
    gruvbox_elements.insert("C".to_string(), Color::from_rgba(102, 92, 84, 255));
    gruvbox_elements.insert("O".to_string(), Color::from_rgba(251, 73, 52, 255));
    gruvbox_elements.insert("N".to_string(), Color::from_rgba(131, 165, 152, 255));
    gruvbox_elements.insert("S".to_string(), Color::from_rgba(250, 189, 47, 255));
    gruvbox_elements.insert("H".to_string(), Color::from_rgba(213, 196, 161, 255));
    gruvbox_elements.insert("P".to_string(), Color::from_rgba(254, 128, 25, 255));
    
    let mut gruvbox_aa_groups = HashMap::new();
    gruvbox_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(254, 128, 25, 255));
    gruvbox_aa_groups.insert("Polar".to_string(), Color::from_rgba(184, 187, 38, 255));
    gruvbox_aa_groups.insert("Positive".to_string(), Color::from_rgba(131, 165, 152, 255));
    gruvbox_aa_groups.insert("Negative".to_string(), Color::from_rgba(204, 36, 29, 255));
    gruvbox_aa_groups.insert("Glycine".to_string(), Color::from_rgba(235, 219, 178, 255));
    
    let gruvbox_gradient_start = Color::from_rgba(69, 133, 136, 255);
    let gruvbox_gradient_end = Color::from_rgba(251, 73, 52, 255);
    
    themes.push(Theme {
        name: "Gruvbox".to_string(),
        elements: gruvbox_elements.clone(),
        aa_groups: gruvbox_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&gruvbox_elements, &gruvbox_aa_groups, gruvbox_gradient_start, gruvbox_gradient_end),
        gradient_start: gruvbox_gradient_start,
        gradient_end: gruvbox_gradient_end,
    });
    
    // Tokyo Night theme - Modern night-time cityscape
    let mut tokyo_elements = HashMap::new();
    tokyo_elements.insert("C".to_string(), Color::from_rgba(122, 162, 247, 255));
    tokyo_elements.insert("O".to_string(), Color::from_rgba(247, 118, 142, 255));
    tokyo_elements.insert("N".to_string(), Color::from_rgba(125, 207, 255, 255));
    tokyo_elements.insert("S".to_string(), Color::from_rgba(224, 175, 104, 255));
    tokyo_elements.insert("H".to_string(), Color::from_rgba(169, 177, 214, 255));
    tokyo_elements.insert("P".to_string(), Color::from_rgba(255, 158, 100, 255));
    
    let mut tokyo_aa_groups = HashMap::new();
    tokyo_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(224, 175, 104, 255));
    tokyo_aa_groups.insert("Polar".to_string(), Color::from_rgba(158, 206, 106, 255));
    tokyo_aa_groups.insert("Positive".to_string(), Color::from_rgba(122, 162, 247, 255));
    tokyo_aa_groups.insert("Negative".to_string(), Color::from_rgba(247, 118, 142, 255));
    tokyo_aa_groups.insert("Glycine".to_string(), Color::from_rgba(192, 202, 245, 255));
    
    let tokyo_gradient_start = Color::from_rgba(187, 154, 247, 255);
    let tokyo_gradient_end = Color::from_rgba(247, 118, 142, 255);
    
    themes.push(Theme {
        name: "Tokyo Night".to_string(),
        elements: tokyo_elements.clone(),
        aa_groups: tokyo_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&tokyo_elements, &tokyo_aa_groups, tokyo_gradient_start, tokyo_gradient_end),
        gradient_start: tokyo_gradient_start,
        gradient_end: tokyo_gradient_end,
    });
    
    // Catppuccin theme - Soothing pastel palette
    let mut catppuccin_elements = HashMap::new();
    catppuccin_elements.insert("C".to_string(), Color::from_rgba(116, 199, 236, 255));
    catppuccin_elements.insert("O".to_string(), Color::from_rgba(242, 143, 173, 255));
    catppuccin_elements.insert("N".to_string(), Color::from_rgba(137, 180, 250, 255));
    catppuccin_elements.insert("S".to_string(), Color::from_rgba(249, 226, 175, 255));
    catppuccin_elements.insert("H".to_string(), Color::from_rgba(203, 166, 247, 255));
    catppuccin_elements.insert("P".to_string(), Color::from_rgba(245, 194, 231, 255));
    
    let mut catppuccin_aa_groups = HashMap::new();
    catppuccin_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(249, 226, 175, 255));
    catppuccin_aa_groups.insert("Polar".to_string(), Color::from_rgba(166, 227, 161, 255));
    catppuccin_aa_groups.insert("Positive".to_string(), Color::from_rgba(137, 180, 250, 255));
    catppuccin_aa_groups.insert("Negative".to_string(), Color::from_rgba(243, 139, 168, 255));
    catppuccin_aa_groups.insert("Glycine".to_string(), Color::from_rgba(238, 212, 232, 255));
    
    let catppuccin_gradient_start = Color::from_rgba(203, 166, 247, 255);
    let catppuccin_gradient_end = Color::from_rgba(242, 143, 173, 255);
    
    themes.push(Theme {
        name: "Catppuccin".to_string(),
        elements: catppuccin_elements.clone(),
        aa_groups: catppuccin_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&catppuccin_elements, &catppuccin_aa_groups, catppuccin_gradient_start, catppuccin_gradient_end),
        gradient_start: catppuccin_gradient_start,
        gradient_end: catppuccin_gradient_end,
    });
    
    // Monokai theme - Classic code editor colors
    let mut monokai_elements = HashMap::new();
    monokai_elements.insert("C".to_string(), Color::from_rgba(102, 217, 239, 255));
    monokai_elements.insert("O".to_string(), Color::from_rgba(249, 38, 114, 255));
    monokai_elements.insert("N".to_string(), Color::from_rgba(174, 129, 255, 255));
    monokai_elements.insert("S".to_string(), Color::from_rgba(230, 219, 116, 255));
    monokai_elements.insert("H".to_string(), Color::from_rgba(248, 248, 242, 255));
    monokai_elements.insert("P".to_string(), Color::from_rgba(253, 151, 31, 255));
    
    let mut monokai_aa_groups = HashMap::new();
    monokai_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(253, 151, 31, 255));
    monokai_aa_groups.insert("Polar".to_string(), Color::from_rgba(166, 226, 46, 255));
    monokai_aa_groups.insert("Positive".to_string(), Color::from_rgba(102, 217, 239, 255));
    monokai_aa_groups.insert("Negative".to_string(), Color::from_rgba(249, 38, 114, 255));
    monokai_aa_groups.insert("Glycine".to_string(), Color::from_rgba(248, 248, 242, 255));
    
    let monokai_gradient_start = Color::from_rgba(174, 129, 255, 255);
    let monokai_gradient_end = Color::from_rgba(249, 38, 114, 255);
    
    themes.push(Theme {
        name: "Monokai".to_string(),
        elements: monokai_elements.clone(),
        aa_groups: monokai_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&monokai_elements, &monokai_aa_groups, monokai_gradient_start, monokai_gradient_end),
        gradient_start: monokai_gradient_start,
        gradient_end: monokai_gradient_end,
    });
    
    // Coral Reef theme - Underwater ocean life
    let mut coral_elements = HashMap::new();
    coral_elements.insert("C".to_string(), Color::from_rgba(0, 119, 182, 255));
    coral_elements.insert("O".to_string(), Color::from_rgba(255, 127, 80, 255));
    coral_elements.insert("N".to_string(), Color::from_rgba(64, 224, 208, 255));
    coral_elements.insert("S".to_string(), Color::from_rgba(255, 218, 185, 255));
    coral_elements.insert("H".to_string(), Color::from_rgba(175, 238, 238, 255));
    coral_elements.insert("P".to_string(), Color::from_rgba(240, 128, 128, 255));
    
    let mut coral_aa_groups = HashMap::new();
    coral_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(255, 165, 0, 255));
    coral_aa_groups.insert("Polar".to_string(), Color::from_rgba(72, 209, 204, 255));
    coral_aa_groups.insert("Positive".to_string(), Color::from_rgba(100, 149, 237, 255));
    coral_aa_groups.insert("Negative".to_string(), Color::from_rgba(250, 128, 114, 255));
    coral_aa_groups.insert("Glycine".to_string(), Color::from_rgba(240, 248, 255, 255));
    
    let coral_gradient_start = Color::from_rgba(0, 191, 255, 255);
    let coral_gradient_end = Color::from_rgba(255, 127, 80, 255);
    
    themes.push(Theme {
        name: "Coral Reef".to_string(),
        elements: coral_elements.clone(),
        aa_groups: coral_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&coral_elements, &coral_aa_groups, coral_gradient_start, coral_gradient_end),
        gradient_start: coral_gradient_start,
        gradient_end: coral_gradient_end,
    });
    
    // Sakura theme - Japanese cherry blossom
    let mut sakura_elements = HashMap::new();
    sakura_elements.insert("C".to_string(), Color::from_rgba(141, 100, 149, 255));
    sakura_elements.insert("O".to_string(), Color::from_rgba(255, 183, 197, 255));
    sakura_elements.insert("N".to_string(), Color::from_rgba(187, 222, 251, 255));
    sakura_elements.insert("S".to_string(), Color::from_rgba(255, 228, 196, 255));
    sakura_elements.insert("H".to_string(), Color::from_rgba(255, 240, 245, 255));
    sakura_elements.insert("P".to_string(), Color::from_rgba(255, 192, 203, 255));
    
    let mut sakura_aa_groups = HashMap::new();
    sakura_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(255, 228, 225, 255));
    sakura_aa_groups.insert("Polar".to_string(), Color::from_rgba(255, 218, 224, 255));
    sakura_aa_groups.insert("Positive".to_string(), Color::from_rgba(221, 160, 221, 255));
    sakura_aa_groups.insert("Negative".to_string(), Color::from_rgba(255, 182, 193, 255));
    sakura_aa_groups.insert("Glycine".to_string(), Color::from_rgba(255, 250, 250, 255));
    
    let sakura_gradient_start = Color::from_rgba(255, 182, 193, 255);
    let sakura_gradient_end = Color::from_rgba(216, 191, 216, 255);
    
    themes.push(Theme {
        name: "Sakura".to_string(),
        elements: sakura_elements.clone(),
        aa_groups: sakura_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&sakura_elements, &sakura_aa_groups, sakura_gradient_start, sakura_gradient_end),
        gradient_start: sakura_gradient_start,
        gradient_end: sakura_gradient_end,
    });
    
    // Autumn Leaves theme - Fall foliage palette
    let mut autumn_elements = HashMap::new();
    autumn_elements.insert("C".to_string(), Color::from_rgba(139, 69, 19, 255));
    autumn_elements.insert("O".to_string(), Color::from_rgba(205, 92, 92, 255));
    autumn_elements.insert("N".to_string(), Color::from_rgba(210, 105, 30, 255));
    autumn_elements.insert("S".to_string(), Color::from_rgba(218, 165, 32, 255));
    autumn_elements.insert("H".to_string(), Color::from_rgba(244, 164, 96, 255));
    autumn_elements.insert("P".to_string(), Color::from_rgba(255, 140, 0, 255));
    
    let mut autumn_aa_groups = HashMap::new();
    autumn_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(205, 133, 63, 255));
    autumn_aa_groups.insert("Polar".to_string(), Color::from_rgba(189, 183, 107, 255));
    autumn_aa_groups.insert("Positive".to_string(), Color::from_rgba(160, 82, 45, 255));
    autumn_aa_groups.insert("Negative".to_string(), Color::from_rgba(178, 34, 34, 255));
    autumn_aa_groups.insert("Glycine".to_string(), Color::from_rgba(255, 228, 196, 255));
    
    let autumn_gradient_start = Color::from_rgba(184, 134, 11, 255);
    let autumn_gradient_end = Color::from_rgba(139, 0, 0, 255);
    
    themes.push(Theme {
        name: "Autumn".to_string(),
        elements: autumn_elements.clone(),
        aa_groups: autumn_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&autumn_elements, &autumn_aa_groups, autumn_gradient_start, autumn_gradient_end),
        gradient_start: autumn_gradient_start,
        gradient_end: autumn_gradient_end,
    });
    
    // Mint Cream theme - Fresh and clean pastels
    let mut mint_elements = HashMap::new();
    mint_elements.insert("C".to_string(), Color::from_rgba(152, 251, 152, 255));
    mint_elements.insert("O".to_string(), Color::from_rgba(255, 160, 122, 255));
    mint_elements.insert("N".to_string(), Color::from_rgba(176, 224, 230, 255));
    mint_elements.insert("S".to_string(), Color::from_rgba(255, 255, 224, 255));
    mint_elements.insert("H".to_string(), Color::from_rgba(240, 255, 240, 255));
    mint_elements.insert("P".to_string(), Color::from_rgba(255, 228, 181, 255));
    
    let mut mint_aa_groups = HashMap::new();
    mint_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(255, 250, 205, 255));
    mint_aa_groups.insert("Polar".to_string(), Color::from_rgba(224, 255, 255, 255));
    mint_aa_groups.insert("Positive".to_string(), Color::from_rgba(230, 230, 250, 255));
    mint_aa_groups.insert("Negative".to_string(), Color::from_rgba(255, 228, 225, 255));
    mint_aa_groups.insert("Glycine".to_string(), Color::from_rgba(248, 248, 255, 255));
    
    let mint_gradient_start = Color::from_rgba(152, 251, 152, 255);
    let mint_gradient_end = Color::from_rgba(255, 182, 193, 255);
    
    themes.push(Theme {
        name: "Mint Cream".to_string(),
        elements: mint_elements.clone(),
        aa_groups: mint_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&mint_elements, &mint_aa_groups, mint_gradient_start, mint_gradient_end),
        gradient_start: mint_gradient_start,
        gradient_end: mint_gradient_end,
    });
    
    // Spectral theme - Rainbow spectrum for scientific visualization
    let mut spectral_elements = HashMap::new();
    spectral_elements.insert("C".to_string(), Color::from_rgba(158, 1, 66, 255));
    spectral_elements.insert("O".to_string(), Color::from_rgba(213, 62, 79, 255));
    spectral_elements.insert("N".to_string(), Color::from_rgba(94, 79, 162, 255));
    spectral_elements.insert("S".to_string(), Color::from_rgba(254, 224, 139, 255));
    spectral_elements.insert("H".to_string(), Color::from_rgba(230, 245, 152, 255));
    spectral_elements.insert("P".to_string(), Color::from_rgba(253, 174, 97, 255));
    
    let mut spectral_aa_groups = HashMap::new();
    spectral_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(244, 109, 67, 255));
    spectral_aa_groups.insert("Polar".to_string(), Color::from_rgba(171, 217, 233, 255));
    spectral_aa_groups.insert("Positive".to_string(), Color::from_rgba(116, 173, 209, 255));
    spectral_aa_groups.insert("Negative".to_string(), Color::from_rgba(213, 62, 79, 255));
    spectral_aa_groups.insert("Glycine".to_string(), Color::from_rgba(255, 255, 191, 255));
    
    let spectral_gradient_start = Color::from_rgba(158, 1, 66, 255);
    let spectral_gradient_end = Color::from_rgba(94, 79, 162, 255);
    
    themes.push(Theme {
        name: "Spectral".to_string(),
        elements: spectral_elements.clone(),
        aa_groups: spectral_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&spectral_elements, &spectral_aa_groups, spectral_gradient_start, spectral_gradient_end),
        gradient_start: spectral_gradient_start,
        gradient_end: spectral_gradient_end,
    });
    
    // Retro theme - Vintage 80s computer graphics
    let mut retro_elements = HashMap::new();
    retro_elements.insert("C".to_string(), Color::from_rgba(0, 204, 204, 255));
    retro_elements.insert("O".to_string(), Color::from_rgba(255, 0, 127, 255));
    retro_elements.insert("N".to_string(), Color::from_rgba(255, 255, 0, 255));
    retro_elements.insert("S".to_string(), Color::from_rgba(255, 204, 0, 255));
    retro_elements.insert("H".to_string(), Color::from_rgba(204, 204, 255, 255));
    retro_elements.insert("P".to_string(), Color::from_rgba(255, 153, 0, 255));
    
    let mut retro_aa_groups = HashMap::new();
    retro_aa_groups.insert("Hydrophobic".to_string(), Color::from_rgba(255, 204, 0, 255));
    retro_aa_groups.insert("Polar".to_string(), Color::from_rgba(0, 255, 127, 255));
    retro_aa_groups.insert("Positive".to_string(), Color::from_rgba(0, 204, 255, 255));
    retro_aa_groups.insert("Negative".to_string(), Color::from_rgba(255, 51, 153, 255));
    retro_aa_groups.insert("Glycine".to_string(), Color::from_rgba(255, 255, 204, 255));
    
    let retro_gradient_start = Color::from_rgba(255, 0, 255, 255);
    let retro_gradient_end = Color::from_rgba(0, 255, 255, 255);
    
    themes.push(Theme {
        name: "Retro".to_string(),
        elements: retro_elements.clone(),
        aa_groups: retro_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&retro_elements, &retro_aa_groups, retro_gradient_start, retro_gradient_end),
        gradient_start: retro_gradient_start,
        gradient_end: retro_gradient_end,
    });
    
    themes
}

fn create_default_aa_types() -> HashMap<String, Color> {
    let mut aa_types = HashMap::new();
    aa_types.insert("ALA".to_string(), Color::from_rgba(120, 180, 255, 255));
    aa_types.insert("ARG".to_string(), Color::from_rgba(255, 50, 100, 255));
    aa_types.insert("ASN".to_string(), Color::from_rgba(0, 220, 180, 255));
    aa_types.insert("ASP".to_string(), Color::from_rgba(255, 80, 50, 255));
    aa_types.insert("CYS".to_string(), Color::from_rgba(255, 230, 0, 255));
    aa_types.insert("GLN".to_string(), Color::from_rgba(80, 200, 200, 255));
    aa_types.insert("GLU".to_string(), Color::from_rgba(255, 120, 0, 255));
    aa_types.insert("GLY".to_string(), Color::from_rgba(240, 240, 240, 255));
    aa_types.insert("HIS".to_string(), Color::from_rgba(100, 150, 255, 255));
    aa_types.insert("ILE".to_string(), Color::from_rgba(50, 200, 50, 255));
    aa_types.insert("LEU".to_string(), Color::from_rgba(150, 255, 50, 255));
    aa_types.insert("LYS".to_string(), Color::from_rgba(0, 100, 255, 255));
    aa_types.insert("MET".to_string(), Color::from_rgba(255, 200, 50, 255));
    aa_types.insert("PHE".to_string(), Color::from_rgba(150, 80, 220, 255));
    aa_types.insert("PRO".to_string(), Color::from_rgba(255, 170, 120, 255));
    aa_types.insert("SER".to_string(), Color::from_rgba(255, 150, 200, 255));
    aa_types.insert("THR".to_string(), Color::from_rgba(255, 120, 180, 255));
    aa_types.insert("TRP".to_string(), Color::from_rgba(200, 50, 200, 255));
    aa_types.insert("TYR".to_string(), Color::from_rgba(180, 120, 255, 255));
    aa_types.insert("VAL".to_string(), Color::from_rgba(0, 180, 120, 255));
    aa_types
}

pub fn generate_aa_types_from_palette(elements: &HashMap<String, Color>, aa_groups: &HashMap<String, Color>, gradient_start: Color, gradient_end: Color) -> HashMap<String, Color> {
    let mut aa_types = HashMap::new();
    
    // Collect available colors from elements and aa_groups
    let mut palette = Vec::new();
    for color in elements.values() {
        palette.push(*color);
    }
    for color in aa_groups.values() {
        palette.push(*color);
    }
    palette.push(gradient_start);
    palette.push(gradient_end);
    
    // Helper function to blend colors
    let blend = |c1: Color, c2: Color, t: f32| -> Color {
        Color::from_rgba(
            ((c1.r * (1.0 - t) + c2.r * t) * 255.0) as u8,
            ((c1.g * (1.0 - t) + c2.g * t) * 255.0) as u8,
            ((c1.b * (1.0 - t) + c2.b * t) * 255.0) as u8,
            255
        )
    };
    
    // Assign colors to amino acids with variations
    let amino_acids = [
        "ALA", "ARG", "ASN", "ASP", "CYS", "GLN", "GLU", "GLY", "HIS", "ILE",
        "LEU", "LYS", "MET", "PHE", "PRO", "SER", "THR", "TRP", "TYR", "VAL"
    ];
    
    for (i, aa) in amino_acids.iter().enumerate() {
        let idx1 = i % palette.len();
        let idx2 = (i + 3) % palette.len();
        let t = (i as f32) / (amino_acids.len() as f32);
        let color = blend(palette[idx1], palette[idx2], t);
        aa_types.insert(aa.to_string(), color);
    }
    
    // Special handling for Glycine - use the aa_groups glycine if available
    if let Some(gly_color) = aa_groups.get("Glycine") {
        aa_types.insert("GLY".to_string(), *gly_color);
    }
    
    aa_types
}
