use crate::{Brush, Brushable, FSem};
use crate::aliases::*;

impl <'a, B: Brushable> Brush<'a, B> {
    pub fn bevel_w95(&self, top_left: u8, bottom_right: u8) {
        // shorthand for w95-style bevels
        self.bevel_top(top_left);
        self.bevel_left(top_left);
        self.bevel_right(bottom_right);
        self.bevel_bottom(bottom_right);
    }

    pub fn bevel_w95_sleek(&self, left: u8, right: u8) {
        // w95-style bevels with no top or bottom
        self.bevel_left(left);
        self.bevel_right(right);
    }

    pub fn bevel_top(&self, color: u8) {
        if self.rect.height() == 0 { return; }

        let mut sem = FSem::new();
        sem.bevels.top = Some(color);

        for x in self.rect.min_x()..self.rect.max_x() {
            self.draw(point2(x, self.rect.min_y()), sem);
        }
    }

    pub fn bevel_left(&self, color: u8) {
        if self.rect.width() == 0 { return; }

        let mut sem = FSem::new();
        sem.bevels.left = Some(color);

        for y in self.rect.min_y()..self.rect.max_y() {
            self.draw(point2(self.rect.min_x(), y), sem);
        }
    }

    pub fn bevel_right(&self, color: u8) {
        if self.rect.width() == 0 { return; }

        let mut sem = FSem::new();
        sem.bevels.right = Some(color);

        for y in self.rect.min_y()..self.rect.max_y() {
            self.draw(point2(self.rect.max_x() - 1, y), sem);
        }
    }

    pub fn bevel_bottom(&self, color: u8) {
        if self.rect.height() == 0 { return; }

        let mut sem = FSem::new();
        sem.bevels.bottom = Some(color);

        for x in self.rect.min_x()..self.rect.max_x() {
            self.draw(point2(x, self.rect.max_y() - 1), sem);
        }
    }
}