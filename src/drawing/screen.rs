use euclid::Size2D;

use crate::aliases::*;
use crate::formatting::FSem;
use crate::geom::Grid;
use crate::rendering::{CellContent, SemanticContent};

use super::brush::Brushlike;

pub struct Screen {
    pub(crate) cells: Grid<CellContent, CellSpace>,  // pub(crate) so the renderer can access this directly
    // TODO: color
}

impl Screen {
    pub fn new() -> Screen {
        Screen { cells: Grid::new(
            rect(0, 0, 0, 0), 
            CellContent {
                fg: 1,
                bg: 0,
                sem: SemanticContent::Blank,
            }
        )}
    }

    pub fn resize(&mut self, sz: CellSize) {
        self.cells.resize(rect(0, 0, sz.width, sz.height), CellContent {
            fg: 1,
            bg: 0,
            sem: SemanticContent::Blank,
        })
    }

    pub fn rect(&self) -> CellRect {
        self.cells.rect()
    }
}

impl Brushlike for Screen {
    fn draw(&mut self, at: CellPoint, f: FSem) {
        if !self.cells.rect().contains(at) { return; }

        let c = self.cells.get_mut(at).unwrap();
        if let Some(bg) = f.bg { c.bg = bg; }
        if let Some(fg) = f.fg { c.fg = fg; }
        if let Some(sprite) = f.sem { c.sem = sprite; }
    }
}