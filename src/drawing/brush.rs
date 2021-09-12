use crate::{aliases::{AtCell, AtCellI}, formatting::{FChar, FSem}};


pub trait Brushlike {
    fn draw(&mut self, at: AtCellI, f: FSem);
}