mod math;

use euclid::size2;
use minifb::{Key, KeyRepeat, Scale, ScaleMode, Window, WindowOptions};

use crate::{drawing::Screen, rendering::{Render, Swatch}};

use self::math::{AspectConfig, calculate_aspect, default_window_size};
pub use self::math::Aspect;

// TODO: Is a cell 2 characters across
const ASPECT_CONFIG: AspectConfig = AspectConfig {
    pref_min_term_size: size2(64, 48),  // but expect ~112x60
    pref_max_term_size: size2(256, 256),
    cell_size: size2(8, 8),
};

const FRAMES_PER_SECOND: usize = 30;
const FRAME_DURATION: usize = 2 * 16600; // 30 FPS

pub struct IO {
    frame: u64,
    window: Option<Window>,
    buffer: Vec<u32>,
    swatch: Swatch,
    pub screen: Screen,  // TODO: Make private again later

    default_on_exit: fn(&mut IO),
}

struct EventLoop<'a> {
    on_redraw: Box<dyn 'a+FnMut(&mut IO)>,
    on_exit: Box<dyn 'a+FnMut(&mut IO)>,

    on_keypress: Box<dyn 'a+FnMut(&mut IO, Key) -> Resume>,
    on_frame: Box<dyn 'a+FnMut(&mut IO) -> Resume>,
}

enum Resume {
    NotYet,
    Now,
}

impl IO {
    pub fn new(swatch: Swatch, default_on_exit: fn(&mut IO)) -> IO {
        IO { 
            frame: 0, window: None, buffer: vec![], swatch, screen: Screen::new(swatch.default_bg, swatch.default_fg),
            default_on_exit,
        }
    }

    fn reconstitute_window(&mut self) {
        let mut opts = WindowOptions::default();
        opts.scale = Scale::FitScreen;
        opts.scale_mode = ScaleMode::Stretch;
        opts.resize = true;

        let wsz = default_window_size(ASPECT_CONFIG);  

        let mut window = Window::new(
            "TODO: Entitle this window",
            wsz.width as usize, wsz.height as usize,
            opts,
        ).unwrap_or_else(|e| {
            panic!("{}", e); // TODO: Handle some errors
        });
        window.set_background_color(0, 0, 0); // TODO:
        // max 30FPS
        window.limit_update_rate(Some(std::time::Duration::from_micros(FRAME_DURATION as u64)));
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

    pub fn getch(&mut self, on_redraw: impl FnMut(&mut IO)) -> Key {
        let mut key = Key::Unknown;
        self.wait(EventLoop {
            on_redraw: Box::new(on_redraw),
            on_exit: Box::new(self.default_on_exit),

            on_keypress: Box::new(|_, k| { key = k; Resume::Now }),
            on_frame: Box::new(|_| Resume::NotYet),
        });
        key
    }

    pub fn sleep(&mut self, time: f64, on_redraw: impl FnMut(&mut IO)) {
        let mut frame = 0;
        self.wait(EventLoop {
            on_redraw: Box::new(on_redraw),
            on_exit: Box::new(self.default_on_exit),

            on_keypress: Box::new(|_, __| Resume::NotYet),
            on_frame: Box::new(|_| {
                if frame as f64 / FRAMES_PER_SECOND as f64 > time {
                    return Resume::Now;
                }
                frame += 1;
                Resume::NotYet
            }),
        });
    }

    fn wait<'a>(&mut self, mut evt: EventLoop<'a>) {
        // NYEO NOTE: What are we waiting for? 
        // Multiple fns that call this would be ideal

        loop {
            if let None = self.window { self.reconstitute_window() }
            let aspect = self.reconstitute_buffer();  
            // TODO: On the _first_ pass, clear the screen

            // set by reconstitute()
            let win = self.window.as_mut().unwrap();

            if !win.is_open() {
                (evt.on_exit)(self);
                self.window = None;
                continue;  // try again
            }

            if let Resume::Now = (evt.on_frame)(self) { return }

            let win = self.window.as_mut().unwrap();
            if let Some(pressed) = win.get_keys_pressed(KeyRepeat::Yes) {
                for key in pressed {
                    if let Resume::Now = (evt.on_keypress)(self, key) { return }
                }
            }

            self.screen.resize(aspect.term_size.cast());
            (evt.on_redraw)(self);
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
        Render { 
            aspect, 
            frame: self.frame, 
            buffer: &mut self.buffer, 
            swatch: self.swatch,
            screen: &self.screen,
        }.draw()
    }
}