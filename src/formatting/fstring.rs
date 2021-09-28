use crate::rendering::{Interactor, SemanticContent};

pub struct FString(pub Vec<FChar>);

#[derive(Clone, Copy)]
pub struct FChar {
    pub sprite: Option<u16>,
    pub bg: Option<u8>,
    pub fg: Option<u8>,
    pub interactor: Option<Interactor>,
}

impl FChar {
    pub(crate) fn sem(&self, semantic: impl Fn(u16) -> SemanticContent) -> FSem {
        FSem { 
            sem: self.sprite.map(semantic),
            bg: self.bg,
            fg: self.fg,
            interactor: self.interactor,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FSem {
    pub sem: Option<SemanticContent>,
    pub bg: Option<u8>,
    pub fg: Option<u8>,
    pub interactor: Option<Interactor>,
}

impl FSem {
    pub fn superimposed_on(self, below: FSem) -> FSem {
        FSem { 
            sem: self.sem.or(below.sem), 
            bg: self.bg.or(below.bg),
            fg: self.fg.or(below.fg),
            interactor: self.interactor.or(below.interactor),
        }
    }
}
