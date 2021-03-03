use std::any::Any;

pub(super) trait Event: Any {
    fn as_any(&self) -> &dyn Any;
}

pub(super) trait EventHandler {
    fn handle(&mut self, event: Box<dyn Event>) -> Events {
        vec![event]
    }
}

#[macro_export]
macro_rules! event {
    ($vis:vis $name:ident) => {
        $vis struct $name;
        impl Event for $name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };

    ($vis:vis $name:ident ( $($t:ty),+ ) ) => {
        $vis struct $name ( $( pub $t ),+ );
        impl Event for $name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };

    ($vis:vis $name:ident { $($field:ident : $t:ty),+ } ) => {
        $vis struct $name { $( pub $field: $t ),+ }
        impl Event for $name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}

pub(super) type Events = Vec<Box<dyn Event>>;

event!(pub(super) Quit);
event!(pub(super) Command(&'static str));
