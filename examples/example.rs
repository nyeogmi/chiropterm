use std::process::exit;

use chiropterm::*;
use euclid::*;

const ASPECT_CONFIG: AspectConfig = AspectConfig {
    pref_min_term_size: size2(64, 48),  // but expect ~112x60
    pref_max_term_size: size2(256, 256),
};

fn main() {
    use colors::*;

    let mut io = IO::new(
        "Chiropterm example".to_string(), 
        ASPECT_CONFIG, 
        |_| exit(0)
    );

    io.menu(
        |out, menu| {
            let ramp_bg = Purple;
            let ramp_fg = Yellow;

            let content_box = out.brush().region(out.rect().inflate(-2, -2));

            content_box.fill(FSem::new().bg(ramp_bg[1]));
            content_box.bevel_w95_sleek(ramp_bg[0], ramp_bg[3]);

            let interactor_one = menu.on(Keycode::B, |k| {
                println!("hit (1) {:?}", k)
            });

            let interactor_two = menu.on(Keycode::C, |k| {
                println!("hit (2) {:?}", k)
            });

            let b = content_box.at(point2(0, 0))
            .bg(ramp_bg[2]).fg(ramp_fg[2])
            .font(Font::Set).putfs("WELCOME TO ")
            .bg(ramp_bg[3]).font(Font::Fat).interactor(interactor_one, ramp_fg[2], ramp_bg[3]).putfs("BATCON").no_interactor()
            .font(Font::Small).putfs("TM").font(Font::Fat); // fat again (so the newline will work)

            b.bg(ramp_bg[0]).fg(ramp_fg[3]).on_newline().font(Font::Normal).interactor(interactor_two, ramp_fg[3], ramp_bg[0]).putfs(concat!(
                "the premier convention for all the bats ",
                "and all the big bats and all the little ",
                "bats and the bats and the bats",
            ));
        },
    );
    
    io.sleep(
        1.0,
        |out| {
            let content_box = out.brush().region(out.rect().inflate(-2, -2));

            content_box.at(point2(0, 0))
            .bg(10).fg(15)
            .font(Font::Set).putfs("PLEASE WAIT");
        }
    );
}