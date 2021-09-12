#![feature(type_alias_impl_trait)]
extern crate minifb;

#[macro_use] extern crate lazy_static;

mod aliases;
mod cp437;
mod drawing;
mod formatting;
mod geom;
mod rendering;
mod window_management;

use std::process::exit;

use window_management::IO;

fn main() {
    let mut io = IO::new(|_| exit(0));
    io.wait()
}