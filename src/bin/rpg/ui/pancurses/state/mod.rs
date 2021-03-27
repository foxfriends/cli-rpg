use super::*;
use rpg::{Render, Renderer};

mod game;
mod main_menu;
mod menu;
mod scene;

use game::Game;
use main_menu::MainMenu;
use menu::{Menu, MenuOptions};
use scene::Scene;

pub struct UiState {
    scenes: Vec<Scene>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            scenes: vec![Scene::default()],
        }
    }
}

impl<InputEvent> Process<InputEvent> for UiState
where
    Scene: Process<InputEvent>,
{
    fn process(&mut self, input: InputEvent) -> Events {
        self.scenes.last_mut().unwrap().process(input)
    }
}

impl EventHandler for UiState {
    fn handle(&mut self, event: Box<dyn Event>) -> Events {
        downcast!(event.as_any(), {
            scene::Goto(scene) => {
                self.scenes = vec![scene.clone()];
            },
            else => { return vec![event] }
        });
        vec![]
    }
}

impl Render for UiState {
    fn render(&self, renderer: &mut dyn Renderer) {
        // TODO: is there a more efficient way to do this?
        // maybe reverse render without overwrite until flip?
        for scene in &self.scenes {
            scene.render(renderer);
        }
    }
}
