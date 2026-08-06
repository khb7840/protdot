use crate::{
    animation::{AnimationMode, AnimationState},
    types::{ColorScheme, RenderMode},
    visualization_state::VisualizationState,
};

pub struct VisualizationPreset {
    pub key: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub color_scheme: ColorScheme,
    pub render_mode: RenderMode,
    pub radius_multiplier: f32,
    pub animation_enabled: bool,
    pub animation_mode: AnimationMode,
    pub animation_speed: f32,
}

pub const VISUALIZATION_PRESETS: [VisualizationPreset; 6] = [
    VisualizationPreset {
        key: "1",
        name: "Atom Detail",
        description: "Element-colored atoms for chemistry-first inspection.",
        color_scheme: ColorScheme::ByElement,
        render_mode: RenderMode::PerAtom,
        radius_multiplier: 1.0,
        animation_enabled: false,
        animation_mode: AnimationMode::None,
        animation_speed: 1.0,
    },
    VisualizationPreset {
        key: "2",
        name: "Chemistry Groups",
        description: "Residue groups with a calm spin for quick functional clustering.",
        color_scheme: ColorScheme::ByAminoAcidGroup,
        render_mode: RenderMode::PerResidue,
        radius_multiplier: 1.25,
        animation_enabled: true,
        animation_mode: AnimationMode::RotateY,
        animation_speed: 1.2,
    },
    VisualizationPreset {
        key: "3",
        name: "Residue Identity",
        description: "Residue-type colors with a gentle wobble to spot motifs.",
        color_scheme: ColorScheme::ByAminoAcidType,
        render_mode: RenderMode::PerResidue,
        radius_multiplier: 1.15,
        animation_enabled: true,
        animation_mode: AnimationMode::Wobble,
        animation_speed: 1.5,
    },
    VisualizationPreset {
        key: "4",
        name: "Sequence Flow",
        description: "N→C gradient and orbit motion for pathway storytelling.",
        color_scheme: ColorScheme::NToCGradient,
        render_mode: RenderMode::PerResidue,
        radius_multiplier: 1.2,
        animation_enabled: true,
        animation_mode: AnimationMode::Orbit,
        animation_speed: 0.9,
    },
    VisualizationPreset {
        key: "5",
        name: "Chain Contrast",
        description: "Chain-aware colors to separate partners in complexes.",
        color_scheme: ColorScheme::RandomChain,
        render_mode: RenderMode::PerResidue,
        radius_multiplier: 1.1,
        animation_enabled: true,
        animation_mode: AnimationMode::RotateXY,
        animation_speed: 1.1,
    },
    VisualizationPreset {
        key: "6",
        name: "Theme Spotlight",
        description: "Theme palettes and mappings for polished screenshots and demos.",
        color_scheme: ColorScheme::Theme,
        render_mode: RenderMode::PerAtom,
        radius_multiplier: 0.95,
        animation_enabled: true,
        animation_mode: AnimationMode::Tumble,
        animation_speed: 1.4,
    },
];

pub fn apply_visualization_preset(
    preset_index: usize,
    vis: &mut VisualizationState,
    anim: &mut AnimationState,
) {
    let Some(preset) = VISUALIZATION_PRESETS.get(preset_index) else {
        return;
    };

    vis.color_scheme = preset.color_scheme;
    vis.render_mode = preset.render_mode;
    vis.radius_scale = VisualizationState::default_radius_scale() * preset.radius_multiplier;

    anim.enabled = preset.animation_enabled;
    anim.mode = preset.animation_mode;
    anim.speed = preset.animation_speed;
    anim.time = 0.0;
    anim.reverse = false;
}

pub fn active_visualization_preset(
    vis: &VisualizationState,
    anim: &AnimationState,
) -> Option<usize> {
    VISUALIZATION_PRESETS.iter().position(|preset| {
        vis.color_scheme == preset.color_scheme
            && vis.render_mode == preset.render_mode
            && (vis.radius_scale
                - (VisualizationState::default_radius_scale() * preset.radius_multiplier))
                .abs()
                < 0.01
            && anim.enabled == preset.animation_enabled
            && anim.mode == preset.animation_mode
            && (anim.speed - preset.animation_speed).abs() < 0.01
            && !anim.reverse
    })
}
