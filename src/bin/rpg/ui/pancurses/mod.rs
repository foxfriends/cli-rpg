use crossbeam_channel::{Receiver, Sender, TryRecvError};
use pancurses::*;
use rpg::{EngineCommand, Render, UiCommand};
use std::cell::RefCell;
use std::time::{Duration, Instant};

#[macro_use]
mod event;
mod input;
mod layout;
mod process;
mod state;
mod window_renderer;

use event::{Event, EventHandler, Events};
use input::{Input, InputKind};
use layout::Layout;
use process::Process;
use state::UiState;
use window_renderer::WindowRenderer;

const FPS: u64 = 30;

pub struct Ui {
    window: Window,
    layout: Layout,
    ui_state: RefCell<UiState>,
    to_engine: Sender<EngineCommand>,
    from_engine: Receiver<UiCommand>,
}

impl Ui {
    pub fn new(to_engine: Sender<EngineCommand>, from_engine: Receiver<UiCommand>) -> Self {
        let window = initscr();
        window.nodelay(true);
        noecho();
        cbreak();
        curs_set(0);
        Self {
            layout: Layout::debug(&window),
            window,
            ui_state: RefCell::default(),
            to_engine,
            from_engine,
        }
    }

    pub fn run(self) {
        let frame_length = Duration::from_millis(1000 / FPS);
        loop {
            self.repaint();
            let next_frame = Instant::now() + frame_length;
            let mut debug_erased = false;
            loop {
                loop {
                    match self.from_engine.try_recv() {
                        Ok(command) => self.process_and_handle(command),
                        Err(TryRecvError::Empty) => break,
                        Err(TryRecvError::Disconnected) => return,
                    }
                }
                for input in Input::read(&self.window) {
                    if let Some(debug) = self.layout.get("debug") {
                        if !debug_erased {
                            debug.erase();
                            debug_erased = true;
                        }
                        debug.addstr(format!("{:?} ", input));
                    }
                    self.process_and_handle(input);
                }
                if Instant::now() <= next_frame {
                    break;
                }
            }
        }
    }

    fn process_and_handle<InputEvent>(&self, input: InputEvent)
    where
        UiState: Process<InputEvent>,
    {
        for event in self
            .ui_state
            .borrow_mut()
            .process_and_handle(input)
            .into_iter()
        {
            downcast!(event.as_any(), {
                event::Quit => self.to_engine.send(EngineCommand::Stop).unwrap(),
                event::Command(cmd) => self.to_engine.send(cmd.clone()).unwrap(),
                else => {}
            });
        }
    }

    fn repaint(&self) {
        if let Some(debug) = self.layout.get("debug") {
            debug.refresh();
            debug.mv(0, 0);
        }
        if let Some(main) = self.layout.get("main") {
            main.erase();
            self.ui_state.borrow().render(&mut WindowRenderer(main));
            main.refresh();
        }
    }
}

// TODO: Is this all we need to do to clean this up? What if there are many subwindows?
impl Drop for Ui {
    fn drop(&mut self) {
        endwin();
    }
}
