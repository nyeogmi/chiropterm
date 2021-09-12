use crate::rendering::SemanticContent;

pub struct FString(pub Vec<FChar>);

#[derive(Clone, Copy)]
pub struct FChar {
    pub sprite: Option<u16>,
    pub bg: Option<u8>,
    pub fg: Option<u8>,
}

impl FChar {
    pub fn superimposed_on(self, below: FChar) -> FChar {
        FChar { 
            sprite: self.sprite.or(below.sprite), 
            bg: self.bg.or(below.bg),
            fg: self.fg.or(below.fg),
        }
    }
}

#[derive(Clone, Copy)]
pub struct FSem {
    pub sem: Option<SemanticContent>,
    pub bg: Option<u32>,
    pub fg: Option<u32>,
}

impl FSem {
    pub fn superimposed_on(self, below: FSem) -> FSem {
        FSem { 
            sem: self.sem.or(below.sem), 
            bg: self.bg.or(below.bg),
            fg: self.fg.or(below.fg),
        }
    }
}
