use crate::{Brush, Brushable, FSem};
use gridd_euclid::PointsIn;

use euclid::rect;

impl <'a> Brush<'a> {
    pub fn fill(&self, f: FSem) {
        for i in isize::points_in(rect(0, 0, self.size().width, self.size().height)) {
            self.draw(i, f)
        }
    }
}