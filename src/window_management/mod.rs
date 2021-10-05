mod input;
mod keyboard;
mod math;
mod menu;
mod mouse;
mod redraw_tracking_screen;

use std::time::{Instant};

use euclid::size2;
use minifb::{Scale, ScaleMode, Window, WindowOptions};

use crate::{drawing::Screen, rendering::{self, Interactor, Render, Swatch}, window_management::keyboard::Keyboard};

use self::{math::{calculate_aspect, default_window_size}, mouse::Mouse, redraw_tracking_screen::RedrawTrackingScreen};
pub(crate) use self::math::Aspect;

pub use menu::{Menu, Signal};

pub use self::math::AspectConfig;
pub use input::*;

const TICKS_PER_SECOND: usize = 15;
const APPARENT_TICK_MICROSECONDS: u128 = 66666;  // 15 FPS
const HANDLE_INPUT_EVERY: usize = 4166; // 240 FPS

pub struct IO {
    // user vars
    iteration: u64,
    tick: u64,
    last_tick_at: Option<Instant>,
    window_title: String,
    aspect_config: AspectConfig,

    // io drivers
    window: Option<Window>,
    keyboard: Keyboard,
    mouse: Mouse,

    // renderer state
    buffer: Vec<u32>,
    swatch: Swatch,
    screen: RedrawTrackingScreen,  

    // evt loop default hooks
    default_on_exit: fn(&mut IO),
}

struct EventLoop<'a> {
    on_redraw: Box<dyn 'a+FnMut(&mut IO)>,
    on_exit: Box<dyn 'a+FnMut(&mut IO)>,

    on_input: Box<dyn 'a+FnMut(&mut IO, InputEvent) -> Resume>,
}

enum Resume {
    NotYet,
    PopEvtLoop,
}

macro_rules! handle_resume {
    ( $l:tt, $x:expr ) => {
        // check events
        match $x { 
            Resume::NotYet => {},
            Resume::PopEvtLoop => { break $l; }
        }
    }
}

impl IO {
    pub fn new(window_title: String, aspect_config: AspectConfig, default_on_exit: fn(&mut IO)) -> IO {
        let swatch = *rendering::DEFAULT_SWATCH;

        IO { 
            iteration: 0, tick: 0, last_tick_at: None, window_title, aspect_config,
            
            window: None, keyboard: Keyboard::new(), mouse: Mouse::new(),
            
            buffer: vec![], swatch, screen: RedrawTrackingScreen::new(swatch.default_bg, swatch.default_fg),
            default_on_exit,
        }
    }

    fn reconstitute_window(&mut self) {
        let mut opts = WindowOptions::default();
        opts.scale = Scale::FitScreen;
        opts.scale_mode = ScaleMode::Stretch;
        opts.resize = true;

        let wsz = default_window_size(self.aspect_config);  

        let mut window = Window::new(
            &self.window_title,
            wsz.width as usize, wsz.height as usize,
            opts,
        ).unwrap_or_else(|e| {
            panic!("{}", e); // TODO: Handle some errors
        });
        window.set_background_color(0, 0, 0); // TODO:
        window.limit_update_rate(Some(std::time::Duration::from_micros(HANDLE_INPUT_EVERY as u64)));
        self.keyboard.monitor_minifb_utf32(&mut window);
        self.window = Some(window);
    }

    fn reconstitute_buffer(&mut self) -> Aspect {
        // NOTE: Must be called after reconstitute_window()
        let win = self.window.as_mut().unwrap();
        let (actual_w, actual_h) = win.get_size();

        let aspect = calculate_aspect(self.aspect_config, size2(actual_w as u16, actual_h as u16));

        let buf_len = aspect.buf_size.width as usize * aspect.buf_size.height as usize;
        if self.buffer.len() != buf_len {
            // TODO: Clear old data?
            self.buffer.resize(buf_len, 0); 
        }
        aspect
    }

    pub fn getch(&mut self, mut on_redraw: impl FnMut(&Screen)) -> KeyEvent {
        let mut inp = None;
        self.wait(EventLoop {
            on_redraw: Box::new(|io| { on_redraw(io.screen.target()) }),
            on_exit: Box::new(self.default_on_exit),

            on_input: Box::new(|_, i| { 
                if let InputEvent::Keyboard(k) = i {
                    inp = Some(k); 
                    Resume::PopEvtLoop 
                } else {
                    Resume::NotYet
                }
            }),
        });
        inp.unwrap()
    }

    pub fn menu(&mut self, mut on_redraw: impl FnMut(&Screen, Menu)) {
        loop {
            let mut sig = Some(self.low_level_menu(&mut on_redraw));
            while let Some(s) = sig.take() {
                match s {
                    Signal::Break => { return }
                    Signal::Modal(m) => {
                        sig.replace(m(self));
                    }
                    Signal::Continue => {}
                }
            }
        } 
    }

    fn low_level_menu(&mut self, on_redraw: &mut impl FnMut(&Screen, Menu)) -> Signal {
        let menu = Menu::new();
        let mut cmd = None;
        self.wait(EventLoop {
            on_redraw: Box::new(|io| { on_redraw(io.screen.target(), menu.share()) }),
            on_exit: Box::new(self.default_on_exit),

            on_input: Box::new(|_, i| { 
                if let Some(x) = menu.handle(i) { cmd = Some(x); return Resume::PopEvtLoop; }
                Resume::NotYet
            }),
        });
        cmd.unwrap()
    }


    pub fn sleep(&mut self, time: f64, mut on_redraw: impl FnMut(&Screen)) {
        // TODO: Clear clicks and keys if we're sleeping
        let mut tick = 0;
        self.wait(EventLoop {
            on_redraw: Box::new(|io| on_redraw(io.screen.target())),
            on_exit: Box::new(self.default_on_exit),

            on_input: Box::new(|_, i| {
                match i {
                    InputEvent::Tick(_) => {
                        if tick as f64 / TICKS_PER_SECOND as f64 > time {
                            return Resume::PopEvtLoop;
                        }
                        tick += 1;
                    }
                    _ => {}
                }
                Resume::NotYet
            }),
        });
    }

    fn wait<'a>(&mut self, mut evt: EventLoop<'a>) {
        let mut old_aspect = None;

        'main: for iter_here in self.iteration as u64.. {
            self.iteration = iter_here;

            let mut window_changed: bool = false;
            if let None = self.window { self.reconstitute_window(); window_changed = true }
            let aspect = self.reconstitute_buffer();  

            let aspect_changed = Some(aspect) != old_aspect;
            old_aspect = Some(aspect);

            // set by reconstitute()
            let win = self.window.as_mut().unwrap();

            if !win.is_open() {
                (evt.on_exit)(self);
                self.window = None;
                continue;  // try again
            }

            // redraw (virtually)
            self.screen.resize(aspect.term_size.cast());
            let needs_virtual_redraw = aspect_changed;

            if needs_virtual_redraw {
                self.screen.switch();
                (evt.on_redraw)(self);
            }

            // check events, starting with ticks
            let now = Instant::now();
            let is_new_tick = if let Some(lfa) = self.last_tick_at {
                now.duration_since(lfa).as_micros() > APPARENT_TICK_MICROSECONDS
            } else { true };
            if is_new_tick {
                self.tick += 1;
                handle_resume!('main, (evt.on_input)(self, InputEvent::Tick(self.tick)));
                self.last_tick_at = Some(now);
            }

            // now keyboard etc
            let win = self.window.as_mut().unwrap();
            self.keyboard.add_pressed_keys(win);
            self.keyboard.correlate();
            let cells = &self.screen.target().cells;
            self.mouse.update(aspect, win, |xy| 
                cells.get(xy).map(|i| (i.get().interactor.interactor, i.get().scroll_interactor))
                .unwrap_or((Interactor::none(), Interactor::none()))
            );

            while let Some(keypress) = self.keyboard.getch() {
                handle_resume!('main, (evt.on_input)(self, InputEvent::Keyboard(keypress)));
            }

            while let Some(mouse_evt) = self.mouse.getch() {
                handle_resume!('main, (evt.on_input)(self, InputEvent::Mouse(mouse_evt)));
            }

            let interactor_changed = self.mouse.interactor_changed();

            let needs_physical_redraw = aspect_changed || window_changed || needs_virtual_redraw || interactor_changed;
            if needs_physical_redraw {
                let touched = self.draw(aspect, self.mouse.interactor());

                let win = self.window.as_mut().unwrap();
                if touched {
                    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
                    win
                        .update_with_buffer(&self.buffer, aspect.buf_size.width as usize, aspect.buf_size.height as usize)
                        .unwrap();
                } else {
                    win.update()
                }
            } else {
                let win = self.window.as_mut().unwrap();
                win.update()
            }
        }

        // before returning: make sure window is updated so we don't get duplicate keypresses
        if let Some(win) = self.window.as_mut() {
            win.update()
        }
    }

    // bool: "was it touched?"
    fn draw(&mut self, aspect: Aspect, interactor: Interactor) -> bool {
        self.screen.draw(
            Render { 
                aspect, 
                swatch: self.swatch,
                interactor,
            }, 
            &mut self.buffer
        )
    }
}
