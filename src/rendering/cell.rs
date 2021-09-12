#[derive(Clone, Copy, Debug)]
pub struct CellContent {
    pub fg: u8,
    pub bg: u8,
    pub sem: SemanticContent,
}

#[derive(Clone, Copy, Debug)]
pub enum SemanticContent {
    Blank,
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