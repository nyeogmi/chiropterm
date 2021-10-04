use crate::{aliases::*, formatting::{FChar, FSem, Justification, Preformatter}, rendering::{Font, Interactor, InteractorFmt}};

mod align;
mod bevel;
mod boxart;
mod fill;
mod split;


pub trait Brushable {
    fn draw(&self, at: CellPoint, f: FSem);
    // TODO: "set cursor" function, can be a no op for types without a cursor

    fn brush_at(&self, rect: CellRect) -> Brush<'_> where Self: Sized {
        Brush { 
            underlying: self,
            rect,
            clip: rect,
            cursor_offset: CellVector::zero(),
            cursor: CellPoint::zero(),
            font: Font::Normal,

            fg: None,
            bg: None,

            // nyeo note: by default we actually overwrite interactors that might have been created by other draw ops
            // (why? because our text probably isn't related to their thing)
            interactor: Some(InteractorFmt::none()),  
        }
    }
}

pub struct Brush<'a> {
    underlying: &'a dyn Brushable,
    // NYEO NOTE: `rect` is the bounds that the outside world sees
    // `clip` is an inner set of boundaries enforced on the underlying object, 
    // in the underlying object's coord system
    rect: CellRect,
    clip: CellRect,
    cursor_offset: CellVector,
    pub cursor: CellPoint, 
    pub font: Font,

    fg: Option<u8>,
    bg: Option<u8>,
    interactor: Option<InteractorFmt>,
}

impl<'a> Clone for Brush<'a> {
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

impl<'a> Brush<'a> {
    pub fn at(&self, cursor: CellPoint) -> Self {
        let mut b = self.clone();
        b.cursor = cursor;
        b
    }

    pub fn rect(&self) -> CellRect { self.rect }
    pub fn clip(&self) -> CellRect { self.clip } 
    pub fn cursor_offset(&self) -> CellVector { self.cursor_offset }  

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

    pub fn color(&self, (bg, fg): (u8, u8)) -> Self {
        self.bg(bg).fg(fg)
    }

    // TODO: method to explicitly clear interactor? might be a good idea
    pub fn interactor(&self, interactor: Interactor, (bg, fg): (u8, u8)) -> Self {
        let mut b = self.clone();
        b.interactor = Some(InteractorFmt { interactor, bg, fg });
        b
    }

    pub fn no_interactor(&self) -> Self {
        let mut b = self.clone();
        b.interactor = Some(InteractorFmt::none());
        b
    }

    pub fn putfs(&self, s: &str) -> Self {
        // TODO: Justification? Probably should be a field
        let mut b = self.clone();

        let first_width_chars: Option<usize>;
        let next_width_chars: Option<usize>;

        let font_width = self.font.char_size().width;

        let first_width_cells = (b.rect.max_x() - self.cursor.x).max(0);
        let next_width_cells = b.rect.size.width;

        first_width_chars = Some((first_width_cells / font_width) as usize);
        next_width_chars = Some((next_width_cells / font_width) as usize);

        let pre = Preformatter {
            font: self.font,
            first_width_chars: first_width_chars,
            main_width_chars: next_width_chars,
            justification: Justification::Left,
        };
        pre.draw(s, &mut b);
        b
    }

    pub fn putch(&self, u: u16) -> Self {
        let mut b = self.clone();
        let font = b.font;
        let cursor = b.cursor;
        font.draw_char(cursor, FChar::new().sprite(u), &mut b);
        // TODO: Update cursor position?
        b
    }

    pub fn clipped(&self, r: CellRect) -> Self {
        let mut b = self.clone();
        b.clip = b.clip.intersection(&r.translate(b.cursor_offset)).unwrap_or(rect(0, 0, 0, 0));
        b
    }

    /// like clipped, but zeroed and affects `rect`
    // (and therefore visible to the outside world)
    pub fn region(&self, r: CellRect) -> Self {
        let mut b = self.clone();
        b.clip = b.clip.intersection(&r.translate(b.cursor_offset)).unwrap_or(rect(0, 0, 0, 0));
        b.cursor_offset += r.origin.to_vector();
        b.rect = CellRect::new(CellPoint::zero(), r.size);
        b.cursor = point2(0, 0);
        b
    }

    pub fn offset_rect(&self, offset: CellVector) -> Self {
        let mut b = self.clone();
        b.rect = b.rect.translate(offset);
        b.cursor_offset += offset;
        b
    }
}

impl<'a> Brushable for Brush<'a> {
    fn draw(&self, mut at: CellPoint, mut f: FSem) {
        at += self.cursor_offset;

        if !self.clip.contains(at) { 
            return; 
        }

        f.bg = f.bg.or(self.bg);
        f.fg = f.fg.or(self.fg);
        f.interactor = f.interactor.or(self.interactor);

        self.underlying.draw(at, f)
    }
}