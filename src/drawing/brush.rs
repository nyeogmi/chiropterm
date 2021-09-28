use crate::{aliases::*, formatting::{FSem, Justification, Preformatter}, rendering::{Font, Interactor}};


pub trait Brushable: Sized {
    fn draw(&self, at: CellPoint, f: FSem);
    // TODO: "set cursor" function, can be a no op for types without a cursor

    fn brush(&self) -> Brush<'_, Self> {
        Brush { 
            underlying: self,
            rect: None,
            clip: None,
            cursor_offset: CellVector::zero(),
            cursor: CellPoint::zero(),
            font: Font::Normal,

            fg: None,
            bg: None,

            // nyeo note: by default we actually overwrite interactors that might have been created by other draw ops
            // (why? because our text probably isn't related to their thing)
            interactor: Some(Interactor::none()),  
        }
    }
}

pub struct Brush<'a, B: Brushable> {
    underlying: &'a B,
    // NYEO NOTE: `rect` is the bounds that the outside world sees
    // `clip` is an inner set of boundaries enforced on the underlying object, 
    // in the underlying object's coord system
    rect: Option<CellRect>,
    clip: Option<CellRect>,
    cursor_offset: CellVector,
    pub cursor: CellPoint, 
    font: Font,

    fg: Option<u8>,
    bg: Option<u8>,
    interactor: Option<Interactor>,
}

impl<'a, B: Brushable> Clone for Brush<'a, B> {
    fn clone(&self) -> Self {
        Self { 
            underlying: self.underlying.clone(), 
            rect: self.rect.clone(), 
            clip: self.clip.clone(), 
            cursor_offset: self.cursor_offset.clone(), 
            cursor: self.cursor.clone(), 
            font: self.font,
            
            fg: self.fg.clone(), 
            bg: self.bg.clone(),
            interactor: self.interactor.clone(),
        }
    }
}

impl<'a, B: Brushable> Brush<'a, B> {
    pub fn at(&self, cursor: CellPoint) -> Self {
        let mut b = self.clone();
        b.cursor = cursor;
        b
    }

    pub fn shift(&self, amt: CellVector) -> Self {
        let mut b = self.clone();
        b.cursor += amt;
        b
    }

    pub fn on_newline(&self) -> Self {
        let mut b = self.clone();
        if b.cursor.x != 0 {
            b.cursor.y += self.font.char_size().height;
            b.cursor.x = 0;
        }
        b
    }

    pub fn font(&self, font: Font) -> Self {
        let mut b = self.clone();
        b.font = font;
        b
    }

    pub fn bg(&self, bg: u8) -> Self {
        let mut b = self.clone();
        b.bg = Some(bg);
        b
    }

    pub fn fg(&self, fg: u8) -> Self {
        let mut b = self.clone();
        b.fg = Some(fg);
        b
    }

    // TODO: method to explicitly clear interactor? might be a good idea
    pub fn interactor(&self, interactor: Interactor) -> Self {
        let mut b = self.clone();
        b.interactor = Some(interactor);
        b
    }

    pub fn no_interactor(&self) -> Self {
        self.interactor(Interactor::none())
    }

    pub fn putfs(&self, s: &str) -> Self {
        // TODO: Justification? Probably should be a field
        let mut b = self.clone();

        let first_width_chars: Option<usize>;
        let next_width_chars: Option<usize>;

        let font_width = self.font.char_size().width;

        if let Some(b) = b.rect {
            let first_width_cells = (b.max_x() - self.cursor.x).max(0);
            let next_width_cells = b.size.width;

            first_width_chars = Some((first_width_cells / font_width) as usize);
            next_width_chars = Some((next_width_cells / font_width) as usize);
        } else {
            first_width_chars = None;
            next_width_chars = None;
        }

        let pre = Preformatter {
            font: self.font,
            first_width_chars: first_width_chars,
            main_width_chars: next_width_chars,
            justification: Justification::Left,
        };
        pre.to_brush(s, &mut b);
        b
    }

    pub fn clipped(&self, r: CellRect) -> Self {
        let mut b = self.clone();
        b.clip = Some(match b.clip {
            Some(clip) => 
                clip.intersection(&r.translate(b.cursor_offset)).unwrap_or(rect(0, 0, 0, 0)),
            None => 
                r.translate(b.cursor_offset),
        });
        b
    }

    /// like clipped, but zeroed and affects `rect`
    // (and therefore visible to the outside world)
    pub fn region(&self, r: CellRect) -> Self {
        let mut b = self.clone();
        b.clip = Some(match b.clip {
            Some(clip) => 
                clip.intersection(&r.translate(b.cursor_offset)).unwrap_or(rect(0, 0, 0, 0)),
            None => 
                r.translate(b.cursor_offset),
        });
        b.cursor_offset = r.origin.to_vector();
        b.rect = Some(CellRect::new(CellPoint::zero(), r.size));
        b
    }
}

impl<'a, B: Brushable> Brushable for Brush<'a, B> {
    fn draw(&self, mut at: CellPoint, mut f: FSem) {
        at += self.cursor_offset;

        if let Some(cl) = self.clip {
            if !cl.contains(at) { 
                return; 
            }
        }

        f.bg = f.bg.or(self.bg);
        f.fg = f.fg.or(self.fg);
        f.interactor = f.interactor.or(self.interactor);

        self.underlying.draw(at, f)
    }
}