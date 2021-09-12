use crate::{aliases::CellPoint, formatting::FSem};


pub trait Brushlike {
    fn draw(&mut self, at: CellPoint, f: FSem);
}