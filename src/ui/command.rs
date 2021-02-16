use crate::GameState;

pub struct UiCommand(GameState);

impl UiCommand {
    pub fn state(self) -> GameState {
        self.0
    }
}
