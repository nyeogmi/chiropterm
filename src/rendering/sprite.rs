use std::convert::TryInto;

use crate::{aliases::{PixelSize}, constants::{CELL_X, CELL_Y}};

pub(crate) struct TileSet<'a> {
    pub buf: &'a [u8],
    pub overall_size: PixelSize,
}

#[derive(Clone, Copy)]
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
    #[inline(always)]  // TODO: Is this an actual savings?
    pub(crate) fn render(&self, 
        out_buf: &mut Vec<u32>, 
        out_x: u16, out_y: u16, out_width: u16, 
        fg: u32, bg: u32,
        top_bevel: bool, top_bevel_fg: u32,
        left_bevel: bool, left_bevel_fg: u32,
        right_bevel: bool, right_bevel_fg: u32,
        bottom_bevel: bool, bottom_bevel_fg: u32,
    ) {
        let real_out_x = out_x as usize * CELL_X;
        let real_out_y = out_y as usize * CELL_Y;
        let real_out_width = out_width as usize * CELL_X;
        
        // bg of text 
        for y in [0, 1, 2, 3, 4, 5, 6, 7] {
            for x in [0, 1, 2, 3, 4, 5, 6, 7] {
                if self.0[y as usize] & (1 << x) == 0 { 
                    out_buf[((real_out_y + y) * real_out_width + real_out_x + x) as usize] = bg;
                }
            }
        }

        // top and bottom supercede left and right
        if left_bevel {
            for y in [0, 1, 2, 3, 4, 5, 6, 7] {
                out_buf[((real_out_y + y) * real_out_width + real_out_x + 0) as usize] = left_bevel_fg;
            }
        }

        if right_bevel {
            for y in [0, 1, 2, 3, 4, 5, 6, 7] {
                out_buf[((real_out_y + y) * real_out_width + real_out_x + (CELL_X - 1)) as usize] = right_bevel_fg;
            }
        }

        if top_bevel {
            for x in [0, 1, 2, 3, 4, 5, 6, 7] {
                out_buf[((real_out_y + 0) * real_out_width + real_out_x + x) as usize] = top_bevel_fg;
            }
        }

        if bottom_bevel {
            for x in [0, 1, 2, 3, 4, 5, 6, 7] {
                out_buf[((real_out_y + CELL_Y - 1) * real_out_width + real_out_x + x) as usize] = bottom_bevel_fg;
            }
        }

        // fg of text 
        for y in [0, 1, 2, 3, 4, 5, 6, 7] {
            for x in [0, 1, 2, 3, 4, 5, 6, 7] {
                if self.0[y as usize] & (1 << x) != 0 { 
                    out_buf[((real_out_y + y) * real_out_width + real_out_x + x) as usize] = fg;
                }
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