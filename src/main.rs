use std::{io::stdout, sync::Arc};

use crossterm::{
    cursor::Hide,
    execute,
    terminal::{enable_raw_mode, Clear, ClearType, EnterAlternateScreen},
};
use game::Game;
use input::GameInputHandler;
use log4rs::config::Deserializers;

mod animator;
mod game;
mod input;
mod layout;
mod palette;
mod renderer;
mod sprite;
mod sync;

fn main() {
    let _ = log4rs::init_file("log4rs.yaml", Deserializers::default()).unwrap();
    
    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen, Clear(ClearType::All), Hide).unwrap();

    // Game creation
    let mut g = Game::default();

    // Attach game input listener
    let mut handler = GameInputHandler::default();
    handler.subscribe(&mut g);
    Arc::new(handler).start();

    g.start();
}
