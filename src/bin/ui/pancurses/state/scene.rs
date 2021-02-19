use super::{Events, MainMenu, Menu, Process};
use pancurses::Input;

pub(super) enum Scene {
    MainMenu(Menu<MainMenu>),
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
        }
    }
}
