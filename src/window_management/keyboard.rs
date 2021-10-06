use std::{cell::RefCell, collections::{BTreeMap, VecDeque}, rc::Rc};

use minifb::{Key as MinifbKey, Window};

use super::{KeyCombo, input::{KeyEvent, Keycode}};

pub(crate) struct Keyboard {
    correlator: KeyCorrelatorRef
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard { correlator: KeyCorrelatorRef(Rc::new(RefCell::new(KeyCorrelator::new()))) }
    }

    pub fn monitor_minifb_utf32(&mut self, window: &mut Window) {
        window.set_input_callback(Box::new(KeyCorrelatorRef(Rc::clone(&self.correlator.0))))
    }

    pub fn add_keys(&mut self, window: &mut Window) {
        // pressed keys
        if let Some(keys_down) = window.get_keys() {  
            let shift = window.is_key_down(MinifbKey::LeftShift) || window.is_key_down(MinifbKey::RightShift);
            let control = window.is_key_down(MinifbKey::LeftCtrl) || window.is_key_down(MinifbKey::RightCtrl);
            self.correlator.0.borrow_mut().add_keys(&keys_down, shift, control);
        }
    }

    pub fn getch(&mut self) -> Option<KeyEvent> {
        self.correlator.0.borrow_mut().events.pop_front()
    }
}

struct KeyCorrelatorRef(Rc<RefCell<KeyCorrelator>>);

impl minifb::InputCallback for KeyCorrelatorRef {
    fn add_char(&mut self, uni_char: u32) {
        self.0.borrow_mut().utf32_keys.push_back(uni_char)
    }
}

#[derive(Debug)]
struct KeyCorrelator {
    utf32_keys: VecDeque<u32>,  // keycode, age in frames
    keys_down: BTreeMap<MinifbKey, MinifbKeyMode>,
    events: VecDeque<KeyEvent>,
}

#[derive(Clone, Copy, Debug)]
struct MinifbKeyMode {
    shift: bool,
    control: bool,
}

impl KeyCorrelator {
    fn new() -> Self {
        KeyCorrelator {
            utf32_keys: VecDeque::new(),
            keys_down: BTreeMap::new(),
            events: VecDeque::new(),
        }
    }

    fn add_keys(&mut self, new_keys_down: &[MinifbKey], shift: bool, control: bool) {
        // TODO: Preserve order instead of always putting utf32 keys first?
        while let Some(u) = self.utf32_keys.pop_front() {
            let c = if let Some(c) = char::from_u32(u) { c } else {
                continue // don't attempt to map utf32 gibberish
            };

            // see if this is a key where we infer a keycode from a typed character
            if let Some(theoretical_code) = most_likely_keycode(c) {
                self.events.push_back(censor_unhelpful_features(
                    KeyEvent::Press(KeyCombo {
                        shift, control, code: theoretical_code,
                    })
                ))
            }  else {
                // still have to type it!!
                self.events.push_back(censor_unhelpful_features(
                    KeyEvent::Type(c)
                ));
            }
        }

        for key in new_keys_down {
            let code = if let Some(code) = minifb_to_keycode(*key) { code } else { continue };

            if !self.keys_down.contains_key(key) {
                // newly down
                self.events.push_back(censor_unhelpful_features(
                    KeyEvent::Press(KeyCombo { code, shift, control })
                ));
            }
        }
        for (key, details) in self.keys_down.iter() {
            let code = if let Some(code) = minifb_to_keycode(*key) { code } else { continue };

            if !new_keys_down.contains(key) {
                self.events.push_back(censor_unhelpful_features(
                    KeyEvent::Release(KeyCombo { code, shift: details.shift, control: details.control })
                ))
            }
        }
        self.keys_down.clear();
        for key in new_keys_down {
            self.keys_down.insert(*key, MinifbKeyMode { shift, control });
        }
    }
}

fn minifb_to_keycode(key: MinifbKey) -> Option<Keycode> {
    use MinifbKey as M;
    use Keycode::*;

    Some(match key {
        M::Key0 => Key0, M::Key1 => Key1, M::Key2 => Key2, M::Key3 => Key3,
        M::Key4 => Key4, M::Key5 => Key5, M::Key6 => Key6, M::Key7 => Key7,
        M::Key8 => Key8, M::Key9 => Key9,

        M::NumPad0 => Key0, M::NumPad1 => Key1, M::NumPad2 => Key2,
        M::NumPad3 => Key3, M::NumPad4 => Key4, M::NumPad5 => Key5,
        M::NumPad6 => Key6, M::NumPad7 => Key7, M::NumPad8 => Key8,
        M::NumPad9 => Key9, 

        M::A => A, M::B => B, M::C => C, M::D => D, M::E => E, M::F => F,
        M::G => G, M::H => H, M::I => I, M::J => J, M::K => K, M::L => L,
        M::M => M, M::N => N, M::O => O, M::P => P, M::Q => Q, M::R => R,
        M::S => S, M::T => T, M::U => U, M::V => V, M::W => W, M::X => X,
        M::Y => Y, M::Z => Z,

        M::F1 => F1, M::F2 => F2, M::F3 => F3, M::F4 => F4, M::F5 => F5, 
        M::F6 => F6, M::F7 => F7, M::F8 => F8, M::F9 => F9, M::F10 => F10, 
        M::F11 => F11, M::F12 => F12, M::F13 => F13, M::F14 => F14, M::F15 => F15, 

        M::Down => Down, M::Left => Left, M::Right => Right, M::Up => Up,
        M::Apostrophe => Apostrophe, M::Backquote => Backquote,

        M::Backslash => Backslash, M::Comma => Comma, M::Equal => Equal,
        M::LeftBracket => LeftBracket, M::Minus => Minus, M::Period => Period,
        M::RightBracket => RightBracket, M::Semicolon => Semicolon,

        // we get backspaces specifically from text
        M::Slash => Slash, M::Backspace => return None, M::Delete => Delete,
        M::End => End, M::Enter => Enter,

        M::Escape => Escape,

        M::Home => Home, M::Insert => Insert, M::Menu => Menu,

        M::PageDown => PageDown, M::PageUp => PageUp,

        M::Pause => Pause, M::Space => Space, M::Tab => Tab,

        M::NumPadDot => Period, M::NumPadSlash => Slash,
        M::NumPadAsterisk => Asterisk, M::NumPadMinus => Minus,
        M::NumPadPlus => Plus, M::NumPadEnter => Enter,

        M::NumLock | M::CapsLock | M::ScrollLock |
        M::LeftShift | M::RightShift | M::LeftCtrl | M::RightCtrl |
        M::LeftAlt | M::RightAlt | M::LeftSuper | M::RightSuper |
        M::Unknown | M::Count =>
            return None
    })
}

fn most_likely_keycode(c: char) -> Option<Keycode> {
    use Keycode::*;
    Some(match c.to_ascii_uppercase() {
        '\u{08}' => Backspace,
        _ => return None,
    })
}

fn censor_unhelpful_features(mut key: KeyEvent) -> KeyEvent {
    // This just deals with a bunch of miscellaneous things bad input systems might do
    key = match key {
        KeyEvent::Type('\r'|'\n') => 
            KeyEvent::Press(KeyCombo { code: Keycode::Enter, shift: false, control: false }),
        KeyEvent::Type('\t') => 
            KeyEvent::Press(KeyCombo { code: Keycode::Tab, shift: false, control: false }),
        _ => key
    };

    use Keycode::*;
    // Even if the char code wasn't found, try to find it by looking at shift
    key.alter_combo(|combo| {
        let old_key_code = combo.code;
        if combo.shift && !combo.control {
            combo.code = match combo.code {
                Backquote => Tilde, Key1 => Exclamation, Key2 => At, 
                Key3 => Pound, Key4 => Dollar, Key5 => Percent,
                Key6 => Caret, Key7 => Ampersand, Key8 => Asterisk,
                Key9 => LeftParen, Key0 => RightParen, Minus => Underscore,
                Equal => Plus, LeftBracket => LeftBrace,
                RightBracket => RightBrace, Backslash => Pipe,
                Semicolon => Colon, Apostrophe => DoubleQuote,
                Comma => LessThan, Period => GreaterThan,
                Slash => QuestionMark,
                _ => combo.code,
            }
        }
        if combo.code != old_key_code {
            // shifty character!!! because it's inherently shifty, turn off the shift modifier
            combo.shift = false;
        }
    });

    key
}