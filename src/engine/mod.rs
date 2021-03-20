use crate::{GameState, UiCommand};
use crossbeam_channel::{Receiver, RecvTimeoutError, Sender};
use legion::{Resources, Schedule, World};
use std::cell::RefCell;
use std::time::{Duration, Instant};

mod command;

use super::systems::*;
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

        let mut handle_move_north = Schedule::builder()
            .add_system(acceleration_system(0, -1))
            .build();
        let mut handle_move_east = Schedule::builder()
            .add_system(acceleration_system(1, 0))
            .build();
        let mut handle_move_south = Schedule::builder()
            .add_system(acceleration_system(0, 1))
            .build();
        let mut handle_move_west = Schedule::builder()
            .add_system(acceleration_system(-1, 0))
            .build();
        let mut step = Schedule::builder().add_system(movement_system()).build();

        'outer: loop {
            let next_frame = Instant::now() + frame_length;
            loop {
                match self.from_ui.recv_deadline(next_frame) {
                    Ok(EngineCommand::Stop) => break 'outer,
                    Ok(EngineCommand::MoveNorth) => self.execute(&mut handle_move_north),
                    Ok(EngineCommand::MoveEast) => self.execute(&mut handle_move_east),
                    Ok(EngineCommand::MoveSouth) => self.execute(&mut handle_move_south),
                    Ok(EngineCommand::MoveWest) => self.execute(&mut handle_move_west),
                    Err(RecvTimeoutError::Timeout) => break,
                    Err(RecvTimeoutError::Disconnected) => break 'outer,
                }
            }
            self.execute(&mut step);
        }
    }

    fn execute(&mut self, schedule: &mut Schedule) {
        schedule.execute(&mut self.world, &mut self.resources);
    }
}
