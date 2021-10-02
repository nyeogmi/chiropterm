use euclid::{Rect, size2};

use crate::{BoxArt, Brush};

impl <'a> Brush<'a> {
    pub fn draw_box(&self, double_border: bool) {
        let mut boxart = BoxArt::new();
        let rect = self.rect();
        let sz = self.font.char_size();
        let rect = Rect::new(
            rect.min(), 
            size2(
                rect.size.width / sz.width, 
                rect.size.height / sz.height
            )
        );
        boxart.draw_box(rect, double_border);
        boxart.draw(self);
    }

    pub fn draw_boxart(&self, f: impl FnOnce(&mut BoxArt)) {
        let mut boxart = BoxArt::new();
        f(&mut boxart);
        boxart.draw(self);
    }
}