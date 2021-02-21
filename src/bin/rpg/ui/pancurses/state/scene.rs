use super::{Event, Events, MainMenu, Menu, Process};
use pancurses::Input;

event!(pub(super) Goto(Scene));

#[derive(Clone)]
pub(super) enum Scene {
    MainMenu(Menu<MainMenu>),
    Game,
}

impl Default for Scene {
    fn default() -> Self {
        Self::MainMenu(Menu::default())
    }
}

impl Process for Scene {
    fn process(&mut self, input: Input) -> Events {
        use Scene::*;
        match self {
            MainMenu(menu) => menu.process(input),
            Game => vec![],
        }
    }
}
