
use rusty_wumpus::game;
use rusty_wumpus::util;
use std::env;
fn main() {
    let args: Vec<_> = env::args().collect();
    game::start();
}