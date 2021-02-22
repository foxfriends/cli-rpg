use super::*;

pub(super) trait MenuOptions: Clone {
    /// The number of options on this menu. Recommended to override this for performance reasons
    /// when possible.
    fn len(&self) -> usize {
        self.options().len()
    }

    /// A title to display above this menu.
    fn title(&self) -> Option<String> {
        None
    }

    /// The labels for the options to display in this menu.
    fn options(&self) -> Vec<String>;

    /// Select a menu option, performing the update.
    fn select(&self, option: usize) -> Box<dyn Event>;

    /// Cancel out of this menu, if possible. This can be left unimplemented if the menu does not
    /// have such a cancellation behaviour.
    fn cancel(&self) {}
}

#[derive(Clone, Default)]
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
        use InputKind::*;
        let mut events = vec![];
        match input.kind {
            Down | Character('s') | Character('j') => self.next_option(),
            Up | Character('w') | Character('k') => self.prev_option(),
            Right | Character('d') | Character('l') | Character('\n') => events.push(self.options.select(self.current_option)),
            Left | Character('a') | Character('h') | Esc => self.options.cancel(),
            _ => {}
        }
        events
    }
}

impl<T> Render for Menu<T>
where
    T: MenuOptions,
{
    fn render(&self, window: &Window) {
        let (h, w) = window.get_max_yx();
        let options = self.options.options();
        if options.is_empty() {
            return;
        }

        let y = (h - options.len() as i32) / 2;

        if let Some(title) = self.options.title() {
            let l = title.len();
            let x = (w - l as i32) / 2;
            window.mvaddstr(y - 3, x, title);
            window.mvaddstr(y - 2, x, "-".repeat(l));
        }

        for (i, mut option) in options.into_iter().enumerate() {
            if i == self.current_option {
                option = format!("-> {} <-", option);
            }
            let l = option.len() as i32;
            let x = (w - l) / 2;
            window.mvaddstr(y + i as i32, x, option);
        }
    }
}
