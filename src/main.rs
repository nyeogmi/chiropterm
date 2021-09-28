#![feature(cell_update)]
#![feature(type_alias_impl_trait)]
extern crate minifb;

#[macro_use] extern crate lazy_static;

mod aliases;
mod cp437;
mod drawing;
mod formatting;
mod rendering;
mod window_management;

use std::process::exit;

use drawing::Brushable;
use aliases::*;
use rendering::Font;
use window_management::IO;

use crate::{window_management::Keycode};

fn main() {
    let mut io = IO::new(*rendering::DEFAULT_SWATCH, |_| exit(0));

    io.menu(
        |io, menu| {
            let content_box = io.screen.brush().region(io.screen.rect().inflate(-2, -2));

            let interactor_one = menu.on(Keycode::B, |k| {
                println!("hit {:?}", k)
            });

            let interactor_two = menu.on(Keycode::B, |k| {
                println!("hit {:?}", k)
            });

            let b = content_box.at(point2(0, 0))
            .bg(10).fg(15)
            .font(Font::Set).putfs("WELCOME TO ")
            .bg(2).font(Font::Fat).interactor(interactor_one).putfs("BATCON").no_interactor()
            .font(Font::Small).putfs("TM").font(Font::Fat); // fat again (so the newline will work)

            b.bg(8).fg(7).on_newline().font(Font::Normal).interactor(interactor_two).putfs(concat!(
                "the premier convention for all the bats ",
                "and all the big bats and all the little ",
                "bats and the bats and the bats",
            ));
        },
    );
    
    io.sleep(
        1.0,
        |io| {
            let content_box = io.screen.brush().region(io.screen.rect().inflate(-2, -2));

            let b = content_box.at(point2(0, 0))
            .bg(10).fg(15)
            .font(Font::Set).putfs("PLEASE WAIT");
        }
    );
}