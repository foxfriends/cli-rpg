use crossbeam_channel::unbounded;
use rpg::Engine;
use std::thread;
use structopt::StructOpt;

#[macro_use]
mod downcast;

mod ui;

use ui::pancurses::Ui;

#[derive(Debug, StructOpt)]
enum Command {
    /// Starts the RPG engine in the background.
    Start,
    /// Stops the RPG engine.
    Stop,
    /// Check whether the RPG engine is running.
    Status,
    /// Open the UI
    Ui,
}

#[derive(Debug, StructOpt)]
struct Args {
    /// Optional subcommand. By default, the RPG engine will be started (if not already running),
    /// and then a full UI attached to that engine will be opened.
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[paw::main]
fn main(args: Args) {
    println!("{:?}", args);

    let (to_engine, from_ui) = unbounded();
    let (to_ui, from_engine) = unbounded();

    let engine_thread = thread::spawn(move || {
        let engine = Engine::new(from_ui, to_ui);
        engine.run();
    });

    let ui = Ui::new(to_engine, from_engine);
    ui.run();

    engine_thread.join().ok();
}
