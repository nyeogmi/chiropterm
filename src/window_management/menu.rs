use std::{cell::{Cell, RefCell}, rc::Rc};

use euclid::point2;

use crate::{CellPoint, IO, rendering::Interactor};

use super::{KeyEvent, MouseEvent, input::{InputEvent}};

// TODO: Clear all interactors in one stroke? Or uh, a sub-menu that generates the None interactor no matter what
// You know, so you can draw a screen with all its menus disabled!

pub struct Menu<'a> {
    state: Rc<MenuState<'a>>,
}

impl<'a> Menu<'a> {
    pub fn new() -> Menu<'a> {
        Menu {
            state: Rc::new(MenuState::new())
        }
    }

    pub fn clear(&self) {
        self.state.clear();
    }

    pub fn share(&self) -> Menu<'a> {
        Menu { state: self.state.clone() }
    }

    pub fn on_tick(&self, cb: impl 'a+FnMut(u64) -> Signal) {
        self.state.on_tick(cb)
    }

    pub fn on_key(&self, k: KeyRecognizer<'a>, cb: impl 'a+FnMut(KeyEvent) -> Signal) {
        self.state.on_key(k, cb, false)
    }

    pub fn on_key_hprio(&self, k: KeyRecognizer<'a>, cb: impl 'a+FnMut(KeyEvent) -> Signal) {
        self.state.on_key(k, cb, true)
    }

    pub fn on_mouse(&self, cb: impl 'a+FnMut(MouseEvent) -> Signal) -> Interactor {
        self.state.on_mouse(cb)
    }

    pub fn on_text(&self, cb: impl 'a+FnMut(char) -> Signal) {
        self.state.on_text(cb, false)
    }

    pub fn on_text_hprio(&self, cb: impl 'a+FnMut(char) -> Signal) {
        self.state.on_text(cb, true)
    }
    
    pub fn mouse_xy(&self) -> CellPoint {
        self.state.mouse_xy.get()
    }

    pub(crate) fn handle(&self, i: InputEvent) -> Option<Signal> {
        self.state.handle(i)
    }
}

pub struct MenuState<'a> {
    handlers: RefCell<Vec<Handler<'a>>>,
    on_tick: RefCell<Option<Handler<'a>>>,
    hprio_key_recognizers: RefCell<Vec<(KeyRecognizer<'a>, Interactor)>>,
    lprio_key_recognizers: RefCell<Vec<(KeyRecognizer<'a>, Interactor)>>,
    mouse_xy: Cell<CellPoint>,
    // TODO: Key handlers again
}

impl<'a> MenuState<'a> {
    pub fn new() -> MenuState<'a> {
        MenuState {
            handlers: RefCell::new(vec![]),
            on_tick: RefCell::new(None),
            hprio_key_recognizers: RefCell::new(vec![]),
            lprio_key_recognizers: RefCell::new(vec![]),
            mouse_xy: Cell::new(point2(-1, -1)),  // will be populated on first tick
        }
    }

    pub fn clear(&self) {
        self.handlers.borrow_mut().clear();
        self.on_tick.borrow_mut().take();
        self.hprio_key_recognizers.borrow_mut().clear();
        self.lprio_key_recognizers.borrow_mut().clear();
    }

    pub fn on_tick(&self, mut cb: impl 'a+FnMut(u64) -> Signal) {
        if self.on_tick.borrow().is_some() {
            panic!("can't have two on_tick callbacks");
        }
        self.on_tick.replace(Some(Handler(Box::new(move |input| {
            match input {
                InputEvent::Tick(t) => { cb(t) }
                _ => unreachable!(),
            }
        }))));
    }
    
    pub fn on_key(&self, k: KeyRecognizer<'a>, mut cb: impl 'a+FnMut(KeyEvent) -> Signal, high_prio: bool) {
        let mut hndl = self.handlers.borrow_mut();
        let ix = hndl.len();
        hndl.push(Handler(Box::new(move |input| {
            match input {
                InputEvent::Keyboard(k) => { cb(k) }
                _ => unreachable!(),
            }
        })));
        let interactor = Interactor::from_index(ix);
        // TODO: Distinguish X and ctrl-X
        let mut krcg = if high_prio {
            self.hprio_key_recognizers.borrow_mut()
        } else {
            self.lprio_key_recognizers.borrow_mut()
        };
        krcg.push((k, interactor)); 
    }

    pub fn on_mouse(&self, mut cb: impl 'a+FnMut(MouseEvent) -> Signal) -> Interactor {
        let mut hndl = self.handlers.borrow_mut();
        let ix = hndl.len();
        hndl.push(Handler(Box::new(move |input| {
            match input {
                InputEvent::Mouse(m) => { cb(m) }
                _ => unreachable!(),
            }
        })));
        Interactor::from_index(ix)
    }

    pub fn on_text(&self, mut cb: impl 'a+FnMut(char) -> Signal, high_prio: bool) {
        let mut hndl = self.handlers.borrow_mut();
        let ix = hndl.len();
        hndl.push(Handler(Box::new(move |input| {
            match input {
                InputEvent::Keyboard(KeyEvent::Type(c)) => { cb(c) }
                _ => unreachable!(),
            }
        })));
        let interactor = Interactor::from_index(ix);
        let mut krcg = if high_prio {
            self.hprio_key_recognizers.borrow_mut()
        } else {
            self.lprio_key_recognizers.borrow_mut()
        };
        krcg.push(
            (
                KeyRecognizer(Box::new(move |key| {
                    if let KeyEvent::Type(_) = key { true }
                    else { false }
                })),
                interactor
            )
        );
    }

    pub(crate) fn handle(&self, i: InputEvent) -> Option<Signal> {
        match i {
            InputEvent::Tick(_) => {
                let mut on_tick = self.on_tick.borrow_mut();
                if let Some(of) = on_tick.as_mut() {
                    return Some((of.0)(i))
                }
                None
            }
            InputEvent::Keyboard(k) => { 
                for (rec, interactor) in self.hprio_key_recognizers.borrow().iter().chain(self.lprio_key_recognizers.borrow().iter()) {
                    if rec.0(k) {
                        if let Some(ix) = interactor.index() {
                            let mut hnd = self.handlers.borrow_mut();
                            if ix < hnd.len() { return Some((hnd[ix].0)(i)); };
                        }
                    }
                }
                None
            }
            InputEvent::Mouse(MouseEvent::Click(_, _, interactor)) => {
                if let Some(ix) = interactor.index() {
                    let mut hnd = self.handlers.borrow_mut();
                    if ix < hnd.len() { return Some((hnd[ix].0)(i)); };
                }
                None
            },
            InputEvent::Mouse(MouseEvent::Wiggle { now_point, now_interactor, .. }) => {
                self.mouse_xy.replace(now_point);

                if let Some(ix) = now_interactor.index() {
                    let mut hnd = self.handlers.borrow_mut();
                    if ix < hnd.len() { return Some((hnd[ix].0)(i)); };
                }
                None
            }
            InputEvent::Mouse(MouseEvent::Drag { start_interactor, .. }) => {
                if let Some(ix) = start_interactor.index() {
                    let mut hnd = self.handlers.borrow_mut();
                    if ix < hnd.len() { return Some((hnd[ix].0)(i)); };
                }
                None
            }
            InputEvent::Mouse(MouseEvent::Scroll(_, _, interactor)) => {
                if let Some(ix) = interactor.index() {
                    let mut hnd = self.handlers.borrow_mut();
                    if ix < hnd.len() { return Some((hnd[ix].0)(i)); };
                }
                None
            },
            InputEvent::Mouse(_) => {
                None
            }
        }
    }
}

struct Handler<'a> (
    Box<dyn 'a+FnMut(InputEvent) -> Signal>,
);

pub struct KeyRecognizer<'a> (
    pub Box<dyn 'a+Fn(KeyEvent) -> bool>,
);

pub enum Signal {
    Break,
    Modal(Box<dyn FnOnce(&mut IO) -> Signal>),
    Continue,
    Refresh,
}