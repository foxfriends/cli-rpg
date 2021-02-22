use super::{Event, Input};

pub(super) trait Process {
    fn process(&mut self, input: Input) -> Vec<Box<dyn Event>>;
}
