use crate::{GameState, UiCommand};
use crossbeam_channel::{Receiver, Sender};
use std::cell::RefCell;

mod command;

pub use command::EngineCommand;

pub struct Engine {
    game_state: RefCell<GameState>,
    from_ui: Receiver<EngineCommand>,
    to_ui: Sender<UiCommand>,
}

impl Engine {
    pub fn new(from_ui: Receiver<EngineCommand>, to_ui: Sender<UiCommand>) -> Self {
        Self {
            game_state: RefCell::default(),
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

    fn handle(&self, _command: EngineCommand) {}
}
