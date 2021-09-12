use euclid::{Point2D};

use crate::{aliases::CellSpace, geom::PointsIn};

use crate::window_management::Aspect;

use super::cell::{CellContent, SemanticContent};

pub struct Render<'a> {
    pub frame: u64,
    pub aspect: Aspect,
    pub buffer: &'a mut Vec<u32>,
}


impl<'a> Render<'a> {
    pub fn draw(&mut self) {
        for term_xy in u16::points_in(self.aspect.term_rect()) {
            let content = self.cell_at(term_xy);
            let tile = super::font::eval(content.sem);

            tile.render(self.buffer, term_xy.x, term_xy.y, self.aspect.term_size.width, content.fg, content.bg);
        }
    }

    pub fn cell_at(&self, xy: Point2D<u16, CellSpace>) -> CellContent {
        if xy.y < 2 {
            self.fat_at(xy)
        }
        else if xy.y < 16 {
            self.norm_at(Point2D::new(xy.x, xy.y - 2))
        } 
        else {
            self.small_at(Point2D::new(xy.x, xy.y - 16))
        }
    }

    pub fn norm_at(&self, xy: Point2D<u16, CellSpace>) -> CellContent {
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
    pub fn fat_at(&self, xy: Point2D<u16, CellSpace>) -> CellContent {
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
    pub fn small_at(&self, xy: Point2D<u16, CellSpace>) -> CellContent {
        let x = xy.x;
        let y = xy.y;

        CellContent { 
            bg: 0x00000000,
            fg: 0x00ffffff,
            sem: SemanticContent::Small(x + y * 16) 
        }
    }
}