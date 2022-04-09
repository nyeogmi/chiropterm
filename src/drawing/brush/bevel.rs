use crate::{Brush, Brushable, FSem};
use crate::aliases::*;

impl <'a> Brush<'a> {
    pub fn bevel_w95(&self, (top_left, bottom_right): (u8, u8)) {
        // shorthand for w95-style bevels
        self.bevel_top(top_left);
        self.bevel_left(top_left);
        self.bevel_right(bottom_right);
        self.bevel_bottom(bottom_right);
    }

    pub fn bevel_w95_sleek(&self, (left, right): (u8, u8)) {
        // w95-style bevels with no top or bottom
        self.bevel_left(left);
        self.bevel_right(right);
    }

    pub fn bevel_top(&self, color: u8) {
        if self.clip.height() == 0 { return; }

        let mut sem = FSem::new();
        sem.bevels.top = Some(color);

        for x in 0..self.clip.size.width {
            self.draw(point2(x, 0), sem);
        }
    }

    pub fn bevel_left(&self, color: u8) {
        if self.clip.width() == 0 { return; }

        let mut sem = FSem::new();
        sem.bevels.left = Some(color);

        for y in 0..self.clip.size.height {
            self.draw(point2(0, y), sem);
        }
    }

    pub fn bevel_right(&self, color: u8) {
        if self.clip.width() == 0 { return; }

        let mut sem = FSem::new();
        sem.bevels.right = Some(color);

        for y in 0..self.clip.size.height {
            self.draw(point2(self.clip.size.width - 1, y), sem);
        }
    }

    pub fn bevel_bottom(&self, color: u8) {
        if self.clip.height() == 0 { return; }

        let mut sem = FSem::new();
        sem.bevels.bottom = Some(color);

        for x in 0..self.clip.size.width {
            self.draw(point2(x, self.clip.size.height - 1), sem);
        }
    }
}