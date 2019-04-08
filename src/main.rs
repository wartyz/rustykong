#[macro_use]
extern crate lazy_static;
// https://www.youtube.com/watch?v=FmOYx1WBYTA&list=PLWMUVtnFsZu5PGl7U0Z1p2y2f1tFzBa4C
#[macro_use]
extern crate log;
extern crate sdl2;
extern crate simple_logger;
extern crate core;

mod rusty_kong;

fn main() {
    simple_logger::init().unwrap();
    rusty_kong::game_run();
}