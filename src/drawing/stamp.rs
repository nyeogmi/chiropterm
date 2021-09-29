use std::{cell::{Cell, RefCell}, collections::{BTreeMap, btree_map::Entry}};

use crate::{aliases::*, formatting::FSem};

use super::Brushable;

pub struct Stamp {
    content: RefCell<BTreeMap<(isize, isize), FSem>>,
    pub cursor_point: Option<CellPoint>,
    bounds: Cell<Option<CellRect>>,  // TODO: Update these
}

impl Stamp {
    pub fn new() -> Stamp {
        let cursor_point: Option<CellPoint> = None;
        Stamp { 
            content: RefCell::new(BTreeMap::new()), 
            cursor_point: cursor_point, 
            bounds: Cell::new(None),
        }
    }

    pub fn iter(&mut self) -> impl '_+DoubleEndedIterator<Item=(CellPoint, FSem)> {
        self.content.get_mut().iter().map(|(k, v)| 
            (CellPoint::new(k.0, k.1), *v)
        )
    }
}

impl Brushable for Stamp {
    fn draw(&self, at: CellPoint, f: FSem) {
        let mut content = self.content.borrow_mut();
        match content.entry((at.x, at.y)) {
            Entry::Occupied(mut o) => { 
                let new = f.superimposed_on(*o.get());
                o.insert(new);
            }
            Entry::Vacant(v) => { 
                v.insert(f);
                self.bounds.replace(match self.bounds.get() {
                    None => Some(rect(at.x, at.y, 1, 1)),
                    Some(mut b) => {
                        if at.x < b.min_x() {
                            b = rect(at.x, b.min_y(), b.max_x() - at.x, b.height())
                        }
                        if at.x > b.max_x() {
                            b = rect(b.min_x(), b.min_y(), at.x - b.min_x() + 1, b.height())
                        }
                        if at.y < b.min_y() {
                            b = rect(b.min_x(), at.y, b.width(), b.max_y() - at.y)
                        }
                        if at.y > b.max_y() {
                            b = rect(b.min_x(), b.min_y(), b.width(), at.y - b.min_y() + 1)
                        }
                        Some(b)
                    }
                });
            }
        }
    }
}