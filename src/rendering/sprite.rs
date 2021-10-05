use std::convert::TryInto;

use crate::{aliases::{PixelSize}, constants::{CELL_X, CELL_Y}};

pub(crate) struct TileSet<'a> {
    pub buf: &'a [u8],
    pub overall_size: PixelSize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tile(pub [u8; 8]);

impl<'a> TileSet<'a> {
    pub fn tile(&self, ix: usize) -> Tile {
        let n_tiles_x = (self.overall_size.width / CELL_X as u16) as usize;
        let n_tiles_y = (self.overall_size.height / CELL_Y as u16) as usize;
        let n_tiles = n_tiles_x * n_tiles_y;

        if ix >= n_tiles { return Tile([0; 8]) }

        let value: [u8; 8] = self.buf[ix * CELL_Y..(ix + 1) * CELL_Y].try_into().unwrap();
        Tile(value)
    }
}

impl Tile {
    pub(crate) fn left(&self) -> Tile {
        fn fix(row: u8) -> u8 { row << 4 }
        Tile([
            fix(self.0[0]), fix(self.0[1]), fix(self.0[2]), fix(self.0[3]),
            fix(self.0[4]), fix(self.0[5]), fix(self.0[6]), fix(self.0[7]),
        ])
    }

    pub(crate) fn right(&self) -> Tile {
        fn fix(row: u8) -> u8 { row >> 4 }
        Tile([
            fix(self.0[0]), fix(self.0[1]), fix(self.0[2]), fix(self.0[3]),
            fix(self.0[4]), fix(self.0[5]), fix(self.0[6]), fix(self.0[7]),
        ])
    }
}