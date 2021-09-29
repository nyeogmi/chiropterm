use crate::{drawing::Screen};
use gridd_euclid::PointsIn;

use crate::window_management::Aspect;

use super::{Interactor, Swatch};

pub struct Render<'a> {
    pub frame: u64,
    pub aspect: Aspect,
    pub buffer: &'a mut Vec<u32>,
    pub swatch: Swatch,
    pub screen: &'a Screen,
}


impl<'a> Render<'a> {
    pub fn draw(&mut self, interactor: Interactor) {
        let screen_rect = self.screen.rect();
        let term_rect = self.aspect.term_rect();
        assert_eq!(screen_rect, term_rect.cast());

        for term_xy in u16::points_in(term_rect) {
            let content = self.screen.cells.get(term_xy.cast()).unwrap().get();
            let interacting_here = if interactor == Interactor::none() { false } else { content.interactor == interactor };

            let tile = super::font::eval(content.sem);

            let fg: u8;
            let bg: u8;
            if interacting_here {
                // flash!
                fg = content.bg;
                bg = content.fg;
            } else {
                fg = content.fg;
                bg = content.bg;
            }

            tile.render(
                self.buffer, term_xy.x, term_xy.y, self.aspect.term_size.width, 
                self.swatch.get(fg), self.swatch.get(bg), 

                !(interacting_here || content.bevels.top == 255), 
                self.swatch.get(content.bevels.top),

                !(interacting_here || content.bevels.left == 255), 
                self.swatch.get(content.bevels.left),

                !(interacting_here || content.bevels.right == 255), 
                self.swatch.get(content.bevels.right),

                !(interacting_here || content.bevels.bottom == 255), 
                self.swatch.get(content.bevels.bottom),
            );
        }
    }
}