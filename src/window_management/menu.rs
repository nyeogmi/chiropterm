use std::{cell::RefCell, rc::Rc};

use crate::{IO, rendering::Interactor};

use super::{KeyEvent, MouseEvent, input::{InputEvent, Keycode}};

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

    pub fn share(&self) -> Menu<'a> {
        Menu { state: self.state.clone() }
    }

    pub fn on_key(&self, k: Keycode, cb: impl 'a+FnMut(KeyEvent) -> Signal) {
        self.state.on_key(k, cb, false)
    }

    pub fn on_key_hprio(&self, k: Keycode, cb: impl 'a+FnMut(KeyEvent) -> Signal) {
        self.state.on_key(k, cb, true)
    }

    pub fn on_click(&self, cb: impl 'a+FnMut(MouseEvent) -> Signal) -> Interactor {
        self.state.on_click(cb)
    }

    pub fn on_text(&self, cb: impl 'a+FnMut(char) -> Signal) {
        self.state.on_text(cb, false)
    }

    pub fn on_text_hprio(&self, cb: impl 'a+FnMut(char) -> Signal) {
        self.state.on_text(cb, true)
    }

    pub(crate) fn handle(&self, i: InputEvent) -> Option<Signal> {
        self.state.handle(i)
    }
}

pub struct MenuState<'a> {
    handlers: RefCell<Vec<Handler<'a>>>,
    on_tick: RefCell<Option<Handler<'a>>>,
    hprio_key_recognizers: RefCell<Vec<KeyRecognizer<'a>>>,
    lprio_key_recognizers: RefCell<Vec<KeyRecognizer<'a>>>,
    // TODO: Key handlers again
}

impl<'a> MenuState<'a> {
    pub fn new() -> MenuState<'a> {
        MenuState {
            handlers: RefCell::new(vec![]),
            on_tick: RefCell::new(None),
            hprio_key_recognizers: RefCell::new(vec![]),
            lprio_key_recognizers: RefCell::new(vec![]),
        }
    }

    pub fn on_key(&self, k: Keycode, mut cb: impl 'a+FnMut(KeyEvent) -> Signal, high_prio: bool) {
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
        krcg.push(KeyRecognizer(Box::new(move |key| {
            if key.code == k { return interactor }
            Interactor::none()
        })));
    }

    pub fn on_click(&self, mut cb: impl 'a+FnMut(MouseEvent) -> Signal) -> Interactor {
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
                InputEvent::Keyboard(k) => { cb(k.char.unwrap()) }
                _ => unreachable!(),
            }
        })));
        let interactor = Interactor::from_index(ix);
        let mut krcg = if high_prio {
            self.hprio_key_recognizers.borrow_mut()
        } else {
            self.lprio_key_recognizers.borrow_mut()
        };
        krcg.push(KeyRecognizer(Box::new(move |key| {
            if let Some(_) = key.char { return interactor; }
            Interactor::none()
        })));
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
                for rec in self.hprio_key_recognizers.borrow().iter().chain(self.lprio_key_recognizers.borrow().iter()) {
                    let interactor = (rec.0)(k);
                    if let Some(ix) = interactor.index() {
                        let mut hnd = self.handlers.borrow_mut();
                        if ix < hnd.len() { return Some((hnd[ix].0)(i)); };
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

struct KeyRecognizer<'a> (
    Box<dyn 'a+Fn(KeyEvent) -> Interactor>,
);

pub enum Signal {
    Break,
    Modal(Box<dyn FnOnce(&mut IO) -> Signal>),
    Continue,
}

trait DoMenu {
}