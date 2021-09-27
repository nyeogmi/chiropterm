mod keyboard;
mod math;
mod menu;

use euclid::size2;
use minifb::{Scale, ScaleMode, Window, WindowOptions};

use crate::{drawing::Screen, rendering::{Render, Swatch}, window_management::keyboard::Keyboard};

use self::{math::{AspectConfig, calculate_aspect, default_window_size}, menu::{Handled, Menu}};
pub use self::math::Aspect;

pub use keyboard::{ChiroptermKey, Keycode};

// TODO: Is a cell 2 characters across
const ASPECT_CONFIG: AspectConfig = AspectConfig {
    pref_min_term_size: size2(64, 48),  // but expect ~112x60
    pref_max_term_size: size2(256, 256),
    cell_size: size2(8, 8),
};

const FRAMES_PER_SECOND: usize = 30;
const TRUE_FRAME_DURATION: usize = 1660; // 600 FPS
const REDRAW_EVERY: u64 = 20;   // practically 30 FPS

pub struct IO {
    frame: u64,
    window: Option<Window>,
    keyboard: Keyboard,

    buffer: Vec<u32>,
    swatch: Swatch,
    pub screen: Screen,  // TODO: Make private again later

    default_on_exit: fn(&mut IO),
}

struct EventLoop<'a> {
    on_redraw: Box<dyn 'a+FnMut(&mut IO)>,
    on_exit: Box<dyn 'a+FnMut(&mut IO)>,

    on_keypress: Box<dyn 'a+FnMut(&mut IO, ChiroptermKey) -> Resume>,
    on_frame: Box<dyn 'a+FnMut(&mut IO) -> Resume>,
}

enum Resume {
    NotYet,
    Now,
}

impl IO {
    pub fn new(swatch: Swatch, default_on_exit: fn(&mut IO)) -> IO {
        IO { 
            frame: 0, window: None, keyboard: Keyboard::new(),
            
            buffer: vec![], swatch, screen: Screen::new(swatch.default_bg, swatch.default_fg),
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
        window.limit_update_rate(Some(std::time::Duration::from_micros(TRUE_FRAME_DURATION as u64)));
        self.keyboard.monitor_minifb_utf32(&mut window);
        self.window = Some(window);
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

    pub fn getch(&mut self, on_redraw: impl FnMut(&mut IO)) -> ChiroptermKey {
        let mut key = None;
        self.wait(EventLoop {
            on_redraw: Box::new(on_redraw),
            on_exit: Box::new(self.default_on_exit),

            on_keypress: Box::new(|_, k| { key = Some(k); Resume::Now }),
            on_frame: Box::new(|_| Resume::NotYet),
        });
        key.unwrap()
    }

    pub fn menu(&mut self, mut on_redraw: impl FnMut(&mut IO, &Menu<'_>)) {
        let menu = Menu::new();
        self.wait(EventLoop {
            on_redraw: Box::new(|io| { on_redraw(io, &menu) }),
            on_exit: Box::new(self.default_on_exit),

            on_keypress: Box::new(|_, k| { 
                if let Handled::Yes = menu.handle(k) {
                    return Resume::Now;
                }
                return Resume::NotYet
            }),
            on_frame: Box::new(|_| Resume::NotYet),
        });
    }

    // TODO: getch alternative that provides a separate `Menu` argument that allows you to register a responder for a key.
    // (or an IO that is configured for that, complete with relevant aliases)
    // The Menu has fn register(k: Key, cb: impl FnMut(&mut IO)) 
    // Doing so adds a handler for k.
    // (Might provide it for k: impl Fn(Key) -> bool too.)

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
        let mut old_aspect = None;
        for iteration in 0 as u64.. {
            let mut window_changed: bool = false;
            if let None = self.window { self.reconstitute_window(); window_changed = true }
            let aspect = self.reconstitute_buffer();  

            let aspect_changed = Some(aspect) != old_aspect;
            old_aspect = Some(aspect);

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
            self.keyboard.add_pressed_keys(win);
            self.keyboard.correlate();

            while let Some(keypress) = self.keyboard.getch() {
                if let Resume::Now = (evt.on_keypress)(self, keypress) { return }
            }

            self.screen.resize(aspect.term_size.cast());
            let needs_redraw = iteration == 0 || aspect_changed || window_changed;
            if needs_redraw || iteration % REDRAW_EVERY == 0  {
                (evt.on_redraw)(self);
                self.draw(aspect);

                let win = self.window.as_mut().unwrap();
                // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
                win
                    .update_with_buffer(&self.buffer, aspect.buf_size.width as usize, aspect.buf_size.height as usize)
                    .unwrap();
            } else {
                let win = self.window.as_mut().unwrap();
                win.update()
            }
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