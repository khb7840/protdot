#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenderMode {
    PerAtom,
    PerResidue,
}

impl RenderMode {
    pub fn from_index(index: u32) -> Option<Self> {
        match index {
            0 => Some(Self::PerAtom),
            1 => Some(Self::PerResidue),
            _ => None,
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
    pub fn from_index(index: u32) -> Option<Self> {
        match index {
            0 => Some(Self::ByElement),
            1 => Some(Self::ByAminoAcidGroup),
            2 => Some(Self::ByAminoAcidType),
            3 => Some(Self::NToCGradient),
            4 => Some(Self::RandomChain),
            5 => Some(Self::Theme),
            _ => None,
        }
    }
}
