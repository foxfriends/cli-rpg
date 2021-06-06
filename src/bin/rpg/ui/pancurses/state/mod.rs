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

impl Process<Input> for UiState {
    fn process(&mut self, input: Input) -> Events {
        self.scenes.last_mut().unwrap().process(input)
    }
}

impl Process<UiCommand> for UiState {
    fn process(&mut self, command: UiCommand) -> Events {
        let mut events: Events = vec![];
        match command {
            UiCommand::Load(state) => {
                events.push(Box::new(scene::Goto(Scene::Game(Game::from(state)))));
            }
            _ => {
                if let Some(scene) = self.scenes.first_mut() {
                    events.extend(scene.process(command));
                }
            }
        }
        events
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
