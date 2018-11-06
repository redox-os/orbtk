use std::any::Any;

pub use self::event_handler::*;
pub use self::event_queue::*;
pub use self::key::*;
pub use self::mouse::*;
pub use self::system::*;

mod event_handler;
mod event_queue;
mod key;
mod mouse;
mod system;

#[derive(PartialEq)]
pub enum EventStrategy {
    TopDown,
    BottomUp,
    Direct
}

pub trait Event : Any {
    fn strategy(&self) -> EventStrategy {
        EventStrategy::BottomUp
    }
}

// todo focuse moved event!!!