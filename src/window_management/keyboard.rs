use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use minifb::{Key as MinifbKey, KeyRepeat, Window};

const FRAMES_UNTIL_GIVEUP: u8 = 1; // give up on correlating utf32 characters after n frames

pub struct Keyboard {
    correlator: KeyCorrelatorRef
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard { correlator: KeyCorrelatorRef(Rc::new(RefCell::new(KeyCorrelator::new()))) }
    }

    pub fn monitor_minifb_utf32(&mut self, window: &mut Window) {
        window.set_input_callback(Box::new(KeyCorrelatorRef(Rc::clone(&self.correlator.0))))
    }

    pub fn add_pressed_keys(&mut self, window: &mut Window) {
        if let Some(pressed) = window.get_keys_pressed(KeyRepeat::Yes) {
            let mut corr = self.correlator.0.borrow_mut();
            let shift = window.is_key_down(MinifbKey::LeftShift) || window.is_key_down(MinifbKey::RightShift);
            let control = window.is_key_down(MinifbKey::LeftCtrl) || window.is_key_down(MinifbKey::RightCtrl);
            let alt = window.is_key_down(MinifbKey::LeftAlt) || window.is_key_down(MinifbKey::RightAlt);
            for key in pressed {
                corr.minifb_keys.push(ModalMinifbKey { shift, control, alt, key });
            }
        }
    }

    pub fn correlate(&self) {
        self.correlator.0.borrow_mut().correlate()
    }

    pub fn getch(&self) -> Option<ChiroptermKey> {
        self.correlator.0.borrow_mut().correlated_keys.pop_front()
    }
}

struct KeyCorrelatorRef(Rc<RefCell<KeyCorrelator>>);

impl minifb::InputCallback for KeyCorrelatorRef {
    fn add_char(&mut self, uni_char: u32) {
        self.0.borrow_mut().utf32_keys.push_back((uni_char, 0))
    }
}

#[derive(Debug)]
struct KeyCorrelator {
    utf32_keys: VecDeque<(u32, u8)>,  // keycode, age in frames
    minifb_keys: Vec<ModalMinifbKey>,
    correlated_keys: VecDeque<ChiroptermKey>,
}

#[derive(Clone, Copy, Debug)]
struct ModalMinifbKey {
    shift: bool,
    control: bool,
    alt: bool,
    key: MinifbKey,
}

impl KeyCorrelator {
    fn new() -> Self {
        KeyCorrelator {
            utf32_keys: VecDeque::new(),
            minifb_keys: vec![],
            correlated_keys: VecDeque::new(),
        }
    }

    fn correlate(&mut self) {
        // TODO: Preserve order instead of always putting utf32 keys first?
        let len = self.utf32_keys.len();
        for i in 0..len {
            let (u, age) = self.utf32_keys.pop_front().unwrap();
            let c = if let Some(c) = char::from_u32(u) { c } else {
                continue // don't attempt to map utf32 gibberish
            };

            // TODO: continue; if c is not representable in our display character set

            let chiropt_key = {
                let mut provider = self.minifb_keys.iter().position(|mmk| minifb_provides(*mmk, c, false));
                if let None = provider {
                    provider = self.minifb_keys.iter().position(|mmk| minifb_provides(*mmk, c, true));
                }

                if let Some(i) = provider {
                    let existing = self.minifb_keys.remove(i);
                    ChiroptermKey {
                        code: minifb_to_keycode(existing.key).or(most_likely_keycode(c)).unwrap_or(Keycode::Unknown),
                        shift: existing.shift,
                        control: existing.control,
                        alt: existing.alt,
                        char: Some(c),
                    }
                } else if age > FRAMES_UNTIL_GIVEUP {
                    if let Some(code) = most_likely_keycode(c) {
                        ChiroptermKey {
                            code,
                            shift: false,
                            control: false,
                            alt: false,
                            char: Some(c),
                        }
                    } else {
                        continue
                    }
                } else {
                    self.utf32_keys.push_back((u, age + 1));
                    continue
                }
            };
            self.correlated_keys.push_back(censor_unhelpful_features(chiropt_key))
        }

        for mmk in self.minifb_keys.drain(..) {
            if let Some(chiropt_keycode) = minifb_to_keycode(mmk.key)  {
                self.correlated_keys.push_back(censor_unhelpful_features(ChiroptermKey {
                    code: chiropt_keycode,
                    shift: mmk.shift,
                    control: mmk.control,
                    alt: mmk.alt,
                    // char: most_likely_char(chiropt_keycode),
                    char: None,
                }))
            }
        }
    }
}

// TODO: Add an "is_accept()" method that returns true for enter and space
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChiroptermKey {
    pub code: Keycode,  // TODO: Provide a KeyCode enum
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub char: Option<char>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum Keycode {
    // Unashamedly inspired by a similar enum from minifb
    Key0 = 0, Key1 = 1, Key2 = 2, Key3 = 3, Key4 = 4,
    Key5 = 5, Key6 = 6, Key7 = 7, Key8 = 8, Key9 = 9,

    A = 10, B = 11, C = 12, D = 13, E = 14, F = 15,
    G = 16, H = 17, I = 18, J = 19, K = 20, L = 21,
    M = 22, N = 23, O = 24, P = 25, Q = 26, R = 27,
    S = 28, T = 29, U = 30, V = 31, W = 32, X = 33,
    Y = 34, Z = 35,

    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15,

    Down, Left, Right, Up,
    Apostrophe, Backquote,

    Backslash, Comma, Equal, LeftBracket,
    Minus, Period, RightBracket, Semicolon,
    Slash, Backspace, Delete, End, Enter,

    Escape,
    Home, Insert, Menu,
    PageDown, PageUp,
    Pause, 
    
    Space, Tab,

    // TODO: Shift punctuation
    Tilde,
    Exclamation, At, Pound, Dollar, Percent, Caret, Ampersand, Asterisk,
    LeftParen, RightParen, Underscore, Plus, LeftBrace, RightBrace,
    Pipe, Colon, DoubleQuote,
    LessThan, GreaterThan, QuestionMark,

    Unknown,

    // don't include Lock, Shift, Alt, Super, and Ctrl -- terminals don't respond to 
    // these by themselves

    // don't expose NumPad keys separately: terminals don't know the difference
    // and doing so encourages developers to make UIs that won't work on most laptops
}


fn minifb_provides(mmk: ModalMinifbKey, utf: char, desperate: bool) -> bool {
    if !desperate {
        if mmk.control || mmk.alt {
            return false
        }
    }

    use MinifbKey::*;
    match (mmk.key, utf.to_ascii_uppercase()) {
        (Space, ' ') | (Tab, '\t') =>
            return true,
        (Enter, '\n') | (Enter, '\r') =>
            return true,
        (A, 'A') | (B, 'B') | (C, 'C') | (D, 'D') | (E, 'E') | (F, 'F') | (G, 'G') | (H, 'H') |
        (I, 'I') | (J, 'J') | (K, 'K') | (L, 'L') | (M, 'M') | (N, 'N') | (O, 'O') | (P, 'P') |
        (Q, 'Q') | (R, 'R') | (S, 'S') | (T, 'T') | (U, 'U') | (V, 'V') | (W, 'W') | (X, 'X') |
        (Y, 'Y') | (Z, 'Z') => 
            return true,
        (Key0, '0') | (Key1, '1') | (Key2, '2') | (Key3, '3') | (Key4, '4') | 
        (Key5, '5') | (Key6, '6') | (Key7, '7') | (Key8, '8') | (Key9, '9') => 
            return true,
        (Apostrophe, '\'') | (Backquote, '`') =>
            return true,
        (Backslash, '\\') | (Comma, ',') | (Equal, '=') | (LeftBracket, '[') |
        (Minus, '-') | (Period, '.') | (RightBracket, ']') | (Semicolon, ';') |
        (Slash, '/') =>
            return true,

        (Backquote, '~') | 
        (Key1, '!') | (NumPad1, '!') | (Key2, '@') | (NumPad2, '@') |
        (Key3, '#') | (NumPad3, '#') | (Key4, '$') | (NumPad4, '$') |
        (Key5, '%') | (NumPad5, '%') | (Key6, '^') | (NumPad6, '^') |
        (Key7, '&') | (NumPad7, '&') | (Key8, '*') | (NumPad8, '*') | 
        (Key9, '(') | (NumPad9, '(') | (Key0, ')') | (NumPad0, ')') =>
            return true,

        (Minus, '_') | (Equal, '+') | (LeftBracket, '{') | (RightBracket, '}') | (Backslash, '|') | 
        (Semicolon, ':') | (Apostrophe, '"') | (Comma, '<') | (Period, '>') | (Slash, '?') =>
            return true,

        (NumPadAsterisk, '*') | (NumPadDot, '.') | (NumPadEnter, '\n') | (NumPadEnter, '\r') |
        (NumPadMinus, '-') | (NumPadPlus, '+') | (NumPadSlash, '/') =>
            return true,

        (NumPad0, '0') | (NumPad1, '1') | (NumPad2, '2') | (NumPad3, '3') | (NumPad4, '4') | 
        (NumPad5, '5') | (NumPad6, '6') | (NumPad7, '7') | (NumPad8, '8') | (NumPad9, '9') 
            if desperate =>
                return true,
        
        (NumPadDot, '?') | (NumPadMinus, '+') | (NumPadSlash, '?') 
            if desperate =>
                return true,

        _ => {}
    }

    false
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

        M::Slash => Slash, M::Backspace => Backspace, M::Delete => Delete,
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
        '0' => Key0, '1' => Key1, '2' => Key2, '3' => Key3, '4' => Key4,
        '5' => Key5, '6' => Key6, '7' => Key7, '8' => Key8, '9' => Key9,

        'A' => A, 'B' => B, 'C' => C, 'D' => D, 'E' => E, 'F' => F,
        'G' => G, 'H' => H, 'I' => I, 'J' => J, 'K' => K, 'L' => L,
        'M' => M, 'N' => N, 'O' => O, 'P' => P, 'Q' => Q, 'R' => R,
        'S' => S, 'T' => T, 'U' => U, 'V' => V, 'W' => W, 'X' => X,
        'Y' => Y, 'Z' => Z,

        '\'' => Apostrophe, '`' => Backquote,

        '\\' => Backslash, ',' => Comma, '=' => Equal, '[' => LeftBracket,
        '-' => Minus, '.' => Period, ']' => RightBracket, ';' => Semicolon,
        '/' => Slash, '\n' => Enter, '\r' => Enter,

        ' ' => Space, '\t' => Tab,
        _ => return None,
    })
}

fn censor_unhelpful_features(mut key: ChiroptermKey) -> ChiroptermKey {
    // This just deals with a bunch of miscellaneous things bad input systems might do
    if let Some('\r') | Some('\n') | Some('\t') = key.char {
        key.char = None;
    }

    use Keycode::*;
    // Try really hard to map shifty chars to punctuation codes
    let old_key_code = key.code;
    if let Some(c) = key.char {
        key.code = match c {
            '~' => Tilde, '!' => Exclamation, '@' => At, '#' => Pound,
            '$' => Dollar, '%' => Percent, '^' => Caret, '&' => Ampersand,
            '*' => Asterisk, '(' => LeftParen, ')' => RightParen,
            '_' => Underscore, '+' => Plus, '{' => LeftBrace,
            '}' => RightBrace, '|' => Pipe, ':' => Colon,
            '"' => DoubleQuote, '<' => LessThan, '>' => GreaterThan,
            '?' => QuestionMark,
            _ => key.code,
        }
    }

    // Even if the char code wasn't found, try to find it by looking at shift
    if key.shift && !key.control && !key.alt {
        key.code = match key.code {
            Backquote => Tilde, Key1 => Exclamation, Key2 => At, 
            Key3 => Pound, Key4 => Dollar, Key5 => Percent,
            Key6 => Caret, Key7 => Ampersand, Key8 => Asterisk,
            Key9 => LeftParen, Key0 => RightParen, Minus => Underscore,
            Equal => Plus, LeftBracket => LeftBrace,
            RightBracket => RightBrace, Backslash => Pipe,
            Semicolon => Colon, Apostrophe => DoubleQuote,
            Comma => LessThan, Period => GreaterThan,
            Slash => QuestionMark,
            _ => key.code,
        }
    }
    if key.code != old_key_code {
        // shifty character!!! because it's inherently shifty, turn off the shift modifier
        key.shift = false;
    }

    key
}