pub struct CellContent {
    pub fg: u32,
    pub bg: u32,
    pub sem: SemanticContent,
}

pub enum SemanticContent {
    TopHalf(u16),
    BottomHalf(u16),
    // TODO: Double-wides
}