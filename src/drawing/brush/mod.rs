use crate::{aliases::*, formatting::{FChar, FSem, Justification, Preformatter}, rendering::{Font, Interactor, InteractorFmt, font}};

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
            clip: rect,
            cursor: CellPoint::zero(),
            font: Font::Normal,

            fg: None,
            bg: None,

            // nyeo note: by default we actually overwrite interactors that might have been created by other draw ops
            // (why? because our text probably isn't related to their thing)
            interactor: Some(InteractorFmt::none()),  
            scroll_interactor: None,
        }
    }
}

pub struct Brush<'a> {
    underlying: &'a dyn Brushable,
    // NYEO NOTE: `rect` is the bounds that the outside world sees
    // `clip` is an inner set of boundaries enforced on the underlying object, 
    // in the underlying object's coord system
    clip: CellRect,
    pub cursor: CellPoint, 
    pub font: Font,

    fg: Option<u8>,
    bg: Option<u8>,
    interactor: Option<InteractorFmt>,
    scroll_interactor: Option<Interactor>,
}

impl<'a> Clone for Brush<'a> {
    fn clone(&self) -> Self {
        Self { 
            underlying: self.underlying.clone(), 
            clip: self.clip.clone(), 
            cursor: self.cursor.clone(), 
            font: self.font,
            
            fg: self.fg.clone(), 
            bg: self.bg.clone(),
            interactor: self.interactor.clone(),
            scroll_interactor: self.scroll_interactor.clone(),
        }
    }
}

impl<'a> Brush<'a> {
    pub fn at(&self, cursor: CellPoint) -> Self {
        let mut b = self.clone();
        b.cursor = cursor;
        b
    }

    pub fn size(&self) -> CellSize { self.clip.size }

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
    pub fn dont_interfere_with_interactor(&self) -> Self {
        let mut b = self.clone();
        b.interactor = None;
        b
    }

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

    pub fn scroll_interactor(&self, interactor: Interactor) -> Self {
        let mut b = self.clone();
        b.scroll_interactor = Some(interactor);
        b
    }

    pub fn no_scroll_interactor(&self) -> Self {
        let mut b = self.clone();
        b.scroll_interactor = Some(Interactor::none());
        b
    }

    pub fn putfs(&self, s: &str) -> Self {
        println!("starting putfs: {}", self.cursor.x);
        let mut b = self.clone();

        let font_width = self.font.char_size().width;

        let pre = Preformatter {
            font: self.font,
            indent: self.cursor.x / font_width,
            first_line_fractional: self.cursor.x % font_width + 1,
            width: Some((b.size().width / font_width) as usize),
            justification: Justification::Left,
        };
        pre.draw(s, &mut b);
        b
    }

    pub fn putch(&self, u: impl Into<u16>) -> Self {
        let mut b = self.clone();
        let font = b.font;
        let cursor = b.cursor;
        font.draw_char(cursor, FChar::new().sprite(u.into()), &mut b);
        // TODO: Update cursor position?
        b
    }

    pub fn region(&self, r: CellRect) -> Self {
        let mut b = self.clone();
        b.clip = b.clip.intersection(&r.translate(b.clip.origin.to_vector())).unwrap_or(rect(0, 0, 0, 0));
        b.cursor = point2(0, 0);
        b
    }
}

impl<'a> Brushable for Brush<'a> {
    fn draw(&self, mut at: CellPoint, mut f: FSem) {
        at += self.clip.origin.to_vector();

        if !self.clip.contains(at) { 
            return; 
        }

        f.bg = f.bg.or(self.bg);
        f.fg = f.fg.or(self.fg);
        f.interactor = f.interactor.or(self.interactor);
        f.scroll_interactor = f.scroll_interactor.or(self.scroll_interactor);

        self.underlying.draw(at, f)
    }
}