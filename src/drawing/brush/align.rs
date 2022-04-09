use euclid::Rect;
use euclid::point2;
use euclid::size2;

use crate::Brush;


impl <'a> Brush<'a> {
    pub fn reshape_for_font(&self) -> Brush<'a> {
        let sz = self.clip.size;
        let align_sz_x = sz.width - sz.width % self.font.char_size().width;
        let align_sz_y = sz.height - sz.height % self.font.char_size().height;
        self.region(Rect::new(
            point2(0, 0),
            size2(align_sz_x, align_sz_y),
        ))
    }
}