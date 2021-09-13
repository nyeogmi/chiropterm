use std::{cell::RefCell, collections::{BTreeMap, btree_map::Entry}};

use crate::{aliases::*, formatting::FSem};

use super::Brushable;

pub struct Stamp {
    content: RefCell<BTreeMap<(isize, isize), FSem>>,
    pub cursor_point: Option<CellPoint>,
    bounds: Option<CellRect>,  // TODO: Update these
}

impl Stamp {
    pub fn new() -> Stamp {
        let cursor_point: Option<CellPoint> = None;
        Stamp { 
            content: RefCell::new(BTreeMap::new()), 
            cursor_point: cursor_point, 
            bounds: None,
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
            }
        }
    }
}