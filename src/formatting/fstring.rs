use crate::rendering::{Bevels, Interactor, InteractorFmt, SemanticContent};

pub struct FString(pub Vec<FChar>);

#[derive(Clone, Copy)]
pub struct FChar {
    pub sprite: Option<u16>,
    pub bg: Option<u8>,
    pub fg: Option<u8>,
    pub interactor: Option<InteractorFmt>,
    pub bevels: FBevels,
}

impl FChar {
    pub(crate) fn new() -> FChar {
        FChar { 
            sprite: None,
            bg: None,
            fg: None,
            interactor: None,
            bevels: FBevels::new(),
        }
    }


    pub(crate) fn sem(&self, semantic: impl Fn(u16) -> SemanticContent) -> FSem {
        FSem { 
            sem: self.sprite.map(semantic),
            bg: self.bg,
            fg: self.fg,
            interactor: self.interactor,
            bevels: self.bevels,
        }
    }

    pub(crate) fn sprite(mut self, u: u16) -> FChar {
        self.sprite = Some(u);
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FSem {
    pub sem: Option<SemanticContent>,
    pub bg: Option<u8>,
    pub fg: Option<u8>,
    pub interactor: Option<InteractorFmt>, // if None, then don't _change_ the interactor
    pub bevels: FBevels,
}

impl FSem {
    pub fn new() -> FSem {
        FSem {
            sem: None,
            bg: None,
            fg: None,
            interactor: None,
            bevels: FBevels::new(),
        }
    }

    pub fn superimposed_on(self, below: FSem) -> FSem {
        FSem { 
            sem: self.sem.or(below.sem), 
            bg: self.bg.or(below.bg),
            fg: self.fg.or(below.fg),
            interactor: self.interactor.or(below.interactor),
            bevels: self.bevels.superimposed_on(below.bevels)
        }
    }

    pub fn sem(mut self, sem: SemanticContent) -> FSem {
        self.sem = Some(sem);
        self
    }

    pub fn bg(mut self, bg: u8) -> FSem {
        self.bg = Some(bg);
        self
    }

    pub fn fg(mut self, fg: u8) -> FSem {
        self.fg = Some(fg);
        self
    }

    pub fn color(mut self, (bg, fg): (u8, u8)) -> FSem {
        self.bg = Some(bg);
        self.fg = Some(fg);
        self
    }

    pub fn interactor(mut self, interactor: Interactor, bg: u8, fg: u8) -> FSem {
        self.interactor = Some(InteractorFmt { interactor, bg, fg });
        self
    }

    // TODO: Don't set bevels here? Probably use rectangle drawing etc and set internally
}

#[derive(Clone, Copy, Debug)]
pub struct FBevels {
    pub top: Option<u8>,
    pub left: Option<u8>,
    pub right: Option<u8>,
    pub bottom: Option<u8>,
}
impl FBevels {
    pub(crate) fn new() -> FBevels {
        FBevels { top: None, left: None, right: None, bottom: None }
    }

    fn superimposed_on(&self, mut bevels: FBevels) -> FBevels {
        if let Some(t) = self.top { bevels.top = Some(t) }
        if let Some(l) = self.left { bevels.left = Some(l) }
        if let Some(r) = self.right { bevels.right = Some(r) }
        if let Some(b) = self.bottom { bevels.bottom = Some(b) }
        bevels
    }

    pub(crate) fn update(&self, bevels: &mut Bevels) {
        if let Some(t) = self.top { bevels.top = t }
        if let Some(l) = self.left { bevels.left = l }
        if let Some(r) = self.right { bevels.right = r }
        if let Some(b) = self.bottom { bevels.bottom = b; }
    }
}