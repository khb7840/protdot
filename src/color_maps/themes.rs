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
    
    // Myth-Bursting - Space Cadet, Keppel, Orange-Yellow, Yellow Orange, Razzmatazz
    let colors = [
        Color::from_rgba(38, 37, 84, 255),   // Space Cadet
        Color::from_rgba(52, 174, 158, 255), // Keppel
        Color::from_rgba(242, 214, 102, 255),// Orange-Yellow
        Color::from_rgba(242, 172, 68, 255), // Yellow Orange
        Color::from_rgba(228, 37, 95, 255),  // Razzmatazz
    ];
    let mut myth_elements = HashMap::new();
    myth_elements.insert("C".to_string(), colors[0]);
    myth_elements.insert("O".to_string(), colors[4]);
    myth_elements.insert("N".to_string(), colors[1]);
    myth_elements.insert("S".to_string(), colors[2]);
    myth_elements.insert("H".to_string(), colors[3]);
    myth_elements.insert("P".to_string(), colors[3]);
    
    let mut myth_aa_groups = HashMap::new();
    myth_aa_groups.insert("Hydrophobic".to_string(), colors[2]);
    myth_aa_groups.insert("Polar".to_string(), colors[1]);
    myth_aa_groups.insert("Positive".to_string(), colors[0]);
    myth_aa_groups.insert("Negative".to_string(), colors[4]);
    myth_aa_groups.insert("Glycine".to_string(), Color::from_rgba(200, 200, 200, 255));
    
    themes.push(Theme {
        name: "Myth-Bursting".to_string(),
        elements: myth_elements.clone(),
        aa_groups: myth_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&myth_elements, &myth_aa_groups, colors[0], colors[4]),
        gradient_start: colors[0],
        gradient_end: colors[4],
    });
    
    // Resistance To Love - Black, Space Cadet, Dark Slate Blue, Dark Pink, Rajah, Purple Plum
    let colors = [
        Color::from_rgba(0, 0, 0, 255),       // Black
        Color::from_rgba(43, 41, 84, 255),    // Space Cadet
        Color::from_rgba(68, 68, 145, 255),   // Dark Slate Blue
        Color::from_rgba(227, 79, 119, 255),  // Dark Pink
        Color::from_rgba(255, 180, 94, 255),  // Rajah
        Color::from_rgba(174, 75, 189, 255),  // Purple Plum
    ];
    let mut resist_elements = HashMap::new();
    resist_elements.insert("C".to_string(), colors[1]);
    resist_elements.insert("O".to_string(), colors[3]);
    resist_elements.insert("N".to_string(), colors[2]);
    resist_elements.insert("S".to_string(), colors[4]);
    resist_elements.insert("H".to_string(), Color::from_rgba(180, 180, 180, 255));
    resist_elements.insert("P".to_string(), colors[5]);
    
    let mut resist_aa_groups = HashMap::new();
    resist_aa_groups.insert("Hydrophobic".to_string(), colors[4]);
    resist_aa_groups.insert("Polar".to_string(), colors[2]);
    resist_aa_groups.insert("Positive".to_string(), colors[5]);
    resist_aa_groups.insert("Negative".to_string(), colors[3]);
    resist_aa_groups.insert("Glycine".to_string(), Color::from_rgba(200, 200, 200, 255));
    
    themes.push(Theme {
        name: "Resistance To Love".to_string(),
        elements: resist_elements.clone(),
        aa_groups: resist_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&resist_elements, &resist_aa_groups, colors[1], colors[3]),
        gradient_start: colors[1],
        gradient_end: colors[3],
    });
    
    // Attractive Vintage - Sunset Orange, Sandy Brown, Seashell, Dark Lavender, Cosmic Cobalt, Fiery Rose
    let colors = [
        Color::from_rgba(255, 88, 96, 255),   // Sunset Orange
        Color::from_rgba(236, 158, 98, 255),  // Sandy Brown
        Color::from_rgba(254, 246, 240, 255), // Seashell
        Color::from_rgba(115, 76, 159, 255),  // Dark Lavender
        Color::from_rgba(50, 50, 126, 255),   // Cosmic Cobalt
        Color::from_rgba(252, 87, 118, 255),  // Fiery Rose
    ];
    let mut vintage_elements = HashMap::new();
    vintage_elements.insert("C".to_string(), colors[4]);
    vintage_elements.insert("O".to_string(), colors[0]);
    vintage_elements.insert("N".to_string(), colors[3]);
    vintage_elements.insert("S".to_string(), colors[1]);
    vintage_elements.insert("H".to_string(), colors[2]);
    vintage_elements.insert("P".to_string(), colors[5]);
    
    let mut vintage_aa_groups = HashMap::new();
    vintage_aa_groups.insert("Hydrophobic".to_string(), colors[1]);
    vintage_aa_groups.insert("Polar".to_string(), colors[2]);
    vintage_aa_groups.insert("Positive".to_string(), colors[4]);
    vintage_aa_groups.insert("Negative".to_string(), colors[5]);
    vintage_aa_groups.insert("Glycine".to_string(), colors[2]);
    
    themes.push(Theme {
        name: "Attractive Vintage".to_string(),
        elements: vintage_elements.clone(),
        aa_groups: vintage_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&vintage_elements, &vintage_aa_groups, colors[4], colors[5]),
        gradient_start: colors[4],
        gradient_end: colors[5],
    });
    
    // Desi Istyle - Earth Yellow, Copper Red, Deep Puce, Imperial, Cookies And Cream
    let colors = [
        Color::from_rgba(226, 159, 99, 255),  // Earth Yellow
        Color::from_rgba(202, 110, 77, 255),  // Copper Red
        Color::from_rgba(174, 85, 103, 255),  // Deep Puce
        Color::from_rgba(103, 58, 110, 255),  // Imperial
        Color::from_rgba(238, 221, 174, 255), // Cookies And Cream
    ];
    let mut desi_elements = HashMap::new();
    desi_elements.insert("C".to_string(), colors[3]);
    desi_elements.insert("O".to_string(), colors[1]);
    desi_elements.insert("N".to_string(), colors[2]);
    desi_elements.insert("S".to_string(), colors[0]);
    desi_elements.insert("H".to_string(), colors[4]);
    desi_elements.insert("P".to_string(), colors[1]);
    
    let mut desi_aa_groups = HashMap::new();
    desi_aa_groups.insert("Hydrophobic".to_string(), colors[0]);
    desi_aa_groups.insert("Polar".to_string(), colors[4]);
    desi_aa_groups.insert("Positive".to_string(), colors[3]);
    desi_aa_groups.insert("Negative".to_string(), colors[1]);
    desi_aa_groups.insert("Glycine".to_string(), colors[4]);
    
    themes.push(Theme {
        name: "Desi Istyle".to_string(),
        elements: desi_elements.clone(),
        aa_groups: desi_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&desi_elements, &desi_aa_groups, colors[3], colors[0]),
        gradient_start: colors[3],
        gradient_end: colors[0],
    });
    
    // Juggle The Rainbow - Yellow-Green, Bleu De France, Dark Lavender, Electric Pink, Deep Saffron, Deep Lemon
    let colors = [
        Color::from_rgba(140, 198, 62, 255),  // Yellow-Green
        Color::from_rgba(41, 137, 227, 255),  // Bleu De France
        Color::from_rgba(114, 68, 152, 255),  // Dark Lavender
        Color::from_rgba(240, 44, 137, 255),  // Electric Pink
        Color::from_rgba(251, 148, 59, 255),  // Deep Saffron
        Color::from_rgba(244, 205, 38, 255),  // Deep Lemon
    ];
    let mut juggle_elements = HashMap::new();
    juggle_elements.insert("C".to_string(), colors[2]);
    juggle_elements.insert("O".to_string(), colors[3]);
    juggle_elements.insert("N".to_string(), colors[1]);
    juggle_elements.insert("S".to_string(), colors[5]);
    juggle_elements.insert("H".to_string(), colors[0]);
    juggle_elements.insert("P".to_string(), colors[4]);
    
    let mut juggle_aa_groups = HashMap::new();
    juggle_aa_groups.insert("Hydrophobic".to_string(), colors[4]);
    juggle_aa_groups.insert("Polar".to_string(), colors[0]);
    juggle_aa_groups.insert("Positive".to_string(), colors[1]);
    juggle_aa_groups.insert("Negative".to_string(), colors[3]);
    juggle_aa_groups.insert("Glycine".to_string(), Color::from_rgba(240, 240, 240, 255));
    
    themes.push(Theme {
        name: "Juggle The Rainbow".to_string(),
        elements: juggle_elements.clone(),
        aa_groups: juggle_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&juggle_elements, &juggle_aa_groups, colors[2], colors[3]),
        gradient_start: colors[2],
        gradient_end: colors[3],
    });
    
    // Indian Marriage - Dark Lemon Lime, Sandstorm, Beer, Pantone Magenta, Crayola's Violet, Dark Cornflower Blue
    let colors = [
        Color::from_rgba(142, 186, 22, 255),  // Dark Lemon Lime
        Color::from_rgba(250, 208, 65, 255),  // Sandstorm
        Color::from_rgba(242, 140, 34, 255),  // Beer
        Color::from_rgba(214, 63, 124, 255),  // Pantone Magenta
        Color::from_rgba(152, 56, 146, 255),  // Crayola's Violet
        Color::from_rgba(46, 64, 135, 255),   // Dark Cornflower Blue
    ];
    let mut indian_elements = HashMap::new();
    indian_elements.insert("C".to_string(), colors[5]);
    indian_elements.insert("O".to_string(), colors[3]);
    indian_elements.insert("N".to_string(), colors[4]);
    indian_elements.insert("S".to_string(), colors[1]);
    indian_elements.insert("H".to_string(), colors[0]);
    indian_elements.insert("P".to_string(), colors[2]);
    
    let mut indian_aa_groups = HashMap::new();
    indian_aa_groups.insert("Hydrophobic".to_string(), colors[2]);
    indian_aa_groups.insert("Polar".to_string(), colors[0]);
    indian_aa_groups.insert("Positive".to_string(), colors[5]);
    indian_aa_groups.insert("Negative".to_string(), colors[3]);
    indian_aa_groups.insert("Glycine".to_string(), Color::from_rgba(240, 240, 200, 255));
    
    themes.push(Theme {
        name: "Indian Marriage".to_string(),
        elements: indian_elements.clone(),
        aa_groups: indian_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&indian_elements, &indian_aa_groups, colors[5], colors[3]),
        gradient_start: colors[5],
        gradient_end: colors[3],
    });
    
    // Cream Truck - Navajo White, Chinese Purple, Maya Blue, French Rose, Cosmic Latte, Ceil
    let colors = [
        Color::from_rgba(254, 221, 174, 255), // Navajo White
        Color::from_rgba(106, 25, 142, 255),  // Chinese Purple
        Color::from_rgba(81, 209, 253, 255),  // Maya Blue
        Color::from_rgba(236, 80, 140, 255),  // French Rose
        Color::from_rgba(254, 254, 232, 255), // Cosmic Latte
        Color::from_rgba(154, 139, 215, 255), // Ceil
    ];
    let mut cream_elements = HashMap::new();
    cream_elements.insert("C".to_string(), colors[1]);
    cream_elements.insert("O".to_string(), colors[3]);
    cream_elements.insert("N".to_string(), colors[2]);
    cream_elements.insert("S".to_string(), colors[0]);
    cream_elements.insert("H".to_string(), colors[4]);
    cream_elements.insert("P".to_string(), colors[5]);
    
    let mut cream_aa_groups = HashMap::new();
    cream_aa_groups.insert("Hydrophobic".to_string(), colors[0]);
    cream_aa_groups.insert("Polar".to_string(), colors[2]);
    cream_aa_groups.insert("Positive".to_string(), colors[5]);
    cream_aa_groups.insert("Negative".to_string(), colors[3]);
    cream_aa_groups.insert("Glycine".to_string(), colors[4]);
    
    themes.push(Theme {
        name: "Cream Truck".to_string(),
        elements: cream_elements.clone(),
        aa_groups: cream_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&cream_elements, &cream_aa_groups, colors[1], colors[3]),
        gradient_start: colors[1],
        gradient_end: colors[3],
    });
    
    // Fakes Are For Free - Spicy Mix, Macaroni And Cheese, Navajo White, Air Superiority Blue, B'dazzled Blue
    let colors = [
        Color::from_rgba(132, 89, 83, 255),   // Spicy Mix
        Color::from_rgba(255, 183, 137, 255), // Macaroni And Cheese
        Color::from_rgba(255, 221, 170, 255), // Navajo White
        Color::from_rgba(110, 156, 184, 255), // Air Superiority Blue
        Color::from_rgba(50, 99, 136, 255),   // B'dazzled Blue
    ];
    let mut fakes_elements = HashMap::new();
    fakes_elements.insert("C".to_string(), colors[0]);
    fakes_elements.insert("O".to_string(), colors[1]);
    fakes_elements.insert("N".to_string(), colors[4]);
    fakes_elements.insert("S".to_string(), colors[2]);
    fakes_elements.insert("H".to_string(), colors[3]);
    fakes_elements.insert("P".to_string(), colors[1]);
    
    let mut fakes_aa_groups = HashMap::new();
    fakes_aa_groups.insert("Hydrophobic".to_string(), colors[1]);
    fakes_aa_groups.insert("Polar".to_string(), colors[3]);
    fakes_aa_groups.insert("Positive".to_string(), colors[4]);
    fakes_aa_groups.insert("Negative".to_string(), colors[0]);
    fakes_aa_groups.insert("Glycine".to_string(), colors[2]);
    
    themes.push(Theme {
        name: "Fakes Are For Free".to_string(),
        elements: fakes_elements.clone(),
        aa_groups: fakes_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&fakes_elements, &fakes_aa_groups, colors[4], colors[1]),
        gradient_start: colors[4],
        gradient_end: colors[1],
    });
    
    // Industrial Use - Japanese Indigo, Carmine Pink, Bright Gray, Tufts Blue, Vampire Black
    let colors = [
        Color::from_rgba(44, 63, 80, 255),    // Japanese Indigo
        Color::from_rgba(232, 76, 61, 255),   // Carmine Pink
        Color::from_rgba(236, 240, 241, 255), // Bright Gray
        Color::from_rgba(50, 151, 219, 255),  // Tufts Blue
        Color::from_rgba(6, 6, 6, 255),       // Vampire Black
    ];
    let mut industrial_elements = HashMap::new();
    industrial_elements.insert("C".to_string(), colors[0]);
    industrial_elements.insert("O".to_string(), colors[1]);
    industrial_elements.insert("N".to_string(), colors[3]);
    industrial_elements.insert("S".to_string(), Color::from_rgba(230, 210, 80, 255));
    industrial_elements.insert("H".to_string(), colors[2]);
    industrial_elements.insert("P".to_string(), colors[1]);
    
    let mut industrial_aa_groups = HashMap::new();
    industrial_aa_groups.insert("Hydrophobic".to_string(), colors[0]);
    industrial_aa_groups.insert("Polar".to_string(), colors[3]);
    industrial_aa_groups.insert("Positive".to_string(), colors[3]);
    industrial_aa_groups.insert("Negative".to_string(), colors[1]);
    industrial_aa_groups.insert("Glycine".to_string(), colors[2]);
    
    themes.push(Theme {
        name: "Industrial Use".to_string(),
        elements: industrial_elements.clone(),
        aa_groups: industrial_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&industrial_elements, &industrial_aa_groups, colors[0], colors[1]),
        gradient_start: colors[0],
        gradient_end: colors[1],
    });
    
    // Spring Sunset - Rich Electric Blue, Moonstone, Deep Peach, Pastel Red, Fuchsia Rose, Maximum Purple
    let colors = [
        Color::from_rgba(7, 148, 197, 255),   // Rich Electric Blue
        Color::from_rgba(53, 176, 201, 255),  // Moonstone
        Color::from_rgba(246, 202, 161, 255), // Deep Peach
        Color::from_rgba(251, 100, 110, 255), // Pastel Red
        Color::from_rgba(200, 62, 119, 255),  // Fuchsia Rose
        Color::from_rgba(110, 55, 113, 255),  // Maximum Purple
    ];
    let mut spring_elements = HashMap::new();
    spring_elements.insert("C".to_string(), colors[5]);
    spring_elements.insert("O".to_string(), colors[3]);
    spring_elements.insert("N".to_string(), colors[0]);
    spring_elements.insert("S".to_string(), colors[2]);
    spring_elements.insert("H".to_string(), colors[1]);
    spring_elements.insert("P".to_string(), colors[4]);
    
    let mut spring_aa_groups = HashMap::new();
    spring_aa_groups.insert("Hydrophobic".to_string(), colors[2]);
    spring_aa_groups.insert("Polar".to_string(), colors[1]);
    spring_aa_groups.insert("Positive".to_string(), colors[0]);
    spring_aa_groups.insert("Negative".to_string(), colors[4]);
    spring_aa_groups.insert("Glycine".to_string(), Color::from_rgba(240, 230, 220, 255));
    
    themes.push(Theme {
        name: "Spring Sunset".to_string(),
        elements: spring_elements.clone(),
        aa_groups: spring_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&spring_elements, &spring_aa_groups, colors[5], colors[3]),
        gradient_start: colors[5],
        gradient_end: colors[3],
    });
    
    // Alone With Thoughts - Deep Taupe, Rose Gold, Desert Sand, Jelly Bean Blue, Dark Slate Blue
    let colors = [
        Color::from_rgba(112, 77, 109, 255),  // Deep Taupe
        Color::from_rgba(183, 113, 123, 255), // Rose Gold
        Color::from_rgba(234, 191, 173, 255), // Desert Sand
        Color::from_rgba(51, 137, 158, 255),  // Jelly Bean Blue
        Color::from_rgba(68, 69, 150, 255),   // Dark Slate Blue
    ];
    let mut alone_elements = HashMap::new();
    alone_elements.insert("C".to_string(), colors[0]);
    alone_elements.insert("O".to_string(), colors[1]);
    alone_elements.insert("N".to_string(), colors[4]);
    alone_elements.insert("S".to_string(), colors[2]);
    alone_elements.insert("H".to_string(), colors[2]);
    alone_elements.insert("P".to_string(), colors[3]);
    
    let mut alone_aa_groups = HashMap::new();
    alone_aa_groups.insert("Hydrophobic".to_string(), colors[1]);
    alone_aa_groups.insert("Polar".to_string(), colors[2]);
    alone_aa_groups.insert("Positive".to_string(), colors[4]);
    alone_aa_groups.insert("Negative".to_string(), colors[3]);
    alone_aa_groups.insert("Glycine".to_string(), colors[2]);
    
    themes.push(Theme {
        name: "Alone With Thoughts".to_string(),
        elements: alone_elements.clone(),
        aa_groups: alone_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&alone_elements, &alone_aa_groups, colors[0], colors[3]),
        gradient_start: colors[0],
        gradient_end: colors[3],
    });
    
    // Naughty Generation X - Dark Cornflower Blue, Maximum Blue, Navajo White, Pantone Pink
    let colors = [
        Color::from_rgba(49, 63, 132, 255),   // Dark Cornflower Blue
        Color::from_rgba(84, 166, 196, 255),  // Maximum Blue
        Color::from_rgba(255, 223, 168, 255), // Navajo White
        Color::from_rgba(217, 74, 152, 255),  // Pantone Pink
    ];
    let mut naughty_elements = HashMap::new();
    naughty_elements.insert("C".to_string(), colors[0]);
    naughty_elements.insert("O".to_string(), colors[3]);
    naughty_elements.insert("N".to_string(), colors[1]);
    naughty_elements.insert("S".to_string(), colors[2]);
    naughty_elements.insert("H".to_string(), Color::from_rgba(230, 230, 230, 255));
    naughty_elements.insert("P".to_string(), colors[3]);
    
    let mut naughty_aa_groups = HashMap::new();
    naughty_aa_groups.insert("Hydrophobic".to_string(), colors[2]);
    naughty_aa_groups.insert("Polar".to_string(), colors[1]);
    naughty_aa_groups.insert("Positive".to_string(), colors[0]);
    naughty_aa_groups.insert("Negative".to_string(), colors[3]);
    naughty_aa_groups.insert("Glycine".to_string(), colors[2]);
    
    themes.push(Theme {
        name: "Naughty Generation X".to_string(),
        elements: naughty_elements.clone(),
        aa_groups: naughty_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&naughty_elements, &naughty_aa_groups, colors[0], colors[3]),
        gradient_start: colors[0],
        gradient_end: colors[3],
    });
    
    // Cleaning Wounds - Verdigris, Deep Champagne, China Pink, Midnight, Cosmic Cobalt
    let colors = [
        Color::from_rgba(81, 176, 173, 255),  // Verdigris
        Color::from_rgba(240, 217, 158, 255), // Deep Champagne
        Color::from_rgba(227, 102, 154, 255), // China Pink
        Color::from_rgba(122, 33, 127, 255),  // Midnight
        Color::from_rgba(53, 33, 128, 255),   // Cosmic Cobalt
    ];
    let mut cleaning_elements = HashMap::new();
    cleaning_elements.insert("C".to_string(), colors[4]);
    cleaning_elements.insert("O".to_string(), colors[2]);
    cleaning_elements.insert("N".to_string(), colors[0]);
    cleaning_elements.insert("S".to_string(), colors[1]);
    cleaning_elements.insert("H".to_string(), Color::from_rgba(220, 220, 220, 255));
    cleaning_elements.insert("P".to_string(), colors[3]);
    
    let mut cleaning_aa_groups = HashMap::new();
    cleaning_aa_groups.insert("Hydrophobic".to_string(), colors[1]);
    cleaning_aa_groups.insert("Polar".to_string(), colors[0]);
    cleaning_aa_groups.insert("Positive".to_string(), colors[4]);
    cleaning_aa_groups.insert("Negative".to_string(), colors[2]);
    cleaning_aa_groups.insert("Glycine".to_string(), Color::from_rgba(235, 235, 220, 255));
    
    themes.push(Theme {
        name: "Cleaning Wounds".to_string(),
        elements: cleaning_elements.clone(),
        aa_groups: cleaning_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&cleaning_elements, &cleaning_aa_groups, colors[4], colors[2]),
        gradient_start: colors[4],
        gradient_end: colors[2],
    });
    
    // Another World - Steel Blue, Arylide Yellow, Green Sheen, Maximum Blue, Pastel Gray, B'dazzled Blue
    let colors = [
        Color::from_rgba(49, 140, 175, 255),  // Steel Blue
        Color::from_rgba(229, 201, 109, 255), // Arylide Yellow
        Color::from_rgba(121, 184, 170, 255), // Green Sheen
        Color::from_rgba(77, 170, 197, 255),  // Maximum Blue
        Color::from_rgba(215, 201, 187, 255), // Pastel Gray
        Color::from_rgba(48, 90, 142, 255),   // B'dazzled Blue
    ];
    let mut another_elements = HashMap::new();
    another_elements.insert("C".to_string(), colors[5]);
    another_elements.insert("O".to_string(), Color::from_rgba(200, 100, 100, 255));
    another_elements.insert("N".to_string(), colors[0]);
    another_elements.insert("S".to_string(), colors[1]);
    another_elements.insert("H".to_string(), colors[4]);
    another_elements.insert("P".to_string(), colors[2]);
    
    let mut another_aa_groups = HashMap::new();
    another_aa_groups.insert("Hydrophobic".to_string(), colors[1]);
    another_aa_groups.insert("Polar".to_string(), colors[2]);
    another_aa_groups.insert("Positive".to_string(), colors[3]);
    another_aa_groups.insert("Negative".to_string(), Color::from_rgba(200, 100, 100, 255));
    another_aa_groups.insert("Glycine".to_string(), colors[4]);
    
    themes.push(Theme {
        name: "Another World".to_string(),
        elements: another_elements.clone(),
        aa_groups: another_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&another_elements, &another_aa_groups, colors[5], colors[1]),
        gradient_start: colors[5],
        gradient_end: colors[1],
    });
    
    // Fire Mountain - Dark Midnight Blue, Blue Sapphire, Sea Blue, Crayola's Yellow, Middle Red, Chinese Orange
    let colors = [
        Color::from_rgba(10, 58, 97, 255),    // Dark Midnight Blue
        Color::from_rgba(14, 86, 129, 255),   // Blue Sapphire
        Color::from_rgba(20, 104, 148, 255),  // Sea Blue
        Color::from_rgba(253, 232, 138, 255), // Crayola's Yellow
        Color::from_rgba(239, 148, 106, 255), // Middle Red
        Color::from_rgba(233, 104, 59, 255),  // Chinese Orange
    ];
    let mut fire_elements = HashMap::new();
    fire_elements.insert("C".to_string(), colors[0]);
    fire_elements.insert("O".to_string(), colors[5]);
    fire_elements.insert("N".to_string(), colors[2]);
    fire_elements.insert("S".to_string(), colors[3]);
    fire_elements.insert("H".to_string(), colors[4]);
    fire_elements.insert("P".to_string(), colors[5]);
    
    let mut fire_aa_groups = HashMap::new();
    fire_aa_groups.insert("Hydrophobic".to_string(), colors[4]);
    fire_aa_groups.insert("Polar".to_string(), colors[3]);
    fire_aa_groups.insert("Positive".to_string(), colors[1]);
    fire_aa_groups.insert("Negative".to_string(), colors[5]);
    fire_aa_groups.insert("Glycine".to_string(), Color::from_rgba(240, 240, 220, 255));
    
    themes.push(Theme {
        name: "Fire Mountain".to_string(),
        elements: fire_elements.clone(),
        aa_groups: fire_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&fire_elements, &fire_aa_groups, colors[0], colors[5]),
        gradient_start: colors[0],
        gradient_end: colors[5],
    });
    
    // Beautiful Sundowners - American Blue, Coral Reef, Jasmine, Middle Green Yellow, Myrtle Green
    let colors = [
        Color::from_rgba(55, 64, 99, 255),    // American Blue
        Color::from_rgba(255, 122, 102, 255), // Coral Reef
        Color::from_rgba(255, 217, 128, 255), // Jasmine
        Color::from_rgba(167, 190, 108, 255), // Middle Green Yellow
        Color::from_rgba(51, 105, 101, 255),  // Myrtle Green
    ];
    let mut sundowners_elements = HashMap::new();
    sundowners_elements.insert("C".to_string(), colors[0]);
    sundowners_elements.insert("O".to_string(), colors[1]);
    sundowners_elements.insert("N".to_string(), colors[4]);
    sundowners_elements.insert("S".to_string(), colors[2]);
    sundowners_elements.insert("H".to_string(), colors[3]);
    sundowners_elements.insert("P".to_string(), colors[1]);
    
    let mut sundowners_aa_groups = HashMap::new();
    sundowners_aa_groups.insert("Hydrophobic".to_string(), colors[2]);
    sundowners_aa_groups.insert("Polar".to_string(), colors[3]);
    sundowners_aa_groups.insert("Positive".to_string(), colors[4]);
    sundowners_aa_groups.insert("Negative".to_string(), colors[1]);
    sundowners_aa_groups.insert("Glycine".to_string(), Color::from_rgba(245, 245, 230, 255));
    
    themes.push(Theme {
        name: "Beautiful Sundowners".to_string(),
        elements: sundowners_elements.clone(),
        aa_groups: sundowners_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&sundowners_elements, &sundowners_aa_groups, colors[0], colors[1]),
        gradient_start: colors[0],
        gradient_end: colors[1],
    });
    
    // Bad For Economy - Deep Space Sparkle, Gunmetal, English Violet, Sugar Plum, Mauvelous, Bisque
    let colors = [
        Color::from_rgba(73, 98, 103, 255),   // Deep Space Sparkle
        Color::from_rgba(45, 43, 61, 255),    // Gunmetal
        Color::from_rgba(80, 63, 92, 255),    // English Violet
        Color::from_rgba(143, 83, 118, 255),  // Sugar Plum
        Color::from_rgba(250, 167, 167, 255), // Mauvelous
        Color::from_rgba(255, 228, 196, 255), // Bisque
    ];
    let mut bad_elements = HashMap::new();
    bad_elements.insert("C".to_string(), colors[1]);
    bad_elements.insert("O".to_string(), colors[4]);
    bad_elements.insert("N".to_string(), colors[0]);
    bad_elements.insert("S".to_string(), colors[5]);
    bad_elements.insert("H".to_string(), colors[5]);
    bad_elements.insert("P".to_string(), colors[3]);
    
    let mut bad_aa_groups = HashMap::new();
    bad_aa_groups.insert("Hydrophobic".to_string(), colors[5]);
    bad_aa_groups.insert("Polar".to_string(), colors[4]);
    bad_aa_groups.insert("Positive".to_string(), colors[2]);
    bad_aa_groups.insert("Negative".to_string(), colors[3]);
    bad_aa_groups.insert("Glycine".to_string(), colors[5]);
    
    themes.push(Theme {
        name: "Bad For Economy".to_string(),
        elements: bad_elements.clone(),
        aa_groups: bad_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&bad_elements, &bad_aa_groups, colors[1], colors[4]),
        gradient_start: colors[1],
        gradient_end: colors[4],
    });
    
    // Indian Drama - Pale Red-Violet, Purpureus, Pixie Powder, Royal Purple, Congo Pink, Topaz
    let colors = [
        Color::from_rgba(217, 109, 152, 255), // Pale Red-Violet
        Color::from_rgba(168, 60, 171, 255),  // Purpureus
        Color::from_rgba(51, 19, 135, 255),   // Pixie Powder
        Color::from_rgba(110, 69, 168, 255),  // Royal Purple
        Color::from_rgba(247, 124, 124, 255), // Congo Pink
        Color::from_rgba(255, 209, 133, 255), // Topaz
    ];
    let mut drama_elements = HashMap::new();
    drama_elements.insert("C".to_string(), colors[2]);
    drama_elements.insert("O".to_string(), colors[4]);
    drama_elements.insert("N".to_string(), colors[3]);
    drama_elements.insert("S".to_string(), colors[5]);
    drama_elements.insert("H".to_string(), colors[0]);
    drama_elements.insert("P".to_string(), colors[1]);
    
    let mut drama_aa_groups = HashMap::new();
    drama_aa_groups.insert("Hydrophobic".to_string(), colors[5]);
    drama_aa_groups.insert("Polar".to_string(), colors[0]);
    drama_aa_groups.insert("Positive".to_string(), colors[3]);
    drama_aa_groups.insert("Negative".to_string(), colors[4]);
    drama_aa_groups.insert("Glycine".to_string(), Color::from_rgba(240, 230, 240, 255));
    
    themes.push(Theme {
        name: "Indian Drama".to_string(),
        elements: drama_elements.clone(),
        aa_groups: drama_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&drama_elements, &drama_aa_groups, colors[2], colors[4]),
        gradient_start: colors[2],
        gradient_end: colors[4],
    });
    
    // Retro Base - Rose Gold, Fawn, Wintergreen Dream, Tuscany
    let colors = [
        Color::from_rgba(190, 114, 107, 255), // Rose Gold
        Color::from_rgba(228, 182, 110, 255), // Fawn
        Color::from_rgba(71, 147, 135, 255),  // Wintergreen Dream
        Color::from_rgba(180, 158, 145, 255), // Tuscany
    ];
    let mut retro_base_elements = HashMap::new();
    retro_base_elements.insert("C".to_string(), colors[3]);
    retro_base_elements.insert("O".to_string(), colors[0]);
    retro_base_elements.insert("N".to_string(), colors[2]);
    retro_base_elements.insert("S".to_string(), colors[1]);
    retro_base_elements.insert("H".to_string(), Color::from_rgba(230, 220, 210, 255));
    retro_base_elements.insert("P".to_string(), colors[0]);
    
    let mut retro_base_aa_groups = HashMap::new();
    retro_base_aa_groups.insert("Hydrophobic".to_string(), colors[1]);
    retro_base_aa_groups.insert("Polar".to_string(), colors[2]);
    retro_base_aa_groups.insert("Positive".to_string(), colors[3]);
    retro_base_aa_groups.insert("Negative".to_string(), colors[0]);
    retro_base_aa_groups.insert("Glycine".to_string(), Color::from_rgba(235, 225, 215, 255));
    
    themes.push(Theme {
        name: "Retro Base".to_string(),
        elements: retro_base_elements.clone(),
        aa_groups: retro_base_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&retro_base_elements, &retro_base_aa_groups, colors[3], colors[0]),
        gradient_start: colors[3],
        gradient_end: colors[0],
    });
    
    // String Of Holidays - Crayola's Forest Green, Deep Green-Cyan Turquoise, Deep Koamaru, Burnt Umber, Ochre, Dark Tangerine
    let colors = [
        Color::from_rgba(98, 158, 112, 255),  // Crayola's Forest Green
        Color::from_rgba(24, 120, 108, 255),  // Deep Green-Cyan Turquoise
        Color::from_rgba(44, 62, 94, 255),    // Deep Koamaru
        Color::from_rgba(140, 62, 31, 255),   // Burnt Umber
        Color::from_rgba(191, 117, 36, 255),  // Ochre
        Color::from_rgba(246, 169, 15, 255),  // Dark Tangerine
    ];
    let mut holidays_elements = HashMap::new();
    holidays_elements.insert("C".to_string(), colors[2]);
    holidays_elements.insert("O".to_string(), colors[3]);
    holidays_elements.insert("N".to_string(), colors[1]);
    holidays_elements.insert("S".to_string(), colors[5]);
    holidays_elements.insert("H".to_string(), colors[0]);
    holidays_elements.insert("P".to_string(), colors[4]);
    
    let mut holidays_aa_groups = HashMap::new();
    holidays_aa_groups.insert("Hydrophobic".to_string(), colors[4]);
    holidays_aa_groups.insert("Polar".to_string(), colors[0]);
    holidays_aa_groups.insert("Positive".to_string(), colors[1]);
    holidays_aa_groups.insert("Negative".to_string(), colors[3]);
    holidays_aa_groups.insert("Glycine".to_string(), Color::from_rgba(220, 220, 200, 255));
    
    themes.push(Theme {
        name: "String Of Holidays".to_string(),
        elements: holidays_elements.clone(),
        aa_groups: holidays_aa_groups.clone(),
        aa_types: generate_aa_types_from_palette(&holidays_elements, &holidays_aa_groups, colors[2], colors[5]),
        gradient_start: colors[2],
        gradient_end: colors[5],
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
