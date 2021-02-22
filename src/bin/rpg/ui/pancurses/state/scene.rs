use super::*;

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

impl Render for Scene {
    fn render(&self, window: &Window) {
        use Scene::*;
        match self {
            MainMenu(menu) => menu.render(window),
            _ => {} // TODO
        }
    }
}
