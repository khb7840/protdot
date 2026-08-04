#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenderMode {
    PerAtom,
    PerResidue,
}

impl RenderMode {
    pub fn label(self) -> &'static str {
        match self {
            RenderMode::PerAtom => "Per Atom",
            RenderMode::PerResidue => "Per Residue",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorScheme {
    ByElement,
    ByAminoAcidGroup,
    ByAminoAcidType,
    NToCGradient,
    RandomChain,
    Theme,
}

impl ColorScheme {
    pub fn label(self) -> &'static str {
        match self {
            ColorScheme::ByElement => "By Element",
            ColorScheme::ByAminoAcidGroup => "By AA Group",
            ColorScheme::ByAminoAcidType => "By AA Type",
            ColorScheme::NToCGradient => "N→C Gradient",
            ColorScheme::RandomChain => "By Chain",
            ColorScheme::Theme => "Theme",
        }
    }
}
