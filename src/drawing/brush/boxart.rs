use euclid::{Rect, size2, point2};

use crate::{BoxArt, Brush};

impl <'a> Brush<'a> {
    pub fn draw_box(&self, double_border: bool) {
        let mut boxart = BoxArt::new();
        let r_sz = self.clip.size;
        let c_sz = self.font.char_size();
        let rect = Rect::new(
            point2(0, 0),
            size2(
                r_sz.width / c_sz.width, 
                r_sz.height / c_sz.height
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