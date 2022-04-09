use crate::Brush;
use crate::aliases::*;


impl <'a> Brush<'a> {
    pub fn split_vertically(&self, y: isize) -> (Brush<'a>, Brush<'a>) {
        let y = self.clip.width().min(y).max(0);

        let rect_1 = rect(0, 0, self.clip.width(), y);
        let rect_2 = rect(0, y, self.clip.width(), self.clip.height() - y);

        return (self.region(rect_1), self.region(rect_2))
    }

    pub fn split_horizontally(&self, x: isize) -> (Brush<'a>, Brush<'a>) {
        let x = self.clip.width().min(x).max(0);

        let rect_1 = rect(0, 0, x, self.clip.height());
        let rect_2 = rect(x, 0, self.clip.width() - x, self.clip.height());

        return (self.region(rect_1), self.region(rect_2))
    }
}