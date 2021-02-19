use super::{event, Event, Events, Process};
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use match_downcast::match_downcast;
use pancurses::*;
use rpg::{EngineCommand, GameState, UiCommand};
use std::cell::RefCell;
use std::time::{Duration, Instant};

mod state;

use state::UiState;

const FPS: u64 = 60;

pub struct Ui {
    window: Window,
    ui_state: RefCell<UiState>,
    game_state: RefCell<GameState>,
    to_engine: Sender<EngineCommand>,
    from_engine: Receiver<UiCommand>,
}

impl Ui {
    pub fn new(to_engine: Sender<EngineCommand>, from_engine: Receiver<UiCommand>) -> Self {
        let window = initscr();
        window.nodelay(true);
        noecho();
        nocbreak();
        curs_set(0);
        Self {
            window,
            ui_state: RefCell::default(),
            game_state: RefCell::default(),
            to_engine,
            from_engine,
        }
    }

    pub fn run(self) {
        let frame_length = Duration::from_millis(1000 / FPS);
        loop {
            self.repaint();
            let next_frame = Instant::now() + frame_length;
            loop {
                loop {
                    match self.from_engine.try_recv() {
                        Ok(command) => self.handle(command),
                        Err(TryRecvError::Empty) => break,
                        Err(TryRecvError::Disconnected) => return,
                    }
                }
                while let Some(input) = self.window.getch() {
                    self.process(input);
                }
                if Instant::now() <= next_frame {
                    break;
                }
            }
        }
    }

    fn handle(&self, command: UiCommand) {
        *self.game_state.borrow_mut() = command.state();
    }

    fn process(&self, input: Input) {
        for event in self.ui_state.borrow_mut().process(input).into_iter() {
            let any = event.to_any();
            match_downcast!(any, {
                x: event::Quit => self.to_engine.send(EngineCommand::Stop).unwrap(),
                _ => {}
            });
        }
    }

    fn repaint(&self) {}
}

// TODO: Is this all we need to do to clean this up? What if there are many subwindows?
impl Drop for Ui {
    fn drop(&mut self) {
        endwin();
    }
}
