use crate::aliases::{AtCell, CellSpace};
use crate::formatting::FSem;
use crate::geom::Grid;
use crate::rendering::{CellContent, SemanticContent};

use super::brush::Brushlike;

pub struct Screen {
    cells: Grid<CellContent, CellSpace>,
    // TODO: color
}

impl Screen {
    pub fn new() -> Screen {
        Screen { cells: Grid::new()}
    }
}

impl Brushlike for Screen {
    fn draw(&mut self, at: AtCell, f: FSem) {
        if !self.cells.rect().contains(at) { return; }

        let c = self.cells.get_mut(at).unwrap();
        if let Some(bg) = f.bg { c.bg = bg; }
        if let Some(fg) = f.fg { c.fg = fg; }
        if let Some(sprite) = f.sem { c.sem = sprite; }
    }
}