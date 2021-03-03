use super::{EventHandler, Events};

pub(super) trait Process<InputEvent> {
    fn process(&mut self, _input: InputEvent) -> Events {
        vec![]
    }

    fn process_and_handle(&mut self, input: InputEvent) -> Events
    where
        Self: EventHandler,
    {
        self.process(input)
            .into_iter()
            .flat_map(|event| self.handle(event))
            .collect()
    }
}
