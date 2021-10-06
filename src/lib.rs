#![feature(cell_update)]
#![feature(const_fn_trait_bound)]
#![feature(type_alias_impl_trait)]
extern crate minifb;

#[macro_use] extern crate lazy_static;

mod aliases;
mod constants;
pub mod cp437;
mod drawing;
mod formatting;
mod rendering;
mod window_management;

pub use aliases::{CellSpace, CellPoint, CellVector, CellSize, CellRect};
pub use drawing::{BoxArt, Brush, Brushable, Screen, Stamp};
pub use formatting::{FSem, Justification};
pub use rendering::{colors, Font, Interactor, SemanticContent};
pub use window_management::{
    AspectConfig,
    InputEvent, 
    Menu,
    MouseEvent, 
    MouseButton, 
    OnKey,
    KeyEvent, 
    Keycode,
    KeyRecognizer,
    IO,
    Signal,
};