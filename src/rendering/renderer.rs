use euclid::point2;

use crate::{aliases::CellPointU16, drawing::Screen};
use crate::geom::PointsIn;

use crate::window_management::Aspect;

use super::{Swatch, cell::{CellContent, SemanticContent}};

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
            let content = self.screen.cells.get(term_xy.cast()).unwrap();
            let tile = super::font::eval(content.sem);

            tile.render(
                self.buffer, term_xy.x, term_xy.y, self.aspect.term_size.width, 
                self.swatch.get(content.fg), self.swatch.get(content.bg)
            );
        }
    }

    pub fn cell_at(&self, xy: CellPointU16) -> CellContent {
        if xy.y < 2 {
            self.fat_at(xy)
        }
        else if xy.y < 16 {
            self.norm_at(point2(xy.x, xy.y - 2))
        } 
        else {
            self.small_at(point2(xy.x, xy.y - 16))
        }
    }

    pub fn norm_at(&self, xy: CellPointU16) -> CellContent {
        let x = xy.x;
        let y = xy.y / 2;
        let is_top = xy.y % 2 == 0;

        CellContent {
            bg: 0,
            fg: 1,
            sem: if is_top {
                SemanticContent::TopHalf(x + y * 16)
            } else {
                SemanticContent::BottomHalf(x + y * 16)
            }
        }
    }
    pub fn fat_at(&self, xy: CellPointU16) -> CellContent {
        let x = xy.x / 2;
        let y = xy.y / 2;
        let is_left = xy.x % 2 == 0;
        let is_top = xy.y % 2 == 0;

        CellContent {
            bg: 0,
            fg: 1,
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
    pub fn small_at(&self, xy: CellPointU16) -> CellContent {
        let x = xy.x;
        let y = xy.y;

        CellContent { 
            bg: 0,
            fg: 1,
            sem: SemanticContent::Small(x + y * 16) 
        }
    }
}