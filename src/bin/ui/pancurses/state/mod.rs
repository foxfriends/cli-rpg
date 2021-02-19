use super::{Event, Events, Process};
use pancurses::Input;

mod main_menu;
mod menu;
mod scene;

use main_menu::MainMenu;
use menu::Menu;
use scene::Scene;

#[derive(Default)]
pub struct UiState {
    scene: Scene,
}

impl Process for UiState {
    fn process(&mut self, input: Input) -> Events {
        self.scene.process(input)
    }
}
