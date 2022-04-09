use std::cell::RefCell;

use gridd_euclid::{CopyEndlessGrid, PointsIn};

use crate::{Brush, aliases::*, formatting::FSem};

use super::Brushable;

pub struct Stamp {
    content: RefCell<CopyEndlessGrid<Option<FSem>, CellSpace>>,
    pub cursor_point: Option<CellPoint>,
}

impl Stamp {
    pub fn new() -> Stamp {
        let cursor_point: Option<CellPoint> = None;
        Stamp { 
            content: RefCell::new(CopyEndlessGrid::new(None)), 
            cursor_point, 
        }
    }

    pub fn draw(&self, b: Brush) {  // TODO: Offset?
        let content = self.content.borrow();
        let intersecting_region = rect(0, 0, b.size().width, b.size().height).intersection(&content.rect());

        match intersecting_region {
            None => { /* don't bother drawing */ }
            Some(region) => {
                // TODO: Compare area to area of me as a whole
                for xy in isize::points_in(region) {
                    if let Some(sem) = content.get(xy) { 
                        b.draw(xy, sem); 
                    }
                }
                /* 
                for (xy, c) in self.content.borrow().iter() {
                    b.draw(xy, c)
                }
                */
            }
        }

    }

    pub fn rect(&self) -> CellRect {
        self.content.borrow().rect()
    }
}

impl Brushable for Stamp {
    fn draw(&self, at: CellPoint, f: FSem) {
        let mut content = self.content.borrow_mut();
        let present = content.get(at);
        let new = if let Some(p) = present {
            Some(f.superimposed_on(p))
        } else {
            Some(f)
        };
        content.set(at, new);
    }
}