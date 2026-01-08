#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenderMode {
    PerAtom,
    PerResidue,
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
