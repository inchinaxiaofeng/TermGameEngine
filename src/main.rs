/// Term Game Engine

mod tge;
mod util;

use tge::Game;
use std::io::{self, Write};
use crossterm::terminal;

fn main() {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode().unwrap();

    let mut game = Game::new();
    game.start();

    terminal::disable_raw_mode().unwrap();
}
