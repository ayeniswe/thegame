mod animator;
mod event;
mod game;
mod input;
mod layout;
mod mock;
mod palette;
mod prelude;
mod renderer;
mod sprite;
mod sync;
mod window;

use event::EventHandler;
use game::GameState;
use prelude::*;
use window::GameWindow;

fn main() {
    let mut event_handler = EventHandler::new();

    let mut window = GameWindow::new(320, 180, "The Little Knight".into(), &event_handler).unwrap();
    let screen = window.screen();
    let inner_window = window.window();
    
    event_handler.register_window(inner_window);
    
    let mut game = GameState::new(
        30,
        15.0,
        Coordinate::default(),
        Knight::new(),
        screen,
    );
    event_handler.subscribe_coordinate(&mut game);
    game.start();

    event_handler.start().unwrap();
}
