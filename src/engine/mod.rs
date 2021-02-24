use crate::{GameState, UiCommand};
use crossbeam_channel::{Receiver, Sender};
use legion::World;
use std::cell::RefCell;

mod command;

pub use command::EngineCommand;

pub struct Engine {
    game_state: RefCell<GameState>,
    world: World,
    from_ui: Receiver<EngineCommand>,
    to_ui: Sender<UiCommand>,
}

impl Engine {
    pub fn new(from_ui: Receiver<EngineCommand>, to_ui: Sender<UiCommand>) -> Self {
        Self {
            game_state: RefCell::default(),
            world: World::default(),
            from_ui,
            to_ui,
        }
    }

    pub fn run(self) {
        loop {
            match self.from_ui.recv() {
                Ok(EngineCommand::Stop) => break,
                Ok(command) => self.handle(command),
                Err(..) => break,
            }
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
