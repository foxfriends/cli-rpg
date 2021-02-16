use crossbeam_channel::{Receiver, Sender};
use pancurses::*;
use rpg::{EngineCommand, UiCommand};

pub struct Ui {
    window: Window,
    to_engine: Sender<EngineCommand>,
    from_engine: Receiver<UiCommand>,
}

impl Ui {
    pub fn new(to_engine: Sender<EngineCommand>, from_engine: Receiver<UiCommand>) -> Self {
        let window = initscr();
        noecho();
        curs_set(0);
        Self {
            window,
            to_engine,
            from_engine,
        }
    }

    pub fn run(self) {}
}

// TODO: Is this all we need to do to clean this up? What if there are many subwindows?
impl Drop for Ui {
    fn drop(&mut self) {
        endwin();
    }
}
