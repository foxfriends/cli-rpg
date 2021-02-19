use super::{Event, Events, Process};
use pancurses::Input;

pub(super) trait MenuOptions {
    /// The number of options on this menu. Recommended to override this for performance reasons
    /// when possible.
    fn len(&self) -> usize {
        self.options().len()
    }

    /// The labels for the options to display in this menu.
    fn options(&self) -> Vec<String>;

    /// Select a menu option, performing the update.
    fn select(&self, option: usize) -> Box<dyn Event>;

    /// Cancel out of this menu, if possible. This can be left unimplemented if the menu does not
    /// have such a cancellation behaviour.
    fn cancel(&self) {}
}

#[derive(Default)]
pub(super) struct Menu<T> {
    current_option: usize,
    options: T,
}

impl<T> Menu<T>
where
    T: MenuOptions,
{
    fn prev_option(&mut self) {
        self.current_option = match self.current_option {
            0 => self.options.len() - 1,
            i => i - 1,
        };
    }

    fn next_option(&mut self) {
        self.current_option = (self.current_option + 1) % self.options.len();
    }
}

impl<T> Process for Menu<T>
where
    T: MenuOptions,
{
    #[rustfmt::skip]
    fn process(&mut self, input: Input) -> Events {
        use Input::*;
        let mut events = vec![];
        match input {
            KeyDown | Character('s') | Character('j') => self.next_option(),
            KeyUp | Character('w') | Character('k') => self.prev_option(),
            KeyRight | Character('d') | Character('l') | Character('\n') => events.push(self.options.select(self.current_option)),
            KeyLeft | Character('a') | Character('h') | Character('\x1B') => self.options.cancel(),
            _ => {}
        }
        events
    }
}
