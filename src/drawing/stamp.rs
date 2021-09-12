use std::collections::{BTreeMap, btree_map::Entry};

use euclid::{Point2D, Rect, point2};

use crate::{aliases::{AtCell, AtCellI, CellSpace}, cp437, formatting::{FChar, FSem}};

use super::brush::Brushlike;

use crate::geom::PointsIn;

pub struct Stamp {
    content: BTreeMap<(isize, isize), FSem>,
    pub cursor_point: Option<AtCellI>,
    bounds: Option<Rect<isize, CellSpace>>,
}

impl Stamp {
    pub fn new() -> Stamp {
        let cursor_point: Option<AtCellI> = None;
        Stamp { 
            content: BTreeMap::new(), 
            cursor_point: cursor_point, 
            bounds: None,
        }
    }

    pub fn iter(&self) -> impl '_+DoubleEndedIterator<Item=(AtCellI, FSem)> {
        self.content.iter().map(|(k, v)| 
            (AtCellI::new(k.0, k.1), *v)
        )
    }

    /*
    fn to_plain_text(&self) -> String {
        let b = match self.bounds {
            None => { return "".to_owned(); }
            Some(b) => b
        };

        let result = String::new();
        for dy in 0..b.size.height {
            let y = b.origin.y + dy;

            for dx in 0..b.size.width {
                let x = b.origin.x + dx;

                result.push(match self.content.get(&(x, y)) {
                    Some(spr) => match spr.sprite {
                        x if x <= u8::MAX as u16 => cp437::decode_char(x as u8),
                        _ => '?',
                    }
                    None => ' '
                })
            }

            if y != b.size.height - 1 {
                result.push('\n');
            }
        };

        result
    }
    */
}

impl Brushlike for Stamp {
    fn draw(&mut self, at: AtCellI, f: FSem) {
        match self.content.entry((at.x, at.y)) {
            Entry::Occupied(o) => { 
                let new = f.superimposed_on(*o.get());
                o.insert(new);
            }
            Entry::Vacant(v) => { 
                v.insert(f);
            }
        }
    }
}