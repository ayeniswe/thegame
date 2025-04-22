mod animator;
mod input;
mod layout;
mod palette;
mod renderer;
mod sprite;

use crossterm::{
    cursor::Hide,
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use input::InputHandler;
use layout::{Coordinate, Mirrorable};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    widgets::Block,
    Terminal,
};
use sprite::character::{character::Character as _, knight};
use std::{
    io::{stdout, Result},
    sync::{mpsc::channel, Arc, Mutex},
    thread::{self},
    time::{Duration, Instant},
};

use animator::Animation;

fn main() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, Clear(ClearType::All), Hide)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // input handling
    let (tx, rx) = channel::<Coordinate>();
    thread::spawn(move || loop {
        if let Ok(input) = InputHandler::handler() {
            let _ = tx.send(input);
        }
    });

    const FPS: u64 = 60;
    const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / FPS);

    let mut def = knight::Knight::new();
    let mut last_pos = Coordinate { x: 0.0, y: 0.0 };
    let mut delta = 0.0;
    let mut last_update = Instant::now();
    const SPEED: f32 = 60.0;
    loop {
        let start = Instant::now();

        // Track new input of character movement
        if let Ok(coor) = rx.try_recv() {
            last_pos += coor * SPEED * delta
        }

        // Show new frame for user visuals
        def.walk().play(
            &mut terminal,
            delta,
            layout::MirrorDirection::None,
            last_pos,
        );

        // Sleep only the remaining time of frame
        let elapsed = start.elapsed();
        if elapsed < FRAME_DURATION {
            thread::sleep(FRAME_DURATION - elapsed);
        }

        let now = Instant::now();
        delta = now.duration_since(last_update).as_secs_f32();
        last_update = now;
    }

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
