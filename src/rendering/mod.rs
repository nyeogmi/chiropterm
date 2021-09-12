mod cell;
mod font;
mod sprite;

use euclid::{Point2D};

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
            let tile = font::eval(content.sem);

            tile.render(self.buffer, term_xy.x, term_xy.y, self.aspect.term_size.width, content.fg, content.bg);
        }
    }

    pub fn cell_at(&self, xy: Point2D<u16, CellSpace>) -> CellContent {
        /*
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
        */
        let x = xy.x / 2;
        let y = xy.y / 2;
        let is_left = xy.x % 2 == 0;
        let is_top = xy.y % 2 == 0;

        CellContent {
            bg: 0x00000000,
            fg: 0x00ffffff,
            sem: if is_top {
                if is_left {
                    SemanticContent::FatTL(x + y * 16)
                } else {
                    SemanticContent::FatTR(x + y * 16)
                }
            } else {
                if is_left {
                    SemanticContent::FatBL(x + y * 16)
                } else {
                    SemanticContent::FatBR(x + y * 16)
                }
            }
        }
    }
}