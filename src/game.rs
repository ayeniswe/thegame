use crossbeam::channel::Receiver;
use ratatui::{
    prelude::{Backend, CrosstermBackend},
    Terminal,
};
use std::{
    io::{stdout, Stdout},
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{
    layout::{Coordinate, MirrorDirection},
    sprite::character::{character::Character, knight::Knight},
    sync::Subscriber,
};

pub(crate) struct Game<B: Backend> {
    input_handler: Option<Receiver<Coordinate>>,
    delta: f32,
    player: Box<dyn Character<B>>,
    terminal: Terminal<B>,
    player_pos: Coordinate,
    player_speed: f32,
    fps: u64,
}
impl<B: Backend> Game<B> {
    pub(crate) fn start(&mut self) {
        let frame_duration: Duration = Duration::from_micros(1_000_000 / self.fps);
        loop {
            // Track movement
            if let Some(rx) = &self.input_handler {
                if let Ok(input) = rx.try_recv() {
                    self.update_player_pos(input);
                }
            }

            // Frame animation
            let tick = Instant::now();
            let t = self.player.idle();
            t.play(
                &mut self.terminal,
                self.delta,
                MirrorDirection::None,
                self.player_pos,
            );
            // Guarantee frames arent cut short and
            // exhaust their max view time
            let elapsed = tick.elapsed();
            if elapsed < frame_duration {
                sleep(frame_duration - elapsed)
            }

            // Keep frame-rate independent and consistent
            self.delta = Instant::now().duration_since(tick).as_secs_f32();
        }
    }
    fn update_player_pos(&mut self, input: Coordinate) {
        self.player_pos += input * self.player_speed * self.delta;
    }
}
impl Default for Game<CrosstermBackend<Stdout>> {
    fn default() -> Self {
        Self {
            input_handler: None,
            delta: 0.0,
            player: Box::new(Knight::new()),
            terminal: Terminal::new(CrosstermBackend::new(stdout())).unwrap(),
            player_pos: Coordinate { x: 0.0, y: 0.0 },
            player_speed: 60.0,
            fps: 30,
        }
    }
}
impl<B: Backend> Subscriber<Coordinate> for Game<B> {
    fn subscribe(&mut self, rx: Receiver<Coordinate>) {
        self.input_handler = Some(rx)
    }
}
#[cfg(test)]
mod tests {
    use std::io::stdout;

    use ratatui::{prelude::CrosstermBackend, Terminal};

    use crate::{layout::Coordinate, sprite::character::knight::Knight, Game};


    #[test]
    fn test_update_player_pos() {
        let mut game = Game {
            input_handler: None,
            delta: 0.5,
            player: Box::new(Knight::new()),
            terminal: Terminal::new(CrosstermBackend::new(stdout())).unwrap(),
            player_pos: Coordinate { x: 0.0, y: 0.0 },
            player_speed: 10.0,
            fps: 60,
        };

        let input = Coordinate { x: 1.0, y: 0.0 };
        game.update_player_pos(input);

        // Expected: 1 * 10 * 0.5 = 5.0
        assert_eq!(game.player_pos, Coordinate { x: 5.0, y: 0.0 });
    }
}
