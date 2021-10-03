#[derive(Clone, Copy, Debug)]
pub struct CellContent {
    pub bg: u8,
    pub fg: u8,

    // 255: no bevel
    pub bevels: Bevels,

    pub sem: SemanticContent,
    pub interactor: InteractorFmt, // interactor, mouseover color
}

#[derive(Clone, Copy, Debug)]
pub enum SemanticContent {
    Blank,
    Small(u16), 
    SmallPizza1(u16, u16), SmallPizza2(u16, u16), // two small tiles adjacent, divided by a diagonal line

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


#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Interactor(usize);

impl Interactor {
    pub fn none() -> Interactor { Interactor(!0usize) }

    pub(crate) fn from_index(ix: usize) -> Interactor {
        assert_ne!(ix, !0usize);
        Interactor(ix)
    }

    pub(crate) fn index(&self) -> Option<usize> {
        if self.0 == !0usize {
            return None
        }
        Some(self.0)
    }

    // TODO: Create one with a specific ID
}

#[derive(Clone, Copy, Debug)]
pub struct InteractorFmt {
    pub interactor: Interactor,
    pub bg: u8,
    pub fg: u8,
}

impl InteractorFmt {
    pub fn none() -> InteractorFmt { 
        InteractorFmt {
            interactor: Interactor::none(),
            fg: 255,
            bg: 255,
        }
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Bevels {
    // 255: no bevel
    pub top: u8,
    pub left: u8,
    pub right: u8,
    pub bottom: u8,
}

impl Bevels {
    pub fn new() -> Bevels {
        Bevels { 
            top: 255,
            left: 255,
            right: 255,
            bottom: 255,
        }
    }

}