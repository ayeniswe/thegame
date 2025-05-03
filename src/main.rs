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
use window::{GameWindow, WindowDesigner};

fn main() {
    let mut event_handler = EventHandler::new();

    let game_window_instance = GameWindow::new(160, 90, "The Little Knight".into(), &event_handler).unwrap();
    let game_screen = game_window_instance.screen();
    let game_window = game_window_instance.window();

    let designer_window_instance = WindowDesigner::new(90, 180, "The Little Knight - Designer".into(), &event_handler).unwrap();
    let designer_window = designer_window_instance.window();
    
    let mut game = GameState::new(
        30,
    15.0,
        Coordinate::default(),
        Knight::new(),
        game_screen,
    );
    event_handler.subscribe_coordinate(&mut game);
    game.start();

    event_handler.register_window(game_window);
    event_handler.register_window(designer_window);
    event_handler.start().unwrap();
}
