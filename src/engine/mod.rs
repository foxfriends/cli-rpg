use crate::UiCommand;
use crossbeam_channel::{Receiver, Sender};

mod command;

pub use command::EngineCommand;

pub struct Engine {
    from_ui: Receiver<EngineCommand>,
    to_ui: Sender<UiCommand>,
}

impl Engine {
    pub fn new(from_ui: Receiver<EngineCommand>, to_ui: Sender<UiCommand>) -> Self {
        Self { from_ui, to_ui }
    }

    pub fn run(self) {}
}
