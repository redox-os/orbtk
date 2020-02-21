use crate::{event::Event, proc_macros::Event};

#[derive(Event)]
pub enum SystemEvent {
    Quit,
}
