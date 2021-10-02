use crate::{BoxArt, Brush};

impl <'a> Brush<'a> {
    pub fn draw_box(&self, double_border: bool) {
        let mut boxart = BoxArt::new();
        boxart.draw_box(self.rect().inflate(-1, -1), double_border);
        boxart.draw(self);
    }

    pub fn draw_boxart(&self, f: impl FnOnce(&mut BoxArt)) {
        let mut boxart = BoxArt::new();
        f(&mut boxart);
        boxart.draw(self);
    }
}