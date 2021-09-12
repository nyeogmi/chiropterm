use std::convert::TryInto;

use crate::aliases::{PixelSize};

pub struct TileSet<'a> {
    pub buf: &'a [u8],
    pub overall_size: PixelSize,
}

const TILE_X: u16 = 8;
const TILE_Y: u16 = 8;

#[derive(Clone, Copy)]
pub struct Tile(pub [u8; 8]);

impl<'a> TileSet<'a> {
    pub fn tile(&self, ix: usize) -> Tile {
        let n_tiles_x = (self.overall_size.width / TILE_X) as usize;
        let n_tiles_y = (self.overall_size.height / TILE_Y) as usize;
        let n_tiles = n_tiles_x * n_tiles_y;

        if ix >= n_tiles { return Tile([0; 8]) }

        let value: [u8; 8] = self.buf[ix * TILE_Y as usize..(ix + 1) * TILE_Y as usize].try_into().unwrap();
        Tile(value)
    }
}

impl Tile {
    #[inline(always)]  // TODO: Is this an actual savings?
    pub(crate) fn render(&self, out_buf: &mut Vec<u32>, out_x: u16, out_y: u16, out_width: u16, fg: u32, bg: u32) {
        let real_out_x = out_x as usize * TILE_X as usize;
        let real_out_y = out_y as usize * TILE_Y as usize;
        let real_out_width = out_width as usize * TILE_X as usize;

        for y in [0, 1, 2, 3, 4, 5, 6, 7] {
            for x in [0, 1, 2, 3, 4, 5, 6, 7] {
                out_buf[((real_out_y + y) * real_out_width + real_out_x + x) as usize] = 
                    if self.0[y as usize] & (1 << x) != 0 { fg } else { bg };
            }
        }
    }

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