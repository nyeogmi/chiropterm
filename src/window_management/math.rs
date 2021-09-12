use euclid::*;

use crate::aliases::*;

pub struct AspectConfig {
    // TODO: Allow the user to have a preferred scaling factor and choose that if possible

    pub pref_min_term_size: CellSizeU16,
    pub pref_max_term_size: CellSizeU16,
    pub cell_size: PixelSize,
}

#[derive(Clone, Copy, Debug)]
pub struct Aspect {
    pub buf_size: PixelSize,
    pub term_size: CellSizeU16,
    pub cell_size: PixelSize,
}

impl Aspect {
    pub fn term_rect(&self) -> CellRectU16 {
        Rect::new(CellPointU16::zero(), self.term_size)
    }
}


pub fn default_window_size(aspect_config: AspectConfig) -> PixelSize {
    size2(
        aspect_config.pref_min_term_size.width * aspect_config.cell_size.width,
        aspect_config.pref_min_term_size.height * aspect_config.cell_size.height,
    )
}

pub fn calculate_aspect(
    aspect_config: AspectConfig,
    screen_size: PixelSize,
) -> Aspect {
    // step 1: if the screen size is 0 in either dimension, give up
    if screen_size.width == 0 || screen_size.height == 0 {
        return Aspect {
            buf_size: PixelSize::zero(),
            term_size: CellSizeU16::zero(),
            cell_size: PixelSize::zero(),
        }
    }

    // step 2: if the screen size is too small to _possibly_ support one dimension, force downscaling to occur
    let pref_min_buf_size = size2::<u16, PixelSpace>(
        aspect_config.pref_min_term_size.width * aspect_config.cell_size.width,
        aspect_config.pref_min_term_size.height * aspect_config.cell_size.height,
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
            buf_size_f32.width / aspect_config.cell_size.width as f32,
            buf_size_f32.height / aspect_config.cell_size.height as f32,
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
            term_size.width * aspect_config.cell_size.width,
            term_size.height * aspect_config.cell_size.height,
        );

        return Aspect {
            buf_size,
            term_size,
            cell_size: aspect_config.cell_size,
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

        let cell_width = try_scaling_factor * aspect_config.cell_size.width;
        let cell_height = try_scaling_factor * aspect_config.cell_size.height;

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
        // we used to consider the scaling factor here: actually don't!
        let cell_width = aspect_config.cell_size.width;
        let cell_height = aspect_config.cell_size.height;

        let term_width = screen_size.width / (best_scaling_factor * cell_width);
        let term_height = screen_size.height / (best_scaling_factor * cell_height);

        let pixel_width = term_width * cell_width;
        let pixel_height = term_height * cell_height;

        Aspect {
            buf_size: size2(pixel_width, pixel_height),
            term_size: size2(term_width, term_height),
            cell_size: size2(cell_width, cell_height),
        }
    }
}