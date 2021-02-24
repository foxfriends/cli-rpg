use crate::{GameState, UiCommand};
use crossbeam_channel::{Receiver, RecvTimeoutError, Sender};
use legion::{Resources, Schedule, World};
use std::cell::RefCell;
use std::time::{Duration, Instant};

mod command;

pub use command::EngineCommand;

const FPS: u64 = 30;

pub struct Engine {
    game_state: RefCell<GameState>,
    world: World,
    resources: Resources,
    from_ui: Receiver<EngineCommand>,
    to_ui: Sender<UiCommand>,
}

impl Engine {
    pub fn new(from_ui: Receiver<EngineCommand>, to_ui: Sender<UiCommand>) -> Self {
        Self {
            game_state: RefCell::default(),
            world: World::default(),
            resources: Resources::default(),
            from_ui,
            to_ui,
        }
    }

    pub fn run(mut self) {
        let frame_length = Duration::from_millis(1000 / FPS);

        let mut schedule = Schedule::builder().build();

        'outer: loop {
            let next_frame = Instant::now() + frame_length;
            loop {
                match self.from_ui.recv_deadline(next_frame) {
                    Ok(EngineCommand::Stop) => break 'outer,
                    Ok(command) => self.handle(command),
                    Err(RecvTimeoutError::Timeout) => break,
                    Err(RecvTimeoutError::Disconnected) => break 'outer,
                }
            }
            schedule.execute(&mut self.world, &mut self.resources);
        }
    }

    fn handle(&self, command: EngineCommand) {
        use EngineCommand::*;
        match command {
            Stop => unreachable!("should have been handled already"),
            Control(_control) => todo!("implement interactive user input interface"),
            Command(_command) => todo!("implement textual command based interface"),
        }
    }
}
