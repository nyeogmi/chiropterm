use std::cell::RefCell;

use super::input::{InputEvent, KeyEvent, Keycode};

pub struct Menu<'a> {
    key_handlers: RefCell<Vec<Handler<'a>>>
}

impl<'a> Menu<'a> {
    pub fn new() -> Menu<'a> {
        Menu {
            key_handlers: RefCell::new(vec![]),
        }
    }

    pub fn on(&self, k: Keycode, mut cb: impl 'a+FnMut(KeyEvent)) {
        // TODO: Require ctrl/alt to be right
        self.key_handlers.borrow_mut().push(Handler(Box::new(move |k_got| {
            if k_got.code != k { return Handled::No }
            cb(k_got);
            Handled::Yes
        })))
    }

    pub(crate) fn handle(&self, i: InputEvent) -> Handled {
        // println!("got: {:?}", i);
        match i {
            InputEvent::Keyboard(k) =>  {
                for h in self.key_handlers.borrow_mut().iter_mut() {
                    if let Handled::Yes = (h.0)(k) { return Handled::Yes }
                };
                Handled::No
            }
            _ => Handled::No,  // TODO: Menus support mice too, ideally!
        }
    }
}

struct Handler<'a> (
    Box<dyn 'a+FnMut(KeyEvent) -> Handled>,
);

pub(crate) enum Handled { Yes, No }