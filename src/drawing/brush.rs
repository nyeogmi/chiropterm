use crate::{aliases::*, formatting::{FSem, Justification, Preformatter}, rendering::{Font, Interactor}};

use gridd_euclid::PointsIn;


pub trait Brushable: Sized {
    fn draw(&self, at: CellPoint, f: FSem);
    // TODO: "set cursor" function, can be a no op for types without a cursor

    fn brush_at(&self, rect: CellRect) -> Brush<'_, Self> {
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
            interactor: Some(Interactor::none()),  
        }
    }
}

pub struct Brush<'a, B: Brushable> {
    underlying: &'a B,
    // NYEO NOTE: `rect` is the bounds that the outside world sees
    // `clip` is an inner set of boundaries enforced on the underlying object, 
    // in the underlying object's coord system
    rect: CellRect,
    clip: CellRect,
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
        pre.to_brush(s, &mut b);
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
        b.cursor_offset = r.origin.to_vector();
        b.rect = CellRect::new(CellPoint::zero(), r.size);
        b
    }
}

impl<'a, B: Brushable> Brushable for Brush<'a, B> {
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


// more drawing ops!
impl <'a, B: Brushable> Brush<'a, B> {
    pub fn fill(&self, f: FSem) {
        for i in isize::points_in(self.rect) {
            self.draw(i, f)
        }
    }

    pub fn bevel_w95(&self, top_left: u8, bottom_right: u8) {
        // shorthand for w95-style bevels
        self.bevel_top(top_left);
        self.bevel_left(top_left);
        self.bevel_right(bottom_right);
        self.bevel_bottom(bottom_right);
    }

    pub fn bevel_w95_sleek(&self, left: u8, right: u8) {
        // w95-style bevels with no top or bottom
        self.bevel_left(left);
        self.bevel_right(right);
    }

    pub fn bevel_top(&self, color: u8) {
        if self.rect.height() == 0 { return; }

        let mut sem = FSem::new();
        sem.bevels.top = Some(color);

        for x in self.rect.min_x()..self.rect.max_x() {
            self.draw(point2(x, self.rect.min_y()), sem);
        }
    }

    pub fn bevel_left(&self, color: u8) {
        if self.rect.width() == 0 { return; }

        let mut sem = FSem::new();
        sem.bevels.left = Some(color);

        for y in self.rect.min_y()..self.rect.max_y() {
            self.draw(point2(self.rect.min_x(), y), sem);
        }
    }

    pub fn bevel_right(&self, color: u8) {
        if self.rect.width() == 0 { return; }

        let mut sem = FSem::new();
        sem.bevels.right = Some(color);

        for y in self.rect.min_y()..self.rect.max_y() {
            self.draw(point2(self.rect.max_x() - 1, y), sem);
        }
    }

    pub fn bevel_bottom(&self, color: u8) {
        if self.rect.height() == 0 { return; }

        let mut sem = FSem::new();
        sem.bevels.bottom = Some(color);

        for x in self.rect.min_x()..self.rect.max_x() {
            self.draw(point2(x, self.rect.max_y() - 1), sem);
        }
    }
}