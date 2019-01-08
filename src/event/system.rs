use crate::Event;

pub enum SystemEvent {
    Quit,
}

impl Event for SystemEvent {}
