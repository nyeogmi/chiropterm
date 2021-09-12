mod cell;
mod font;
mod sprite;

use euclid::{Point2D, Rect};

use crate::geom::PointsIn;

use crate::spaces::CellSpace;
use crate::window_management::Aspect;

use self::cell::{CellContent, SemanticContent};

pub struct Render<'a> {
    pub frame: u64,
    pub aspect: Aspect,
    pub buffer: &'a mut Vec<u32>,
}


impl<'a> Render<'a> {
    pub fn draw(&mut self) {
        for term_xy in u16::points_in(self.aspect.term_rect()) {
            let content = self.cell_at(term_xy);
            let sprite = font::eval(content.sem);

            let cell = self.aspect.buf_cell_area(term_xy);

            let src = Rect::new(Point2D::zero(), cell.size);
            let dest = cell;

            for (s, d) in u16::points_in(src).zip(u16::points_in(dest)) {
                let buffer_ix = d.y as usize * self.aspect.buf_size.width as usize + d.x as usize;

                if sprite.pixel(s) {
                    self.buffer[buffer_ix] = content.fg;
                } else {
                    self.buffer[buffer_ix] = content.bg;
                }
            }
        }
    }

    pub fn cell_at(&self, xy: Point2D<u16, CellSpace>) -> CellContent {
        let x = xy.x;
        let y = xy.y / 2;
        let is_top = xy.y % 2 == 0;

        CellContent {
            bg: 0x00000000,
            fg: 0x00ffffff,
            sem: if is_top {
                SemanticContent::TopHalf(x + y * 16)
            } else {
                SemanticContent::BottomHalf(x + y * 16)
            }
        }
    }
}