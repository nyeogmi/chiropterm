#![feature(type_alias_impl_trait)]
extern crate minifb;

#[macro_use] extern crate lazy_static;

mod cp437;
mod geom;
mod rendering;
mod spaces;
mod window_management;

use std::process::exit;

use window_management::IO;

fn main() {
    let mut io = IO::new(|_| exit(0));
    io.wait()
}