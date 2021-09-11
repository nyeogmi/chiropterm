mod math;

use euclid::{size2};
use minifb::{Scale, ScaleMode, Window, WindowOptions};

use self::math::{Aspect, AspectConfig, calculate_aspect, default_window_size};

use crate::geom::PointsIn;

// TODO: Is a cell 2 characters across
const ASPECT_CONFIG: AspectConfig = AspectConfig {
    pref_min_term_size: size2(80, 60),  // but expect ~112x60
    pref_max_term_size: size2(256, 256),
    cell_size: size2(8, 8),
};

pub struct IO {
    commands: IOCommands,
    callbacks: IOCallbacks,
}
pub struct IOCommands {
    frame: u64,
    window: Option<Window>,
    buffer: Vec<u32>,
}

pub struct IOCallbacks {
    on_exit: Box<dyn FnMut(&mut IOCommands)>,
}


impl IO {
    pub fn new(on_exit: impl 'static+FnMut(&mut IOCommands)) -> IO {
        IO { 
            commands: IOCommands { frame: 0, window: None, buffer: vec![] },
            callbacks: IOCallbacks { on_exit: Box::new(on_exit) },
        }
    }

    pub fn wait(&mut self) {
        // TODO: Termination condition
        self.commands.wait(&mut self.callbacks)
    }
}

impl IOCommands {
    fn reconstitute_window(&mut self) {
        let mut opts = WindowOptions::default();
        opts.scale = Scale::FitScreen;
        opts.scale_mode = ScaleMode::AspectRatioStretch;
        opts.resize = true;

        let wsz = default_window_size(ASPECT_CONFIG, size2(640, 480));  // TODO: Get actual res

        let mut window = Window::new(
            "TODO: Entitle this window",
            wsz.width as usize, wsz.height as usize,
            opts,
        ).unwrap_or_else(|e| {
            panic!("{}", e); // TODO: Handle some errors
        });
        window.set_background_color(0, 0, 0); // TODO:
        // max 30FPS
        window.limit_update_rate(Some(std::time::Duration::from_micros(2 * 16600)));
        self.window = Some(window)
    }

    fn reconstitute_buffer(&mut self) -> Aspect {
        // NOTE: Must be called after reconstitute_window()
        let win = self.window.as_mut().unwrap();
        let (actual_w, actual_h) = win.get_size();

        let aspect = calculate_aspect(ASPECT_CONFIG, size2(actual_w as u16, actual_h as u16));

        let buf_len = aspect.buf_size.width as usize * aspect.buf_size.height as usize;
        if self.buffer.len() != buf_len {
            // TODO: Clear old data?
            self.buffer.resize(buf_len, 0); 
        }
        aspect
    }

    fn wait(&mut self, callbacks: &mut IOCallbacks) {
        // NYEO NOTE: What are we waiting for? 
        // Multiple fns that call this would be ideal

        loop {
            if let None = self.window { self.reconstitute_window() }
            let aspect = self.reconstitute_buffer();

            // set by reconstitute()
            let win = self.window.as_mut().unwrap();

            if !win.is_open() {
                (callbacks.on_exit)(self);
                self.window = None;
                continue;  // try again
            }

            self.draw(aspect);

            let win = self.window.as_mut().unwrap();
            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            win
                .update_with_buffer(&self.buffer, aspect.buf_size.width as usize, aspect.buf_size.height as usize)
                .unwrap();
        }
    }

    fn draw(&mut self, aspect: Aspect) {
        self.frame += 1;

        for term_xy in u16::points_in(aspect.term_rect()) {
            let cell = aspect.buf_cell_area(term_xy);
            for pixel_xy in u16::points_in(cell) {
                let r = (term_xy.x * 11) as u64 + self.frame;
                let g = (term_xy.y * 7) as u64 + self.frame;
                let b = 0;

                let buffer_ix = pixel_xy.y as usize * aspect.buf_size.width as usize + pixel_xy.x as usize;
                self.buffer[buffer_ix] = 
                    ((r & 0xff) as u32) << 16 |
                    ((g & 0xff) as u32) << 8 |
                    (b & 0xff) as u32 
                ;

            }
        }

        /*
        let mut x = ((self.frame * 4) & 0xffffffff) as u32;
        // draw!
        for (ix, i) in self.buffer.iter_mut().enumerate() {
            if ix % 32768 == 0 {
                x = ((x as u64 + (0x100000000 - 0x0007050b)) & 0xffffffff) as u32;
            }
            *i = x; // write something more funny here!
        }
        */
    }
}