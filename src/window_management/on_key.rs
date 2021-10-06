use crate::Keycode;

use super::{KeyCombo, KeyRecognizer};

#[derive(Clone)]
pub struct OnKey {
    keycode: Keycode,
    control: bool,
    shift: bool,
}

impl OnKey {
    pub fn only(keycode: Keycode) -> OnKey {
        OnKey { keycode, control: false, shift: false, }
    }

    fn match_for(&self, combo: Option<KeyCombo>) -> bool {
        let combo = if let Some(c) = combo { c }
        else { return false };

        self.keycode == combo.code && 
        self.control == combo.control &&
        self.shift == combo.shift
    }

    pub fn up_or_down(self) -> KeyRecognizer<'static> {
        KeyRecognizer(Box::new(move |key| {
            if !self.match_for(key.get_combo()) { return false }
            match key {
                crate::KeyEvent::Press(_) => true,
                crate::KeyEvent::Release(_) => true,
                crate::KeyEvent::Type(_) => false,
            }
        }))
    }

    pub fn pressed(self) -> KeyRecognizer<'static> {
        KeyRecognizer(Box::new(move |key| {
            if !self.match_for(key.get_combo()) { return false }
            match key {
                crate::KeyEvent::Press(_) => true,
                crate::KeyEvent::Release(_) => false,
                crate::KeyEvent::Type(_) => false,
            }
        }))
    }

    pub fn pressed_or_retriggered(self) -> KeyRecognizer<'static> {
        KeyRecognizer(Box::new(move |key| {
            if !self.match_for(key.get_combo()) { return false }
            match key {
                crate::KeyEvent::Press(_) => true,
                crate::KeyEvent::Release(_) => false,
                crate::KeyEvent::Type(_) => false,
            }
        }))
    }

    pub fn released(self) -> KeyRecognizer<'static> {
        KeyRecognizer(Box::new(move |key| {
            if !self.match_for(key.get_combo()) { return false }
            match key {
                crate::KeyEvent::Press(_) => false,
                crate::KeyEvent::Release(_) => true,
                crate::KeyEvent::Type(_) => false,
            }
        }))
    }
}