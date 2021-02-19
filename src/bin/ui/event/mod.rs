use std::any::Any;

pub(super) trait Event: Any {
    fn to_any(self: Box<Self>) -> Box<dyn Any>;
}

pub(super) type Events = Vec<Box<dyn Event>>;

pub(super) struct Quit;
impl Event for Quit {
    fn to_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
