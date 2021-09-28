use std::cell::RefCell;

use crate::rendering::Interactor;

use super::{KeyEvent, MouseButton, MouseEvent, input::{InputEvent, Keycode}};

// TODO: Clear all interactors in one stroke? Or uh, a sub-menu that generates the None interactor no matter what
// You know, so you can draw a screen with all its menus disabled!

pub struct Menu<'a> {
    handlers: RefCell<Vec<Handler<'a>>>,
    key_recognizers: RefCell<Vec<KeyRecognizer<'a>>>,
    // TODO: Key handlers again
}

impl<'a> Menu<'a> {
    pub fn new() -> Menu<'a> {
        Menu {
            handlers: RefCell::new(vec![]),
            key_recognizers: RefCell::new(vec![]),
        }
    }

    // combines on_key and on_click
    pub fn on(&self, k: Keycode, mut cb: impl 'a+FnMut(InputEvent)) -> Interactor {
        let mut hndl = self.handlers.borrow_mut();
        let ix = hndl.len();
        hndl.push(Handler(Box::new(move |input| {
            cb(input);
            Handled::Yes
        })));
        let interactor = Interactor::from_index(ix);
        // TODO: Distinguish X and ctrl-X
        let mut krcg = self.key_recognizers.borrow_mut();
        krcg.push(KeyRecognizer(Box::new(move |key| {
            if key.code == k { return interactor }
            Interactor::none()
        })));
        interactor
    }

    pub fn on_key(&self, k: Keycode, mut cb: impl 'a+FnMut(KeyEvent)) {
        let mut hndl = self.handlers.borrow_mut();
        let ix = hndl.len();
        hndl.push(Handler(Box::new(move |input| {
            match input {
                InputEvent::Keyboard(k) => { cb(k); Handled::Yes }
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

    pub fn on_click(&self, mut cb: impl 'a+FnMut(MouseEvent)) -> Interactor {
        let mut hndl = self.handlers.borrow_mut();
        let ix = hndl.len();
        hndl.push(Handler(Box::new(move |input| {
            match input {
                InputEvent::Mouse(m) => { cb(m); Handled::Yes }
                _ => unreachable!(),
            }
        })));
        Interactor::from_index(ix)
    }

    pub(crate) fn handle(&self, i: InputEvent) -> Handled {
        match i {
            InputEvent::Keyboard(k) => { 
                let kcrg = self.key_recognizers.borrow();
                for rec in kcrg.iter() {
                    let interactor = (rec.0)(k);
                    if let Some(ix) = interactor.index() {
                        let mut hnd = self.handlers.borrow_mut();
                        if ix < hnd.len() { return (hnd[ix].0)(i); };
                    }
                };
                Handled::No
            }
            InputEvent::Mouse(MouseEvent::Click(_, _, interactor)) => {
                if let Some(ix) = interactor.index() {
                    let mut hnd = self.handlers.borrow_mut();
                    if ix < hnd.len() { return (hnd[ix].0)(i); };
                }
                Handled::No
            },
            InputEvent::Mouse(_) => {
                Handled::No
            }
        }
    }
}

struct Handler<'a> (
    Box<dyn 'a+FnMut(InputEvent) -> Handled>,
);

struct KeyRecognizer<'a> (
    Box<dyn 'a+Fn(KeyEvent) -> Interactor>,
);

pub(crate) enum Handled { Yes, No }