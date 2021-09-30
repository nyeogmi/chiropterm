use crate::Brush;
use crate::aliases::*;


impl <'a> Brush<'a> {
    pub fn split_vertically(&self, y: isize) -> (Brush<'a>, Brush<'a>) {
        let y = self.rect.max_y().min(self.rect.min_y().max(y));

        let rect_1 = rect(self.rect.min_x(), self.rect.min_y(), self.rect.width(), y - self.rect.min_y());
        let rect_2 = rect(self.rect.min_x(), y, self.rect.width(), self.rect.max_y() - y);

        return (self.region(rect_1), self.region(rect_2))
    }

    pub fn split_horizontally(&self, x: isize) -> (Brush<'a>, Brush<'a>) {
        let x = self.rect.max_x().min(self.rect.min_x().max(x));

        let rect_1 = rect(self.rect.min_x(), self.rect.min_y(), x - self.rect.min_x(), self.rect.max_y());
        let rect_2 = rect(x, self.rect.min_y(), self.rect.max_x() - x, self.rect.max_y());

        return (self.region(rect_1), self.region(rect_2))
    }
}