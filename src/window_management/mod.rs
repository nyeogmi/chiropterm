mod input;
mod keyboard;
mod math;
mod menu;
mod mouse;

use euclid::size2;
use minifb::{Scale, ScaleMode, Window, WindowOptions};

use crate::{drawing::Screen, rendering::{self, Interactor, Render, Swatch}, window_management::keyboard::Keyboard};

use self::{math::{calculate_aspect, default_window_size}, mouse::Mouse};
pub(crate) use self::math::Aspect;

pub use menu::Menu;
use menu::Handled;

pub use self::math::AspectConfig;
pub use input::*;

const FRAMES_PER_SECOND: usize = 30;
const TRUE_FRAME_DURATION: usize = 1660; // 600 FPS
const REDRAW_EVERY: u64 = 20;   // practically 30 FPS

pub struct IO {
    // user vars
    frame: u64,
    window_title: String,
    aspect_config: AspectConfig,

    // io drivers
    window: Option<Window>,
    keyboard: Keyboard,
    mouse: Mouse,

    // renderer state
    buffer: Vec<u32>,
    swatch: Swatch,
    pub screen: Screen,  // TODO: Make private again later

    // evt loop default hooks
    default_on_exit: fn(&mut IO),
}

struct EventLoop<'a> {
    on_redraw: Box<dyn 'a+FnMut(&mut IO)>,
    on_exit: Box<dyn 'a+FnMut(&mut IO)>,

    on_input: Box<dyn 'a+FnMut(&mut IO, InputEvent) -> Resume>,
    on_frame: Box<dyn 'a+FnMut(&mut IO) -> Resume>,
}

enum Resume {
    NotYet,
    Now,
}

impl IO {
    pub fn new(window_title: String, aspect_config: AspectConfig, default_on_exit: fn(&mut IO)) -> IO {
        let swatch = *rendering::DEFAULT_SWATCH;

        IO { 
            frame: 0, window_title, aspect_config,
            
            window: None, keyboard: Keyboard::new(), mouse: Mouse::new(),
            
            buffer: vec![], swatch, screen: Screen::new(swatch.default_bg, swatch.default_fg),
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
        window.limit_update_rate(Some(std::time::Duration::from_micros(TRUE_FRAME_DURATION as u64)));
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

    pub fn getch(&mut self, on_redraw: impl FnMut(&mut IO)) -> InputEvent {
        let mut inp = None;
        self.wait(EventLoop {
            on_redraw: Box::new(on_redraw),
            on_exit: Box::new(self.default_on_exit),

            on_input: Box::new(|_, i| { inp = Some(i); Resume::Now }),
            on_frame: Box::new(|_| Resume::NotYet),
        });
        inp.unwrap()
    }

    pub fn menu(&mut self, mut on_redraw: impl FnMut(&mut IO, &Menu<'_>)) {
        let menu = Menu::new();
        self.wait(EventLoop {
            on_redraw: Box::new(|io| { on_redraw(io, &menu) }),
            on_exit: Box::new(self.default_on_exit),

            on_input: Box::new(|_, i| { 
                if let Handled::Yes = menu.handle(i) { return Resume::Now; }
                Resume::NotYet
            }),
            on_frame: Box::new(|_| Resume::NotYet),
        });
    }


    pub fn sleep(&mut self, time: f64, on_redraw: impl FnMut(&mut IO)) {
        // TODO: Clear clicks and keys if we're sleeping
        let mut frame = 0;
        self.wait(EventLoop {
            on_redraw: Box::new(on_redraw),
            on_exit: Box::new(self.default_on_exit),

            on_input: Box::new(|_, __| Resume::NotYet),
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

            // set by reconstitute()
            let win = self.window.as_mut().unwrap();

            if !win.is_open() {
                (evt.on_exit)(self);
                self.window = None;
                continue;  // try again
            }

            // redraw (virtually)
            self.screen.resize(aspect.term_size.cast());
            let needs_virtual_redraw = iteration == 0 || aspect_changed;

            if needs_virtual_redraw {
                self.screen.clear();
                (evt.on_redraw)(self);
            }

            // check events
            if let Resume::Now = (evt.on_frame)(self) { return }

            let win = self.window.as_mut().unwrap();
            self.keyboard.add_pressed_keys(win);
            self.keyboard.correlate();
            let cells = &self.screen.cells;
            self.mouse.update(aspect, win, |xy| 
                cells.get(xy).map(|i| i.get().interactor).unwrap_or(Interactor::none())
            );

            while let Some(keypress) = self.keyboard.getch() {
                if let Resume::Now = (evt.on_input)(self, InputEvent::Keyboard(keypress)) { return }
            }

            while let Some(mouse_evt) = self.mouse.getch() {
                if let Resume::Now = (evt.on_input)(self, InputEvent::Mouse(mouse_evt)) { return }
            }

            let interactor_changed = self.mouse.interactor_changed();

            let needs_physical_redraw = iteration == 0 || aspect_changed || window_changed || needs_virtual_redraw || interactor_changed;
            if needs_physical_redraw || iteration % REDRAW_EVERY == 0  {
                self.draw(aspect, self.mouse.interactor());

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

    fn draw(&mut self, aspect: Aspect, interactor: Interactor) {
        self.frame += 1;
        Render { 
            aspect, 
            buffer: &mut self.buffer, 
            swatch: self.swatch,
            screen: &self.screen,
        }.draw(interactor)
    }
}