use std::{cell::RefCell, rc::Rc};

use crate::rendering::Interactor;

use super::{KeyEvent, MouseEvent, input::{InputEvent, Keycode}};

// TODO: Clear all interactors in one stroke? Or uh, a sub-menu that generates the None interactor no matter what
// You know, so you can draw a screen with all its menus disabled!

pub struct Menu<'a, T> {
    state: Rc<MenuState<'a, T>>,
}

impl<'a, T> Menu<'a, T> {
    pub fn new() -> Menu<'a, T> {
        Menu {
            state: Rc::new(MenuState::new())
        }
    }

    pub fn share(&self) -> Menu<'a, T> {
        Menu { state: self.state.clone() }
    }

    pub fn on(&self, k: Keycode, cb: impl 'a+FnMut(InputEvent) -> Signal<T>) -> Interactor {
        self.state.on(k, cb)
    }

    pub fn on_key(&self, k: Keycode, cb: impl 'a+FnMut(KeyEvent) -> Signal<T>) {
        self.state.on_key(k, cb)
    }

    pub fn on_click(&self, cb: impl 'a+FnMut(MouseEvent) -> Signal<T>) -> Interactor {
        self.state.on_click(cb)
    }

    pub fn on_text(&self, cb: impl 'a+FnMut(char) -> Signal<T>) {
        self.state.on_text(cb)
    }

    pub(crate) fn handle(&self, i: InputEvent) -> Option<Signal<T>> {
        self.state.handle(i)
    }
}

pub struct MenuState<'a, T> {
    handlers: RefCell<Vec<Handler<'a, T>>>,
    key_recognizers: RefCell<Vec<KeyRecognizer<'a>>>,
    // TODO: Key handlers again
}

impl<'a, T> MenuState<'a, T> {
    pub fn new() -> MenuState<'a, T> {
        MenuState {
            handlers: RefCell::new(vec![]),
            key_recognizers: RefCell::new(vec![]),
        }
    }

    // combines on_key and on_click
    pub fn on(&self, k: Keycode, mut cb: impl 'a+FnMut(InputEvent) -> Signal<T>) -> Interactor {
        let mut hndl = self.handlers.borrow_mut();
        let ix = hndl.len();
        hndl.push(Handler(Box::new(move |input| { cb(input) })));
        let interactor = Interactor::from_index(ix);
        // TODO: Distinguish X and ctrl-X
        let mut krcg = self.key_recognizers.borrow_mut();
        krcg.push(KeyRecognizer(Box::new(move |key| {
            if key.code == k { return interactor }
            Interactor::none()
        })));
        interactor
    }

    pub fn on_key(&self, k: Keycode, mut cb: impl 'a+FnMut(KeyEvent) -> Signal<T>) {
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
        let mut krcg = self.key_recognizers.borrow_mut();
        krcg.push(KeyRecognizer(Box::new(move |key| {
            if key.code == k { return interactor }
            Interactor::none()
        })));
    }

    pub fn on_click(&self, mut cb: impl 'a+FnMut(MouseEvent) -> Signal<T>) -> Interactor {
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

    pub fn on_text(&self, mut cb: impl 'a+FnMut(char) -> Signal<T>) {
        let mut hndl = self.handlers.borrow_mut();
        let ix = hndl.len();
        hndl.push(Handler(Box::new(move |input| {
            match input {
                InputEvent::Keyboard(k) => { cb(k.char.unwrap()) }
                _ => unreachable!(),
            }
        })));
        let interactor = Interactor::from_index(ix);
        let mut krcg = self.key_recognizers.borrow_mut();
        krcg.push(KeyRecognizer(Box::new(move |key| {
            if let Some(_) = key.char { return interactor; }
            Interactor::none()
        })));
    }

    pub(crate) fn handle(&self, i: InputEvent) -> Option<Signal<T>> {
        match i {
            InputEvent::Keyboard(k) => { 
                let kcrg = self.key_recognizers.borrow();
                for rec in kcrg.iter() {
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

struct Handler<'a, T> (
    Box<dyn 'a+FnMut(InputEvent) -> Signal<T>>,
);

struct KeyRecognizer<'a> (
    Box<dyn 'a+Fn(KeyEvent) -> Interactor>,
);

pub enum Signal<T> {
    Break(T),
    Continue,
}