use super::super::super::{event, Event};
use super::menu::MenuOptions;

#[derive(Default)]
pub struct MainMenu;

impl MenuOptions for MainMenu {
    fn len(&self) -> usize {
        1
    }

    fn options(&self) -> Vec<String> {
        vec![String::from("Quit")]
    }

    fn select(&self, option: usize) -> Box<dyn Event> {
        match option {
            0 => Box::new(event::Quit),
            _ => unreachable!(),
        }
    }
}
