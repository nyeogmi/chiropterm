use std::cell::RefCell;

use crate::rendering::Interactor;

use super::{MouseButton, MouseEvent, input::{InputEvent, Keycode}};

pub struct Menu<'a> {
    handlers: RefCell<Vec<Handler<'a>>>
    // TODO: Key handlers again
}

impl<'a> Menu<'a> {
    pub fn new() -> Menu<'a> {
        Menu {
            handlers: RefCell::new(vec![]),
        }
    }

    /*
    pub fn on(&self, k: Keycode, mut cb: impl 'a+FnMut(KeyEvent)) {
        // TODO: Require ctrl/alt to be right
        self.key_handlers.borrow_mut().push(Handler(Box::new(move |k_got| {
            if k_got.code != k { return Handled::No }
            cb(k_got);
            Handled::Yes
        })))
    }
    */

    pub fn interactor(&self, k: Keycode, mut cb: impl 'a+FnMut(InputEvent)) -> Interactor {
        let mut hndl = self.handlers.borrow_mut();
        let ix = hndl.len();
        hndl.push(Handler(Box::new(move |input| {
            cb(input);
            Handled::Yes
        })));
        Interactor::from_index(ix)
    }

    pub(crate) fn handle(&self, i: InputEvent) -> Handled {
        println!("got: {:?}", i);
        match i {
            InputEvent::Keyboard(_) => { Handled::No },
            InputEvent::Mouse(MouseEvent::Click(MouseButton::Left, _, Some(interactor))) => {
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

pub(crate) enum Handled { Yes, No }