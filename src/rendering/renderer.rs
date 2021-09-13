use crate::{drawing::Screen};
use crate::geom::PointsIn;

use crate::window_management::Aspect;

use super::{Swatch};

pub struct Render<'a> {
    pub frame: u64,
    pub aspect: Aspect,
    pub buffer: &'a mut Vec<u32>,
    pub swatch: Swatch,
    pub screen: &'a Screen,
}


impl<'a> Render<'a> {
    pub fn draw(&mut self) {
        let screen_rect = self.screen.rect();
        let term_rect = self.aspect.term_rect();
        assert_eq!(screen_rect, term_rect.cast());

        for term_xy in u16::points_in(term_rect) {
            let content = self.screen.cells.get(term_xy.cast()).unwrap().get();
            let tile = super::font::eval(content.sem);

            tile.render(
                self.buffer, term_xy.x, term_xy.y, self.aspect.term_size.width, 
                self.swatch.get(content.fg), self.swatch.get(content.bg)
            );
        }
    }
}