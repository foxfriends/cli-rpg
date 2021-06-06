use super::*;
use rpg::{GameState, Render, Renderer};

#[derive(Clone, Default, Debug)]
pub struct Game(GameState);

impl From<GameState> for Game {
    fn from(state: GameState) -> Self {
        Self(state)
    }
}

impl Process<Input> for Game {
    #[rustfmt::skip]
    fn process(&mut self, input: Input) -> Events {
        use InputKind::*;

        let mut events: Events = vec![];
        match input.kind {
            Down | Character('s') | Character('j') => events.push(Box::new(event::Command(EngineCommand::MoveSouth))),
            Up | Character('w') | Character('k') => events.push(Box::new(event::Command(EngineCommand::MoveNorth))),
            Right | Character('d') | Character('l') => events.push(Box::new(event::Command(EngineCommand::MoveEast))),
            Left | Character('a') | Character('h') => events.push(Box::new(event::Command(EngineCommand::MoveWest))),
            _ => {}
        }
        events
    }
}

impl Process<UiCommand> for Game {
    fn process(&mut self, command: UiCommand) -> Events {
        match command {
            UiCommand::Update(state) => self.0 = state,
            _ => {}
        }
        vec![]
    }
}

impl Render for Game {
    fn render(&self, renderer: &mut dyn Renderer) {
        let state = &self.0;

        for entity in state.entities().values() {
            entity.render(renderer);
        }
    }
}
