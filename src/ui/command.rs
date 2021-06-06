use crate::GameState;

pub enum UiCommand {
    Load(GameState),
    Update(GameState),
}
