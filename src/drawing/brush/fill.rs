use crate::{Brush, Brushable, FSem};
use gridd_euclid::PointsIn;


impl <'a> Brush<'a> {
    pub fn fill(&self, f: FSem) {
        for i in isize::points_in(self.rect) {
            self.draw(i, f)
        }
    }
}