use std::cell::Cell;

use crate::aliases::*;
use crate::formatting::FSem;
use crate::rendering::{CellContent, SemanticContent};

use gridd_euclid::{Grid, PointsIn};

use super::brush::Brushable;

pub struct Screen {
    pub(crate) cells: Grid<Cell<CellContent>, CellSpace>,  // pub(crate) so the renderer can access this directly
    pub(crate) bg: u8,
    pub(crate) fg: u8
}

impl Screen {
    pub fn new(bg: u8, fg: u8) -> Screen {
        Screen { bg, fg, cells: Grid::new(
            rect(0, 0, 0, 0), 
            || Cell::new(CellContent {
                bg, fg, sem: SemanticContent::Blank,
            })
        )}
    }

    pub fn clear(&mut self) {
        for at in isize::points_in(self.cells.rect()) {
            let cell = self.cells.get(at).unwrap();
            cell.update(|mut c| {
                c.bg = self.bg;
                c.fg = self.fg;
                c.sem = SemanticContent::Blank;
                c
            });
        }
    }

    pub fn resize(&mut self, sz: CellSize) {
        let bg = self.bg;
        let fg = self.fg;
        self.cells.resize(
            rect(0, 0, sz.width, sz.height), 
            || Cell::new(CellContent {
                bg, fg, sem: SemanticContent::Blank,
            })
        )
    }

    pub fn rect(&self) -> CellRect {
        self.cells.rect()
    }
}

impl Brushable for Screen {
    fn draw(&self, at: CellPoint, f: FSem) {
        if !self.cells.rect().contains(at) { return; }

        let cell = self.cells.get(at).unwrap();
        cell.update(|mut c| {
            if let Some(bg) = f.bg { c.bg = bg; }
            if let Some(fg) = f.fg { c.fg = fg; }
            if let Some(sprite) = f.sem { c.sem = sprite; }
            c
        });
    }
}