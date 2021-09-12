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

use drawing::Brushlike;
use euclid::{point2, vec2};
use formatting::{FSem, Justification, Preformatter};
use rendering::{Font, SemanticContent};
use window_management::IO;

fn main() {
    let mut io = IO::new(*rendering::DEFAULT_SWATCH);
    io.wait(
        |io| {
            for (at, sem) in (Preformatter {
                font: Font::Set,
                first_width_chars: Some(40),
                main_width_chars: Some(40),
                justification: Justification::Left,
            }.to_stamp("WELCOME TO").iter()) {
                io.screen.draw(at + vec2(2, 2), sem)
            }

            for (at, sem) in (Preformatter {
                font: Font::Fat,
                first_width_chars: Some(40),
                main_width_chars: Some(40),
                justification: Justification::Left,
            }.to_stamp("BATCON").iter()) {
                io.screen.draw(at + vec2(24, 2), sem)
            }

            for (at, sem) in (Preformatter {
                font: Font::Small,
                first_width_chars: Some(40),
                main_width_chars: Some(40),
                justification: Justification::Left,
            }.to_stamp("TM").iter()) {
                io.screen.draw(at + vec2(36, 2), sem)
            }

            for (at, sem) in (Preformatter {
                font: Font::Normal,
                first_width_chars: Some(40),
                main_width_chars: Some(40),
                justification: Justification::Left,
            }.to_stamp("the premier convention for all the bats").iter()) {
                io.screen.draw(at + vec2(2, 4), sem)
            }
        },
        |_| exit(0)
    );
}