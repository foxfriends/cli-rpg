use super::*;

#[derive(Copy, Clone, Default)]
pub struct MainMenu;

impl MenuOptions for MainMenu {
    fn title(&self) -> Option<String> {
        Some(String::from("Simple RPG (working title)"))
    }

    fn len(&self) -> usize {
        3
    }

    fn options(&self) -> Vec<String> {
        vec![
            String::from("New Game"),
            String::from("Continue"),
            String::from("Quit"),
        ]
    }

    fn select(&self, option: usize) -> Box<dyn Event> {
        match option {
            0 => Box::new(scene::Goto(Scene::Game)),
            1 => Box::new(scene::Goto(Scene::Game)),
            2 => Box::new(event::Quit),
            _ => unreachable!(),
        }
    }
}
