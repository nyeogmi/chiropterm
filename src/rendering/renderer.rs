use crate::{aliases::*, constants::{CELL_X, CELL_Y}, drawing::Screen};

use crate::window_management::Aspect;

use super::{Interactor, Swatch, sprite::Tile};

#[derive(Eq, PartialEq)]
pub(crate) struct Render {
    pub aspect: Aspect,
    pub swatch: Swatch,
    pub interactor: Interactor,
}


impl Render {
    pub fn get_content(&self, screen: &Screen, term_xy: CellPoint) -> RenderContent {
        let content = screen.cells.get(term_xy).unwrap().get();
        let interacting_here = if self.interactor == Interactor::none() { 
            false 
        } else { 
            content.interactor.interactor == self.interactor 
        };
        let tile = crate::rendering::font::eval(content.sem);
        let fg: u8;
        let bg: u8;
        if interacting_here {
            // flash!
            fg = if content.interactor.fg == 255 { content.bg } else { content.interactor.fg };
            bg = if content.interactor.bg == 255 { content.fg } else { content.interactor.bg };
        } else {
            fg = content.fg;
            bg = content.bg;
        };

        RenderContent {
            tile, bg: self.swatch.get(bg), fg: self.swatch.get(fg),

            bevel_top: !(interacting_here || content.bevels.top == 255), 
            bevel_top_fg: self.swatch.get(content.bevels.top),

            bevel_left: !(interacting_here || content.bevels.left == 255), 
            bevel_left_fg: self.swatch.get(content.bevels.left),

            bevel_right: !(interacting_here || content.bevels.right == 255), 
            bevel_right_fg: self.swatch.get(content.bevels.right),

            bevel_bottom: !(interacting_here || content.bevels.bottom == 255), 
            bevel_bottom_fg: self.swatch.get(content.bevels.bottom),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderContent {
    tile: Tile,
    fg: u32, bg: u32,
    bevel_top: bool, bevel_top_fg: u32,
    bevel_left: bool, bevel_left_fg: u32,
    bevel_right: bool, bevel_right_fg: u32,
    bevel_bottom: bool, bevel_bottom_fg: u32,
}

impl RenderContent {
    pub(crate) fn physically_draw(
        &self, 
        out_buf: &mut Vec<u32>,  // TODO: Do this unchecked
        out_x: u16, out_y: u16, out_width: u16, 
    ) {
        if !(self.bevel_top || self.bevel_left || self.bevel_right || self.bevel_bottom) {
            return self.render_fast(out_buf, out_x, out_y, out_width);
        }

        let real_out_x = out_x as usize * CELL_X;
        let real_out_y = out_y as usize * CELL_Y;
        let real_out_width = out_width as usize * CELL_X;
        
        // bg of text 
        for y in [0, 1, 2, 3, 4, 5, 6, 7] {
            for x in [0, 1, 2, 3, 4, 5, 6, 7] {
                if self.tile.0[y as usize] & (1 << x) == 0 { 
                    out_buf[((real_out_y + y) * real_out_width + real_out_x + x) as usize] = self.bg;
                }
            }
        }

        // top and bottom supercede left and right
        if self.bevel_left {
            for y in [0, 1, 2, 3, 4, 5, 6, 7] {
                unsafe {
                    *out_buf.get_unchecked_mut(((real_out_y + y) * real_out_width + real_out_x + 0) as usize) = self.bevel_left_fg;
                }
            }
        }

        if self.bevel_right {
            for y in [0, 1, 2, 3, 4, 5, 6, 7] {
                unsafe {
                    *out_buf.get_unchecked_mut((real_out_y + y) * real_out_width + real_out_x + (CELL_X - 1) as usize) = self.bevel_right_fg;
                }
            }
        }

        if self.bevel_top {
            for x in [0, 1, 2, 3, 4, 5, 6, 7] {
                unsafe {
                    *out_buf.get_unchecked_mut(((real_out_y + 0) * real_out_width + real_out_x + x) as usize) = self.bevel_top_fg;
                }
            }
        }

        if self.bevel_bottom {
            for x in [0, 1, 2, 3, 4, 5, 6, 7] {
                unsafe {
                    *out_buf.get_unchecked_mut(((real_out_y + CELL_Y - 1) * real_out_width + real_out_x + x) as usize) = self.bevel_bottom_fg;
                }
            }
        }

        // fg of text 
        for y in [0, 1, 2, 3, 4, 5, 6, 7] {
            for x in [0, 1, 2, 3, 4, 5, 6, 7] {
                if self.tile.0[y as usize] & (1 << x) != 0 { 
                    unsafe {
                        *out_buf.get_unchecked_mut(((real_out_y + y) * real_out_width + real_out_x + x) as usize) = self.fg;
                    }
                }
            }
        }
    }

    pub(crate) fn render_fast(
        &self, 
        out_buf: &mut Vec<u32>, 
        out_x: u16, out_y: u16, out_width: u16, 
    ) {
        let real_out_x = out_x as usize * CELL_X;
        let real_out_y = out_y as usize * CELL_Y;
        let real_out_width = out_width as usize * CELL_X;

        for y in [0, 1, 2, 3, 4, 5, 6, 7] {
            for x in [0, 1, 2, 3, 4, 5, 6, 7] {
                let color = if self.tile.0[y as usize] >> x & 1 == 1 { self.fg } else { self.bg };
                unsafe {
                    *out_buf.get_unchecked_mut(((real_out_y + y) * real_out_width + real_out_x + x) as usize) = color;
                }
            }
        }
    }

}