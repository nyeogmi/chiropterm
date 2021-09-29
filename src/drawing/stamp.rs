use std::cell::RefCell;

use gridd_euclid::CopyEndlessGrid;

use crate::{aliases::*, formatting::FSem};

use super::Brushable;

pub struct Stamp {
    content: RefCell<CopyEndlessGrid<FSem, CellSpace>>,
    pub cursor_point: Option<CellPoint>,
}

impl Stamp {
    pub fn new() -> Stamp {
        let cursor_point: Option<CellPoint> = None;
        Stamp { 
            content: RefCell::new(CopyEndlessGrid::new(FSem::new())), 
            cursor_point: cursor_point, 
        }
    }

    pub fn iter(&mut self) -> impl '_+DoubleEndedIterator<Item=(CellPoint, FSem)> {
        self.content.get_mut().iter_populated()
    }
}

impl Brushable for Stamp {
    fn draw(&self, at: CellPoint, f: FSem) {
        let mut content = self.content.borrow_mut();
        let present = content.get(at);
        let new = f.superimposed_on(present);
        content.set(at, new);
    }
}