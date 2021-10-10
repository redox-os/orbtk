use crate::{event::Event, proc_macros::Event};

/// An enumeration of valid system events
#[derive(Event)]
pub enum SystemEvent {
    Quit,
}
