use crate::{CellVector, aliases::CellPoint, rendering::Interactor};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum InputEvent {
    Mouse(MouseEvent),
    Keyboard(KeyEvent),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum MouseEvent {
    Click(MouseButton, CellPoint, Interactor),
    Up(MouseButton, CellPoint, Interactor),
    // wheel, dragging
}

impl MouseEvent {
    pub fn offset(&self, vec: CellVector) -> MouseEvent {
        match *self {
            MouseEvent::Click(mb, cp, int) => 
                MouseEvent::Click(mb, cp + vec, int),
            MouseEvent::Up(mb, cp, int) => 
                MouseEvent::Up(mb, cp + vec, int),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MouseButton {
    Left, Right
}

// TODO: Add an "is_accept()" method that returns true for enter and space
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyEvent {
    pub code: Keycode,  // TODO: Provide a KeyCode enum
    pub shift: bool,
    pub control: bool,
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
