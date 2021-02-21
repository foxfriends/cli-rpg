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
        let mut events = vec![];
        for event in self.scene.process(input).into_iter() {
            downcast!(event.as_any(), {
                scene::Goto(scene) => {
                    self.scene = scene.clone();
                },
                else => events.push(event)
            });
        }
        events
    }
}
