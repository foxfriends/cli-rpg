use super::*;
use rpg::GameState;

#[derive(Clone, Default, Debug)]
pub struct Game(GameState);

impl Process<Input> for Game {
    #[rustfmt::skip]
    fn process(&mut self, input: Input) -> Events {
        use InputKind::*;

        let mut events: Events = vec![];
        match input.kind {
            // TODO: commands probably should not be random strings, can I make a factory?
            Down | Character('s') | Character('j') => events.push(Box::new(event::Command("move south"))),
            Up | Character('w') | Character('k') => events.push(Box::new(event::Command("move north"))),
            Right | Character('d') | Character('l') => events.push(Box::new(event::Command("move east"))),
            Left | Character('a') | Character('h') => events.push(Box::new(event::Command("move west"))),
            _ => {}
        }
        events
    }
}

impl Process<UiCommand> for Game {
    fn process(&mut self, input: UiCommand) -> Events {
        self.0 = input.state();
        vec![]
    }
}