use euclid::*;

use crate::{aliases::*, constants::{CELL_X, CELL_Y}};

#[derive(Clone, Copy)]
pub struct AspectConfig {
    pub pref_min_term_size: CellSizeU16,
    pub pref_max_term_size: CellSizeU16,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub(crate) struct Aspect {
    pub buf_size: PixelSize,
    pub term_size: CellSizeU16,
}

impl Aspect {
    pub fn term_rect(&self) -> CellRectU16 {
        Rect::new(CellPointU16::zero(), self.term_size)
    }
}


pub(crate) fn default_window_size(aspect_config: AspectConfig) -> PixelSize {
    size2(
        aspect_config.pref_min_term_size.width * CELL_X as u16,
        aspect_config.pref_min_term_size.height * CELL_Y as u16,
    )
}

pub(crate) fn calculate_aspect(
    aspect_config: AspectConfig,
    screen_size: PixelSize,
) -> Aspect {
    // step 1: if the screen size is 0 in either dimension, give up
    if screen_size.width == 0 || screen_size.height == 0 {
        return Aspect {
            buf_size: PixelSize::zero(),
            term_size: CellSizeU16::zero(),
        }
    }

    // step 2: if the screen size is too small to _possibly_ support one dimension, force downscaling to occur
    let pref_min_buf_size = size2::<u16, PixelSpace>(
        aspect_config.pref_min_term_size.width * CELL_X as u16,
        aspect_config.pref_min_term_size.height * CELL_Y as u16,
    );
    let min_downscale: f32 =
        if pref_min_buf_size.width == 0 || pref_min_buf_size.height == 0 {
            1.0  // don't bother
        } else {
            1.0_f32
                .min(screen_size.width as f32 / pref_min_buf_size.width as f32)
                .min(screen_size.height as f32 / pref_min_buf_size.height as f32)
        };

    if min_downscale < 1.0 {
        // just generate it by downscaling onto the screen
        let buf_size_f32: PixelSizeF32 = size2(
            screen_size.width as f32 / min_downscale,
            screen_size.height as f32 / min_downscale,
        );
        let term_size_f32: CellSizeF32 = size2(
            buf_size_f32.width / CELL_X as f32,
            buf_size_f32.height / CELL_Y as f32,
        );
        let mut term_size = term_size_f32.ceil().cast::<usize>();

        if term_size.width > aspect_config.pref_max_term_size.width as usize { 
            term_size.width = aspect_config.pref_max_term_size.width as usize 
        }
        if term_size.height > aspect_config.pref_max_term_size.height as usize { 
            term_size.height = aspect_config.pref_max_term_size.height as usize
        }

        let term_size = term_size.cast::<u16>();

        let buf_size: PixelSize = size2(
            term_size.width * CELL_X as u16,
            term_size.height * CELL_Y as u16, 
        );

        return Aspect {
            buf_size,
            term_size,
        }
    }

    // we don't have to downscale
    // do our best to fill the screen
    // integer pixel counts!

    // keep increasing the scaling factor until we are no longer above the minimum in all directions
    // then pick the scaling factor below that
    let mut try_scaling_factor = 0;
    let best_scaling_factor = loop {
        try_scaling_factor += 1;

        let cell_width = try_scaling_factor * CELL_X as u16;
        let cell_height = try_scaling_factor * CELL_Y as u16;

        let term_width = screen_size.width / cell_width;
        let term_height = screen_size.height / cell_height;
        if term_width < aspect_config.pref_min_term_size.width || term_height < aspect_config.pref_min_term_size.height {
            break (try_scaling_factor - 1);
        }

        if try_scaling_factor > 64 {
            panic!("shouldn't need to go bigger than 64x")
        }
    };

    // check this -- because if it were going to be 0, we would have hit the preferred downscale code
    assert_ne!(0, best_scaling_factor);

    {
        let term_width = screen_size.width / (best_scaling_factor * CELL_X as u16);
        let term_height = screen_size.height / (best_scaling_factor * CELL_Y as u16);

        let pixel_width = term_width * CELL_X as u16;
        let pixel_height = term_height * CELL_Y as u16;

        Aspect {
            buf_size: size2(pixel_width, pixel_height),
            term_size: size2(term_width, term_height),
        }
    }
}