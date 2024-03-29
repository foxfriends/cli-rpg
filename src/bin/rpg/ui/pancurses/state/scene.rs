use super::*;
use rpg::{Render, Renderer};

event!(pub(super) Goto(Scene));

#[derive(Clone)]
pub(in crate::ui::pancurses) enum Scene {
    MainMenu(Menu<MainMenu>),
    Game(Game),
}

impl Default for Scene {
    fn default() -> Self {
        Self::MainMenu(Menu::default())
    }
}

impl Process<Input> for Scene {
    fn process(&mut self, input: Input) -> Events {
        use Scene::*;

        match self {
            MainMenu(menu) => menu.process(input),
            Game(game) => game.process(input),
        }
    }
}

impl Process<UiCommand> for Scene {
    fn process(&mut self, command: UiCommand) -> Events {
        use Scene::*;

        match self {
            Game(game) => game.process(command),
            _ => vec![],
        }
    }
}

impl Render for Scene {
    fn render(&self, renderer: &mut dyn Renderer) {
        use Scene::*;

        match self {
            MainMenu(menu) => menu.render(renderer),
            Game(game) => game.render(renderer),
        }
    }
}
