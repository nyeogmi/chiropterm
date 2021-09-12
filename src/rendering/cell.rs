pub struct CellContent {
    pub fg: u32,
    pub bg: u32,
    pub sem: SemanticContent,
}

pub enum SemanticContent {
    Small(u16), 

    TopHalf(u16),
    BottomHalf(u16),

    SetTL(u16),
    SetTR(u16),
    SetBL(u16),
    SetBR(u16),

    FatTL(u16),
    FatTR(u16),
    FatBL(u16),
    FatBR(u16),
    // TODO: Double-wides
}