use crate::Event;

pub enum WindowEvent {
    Resize { width: u32, height: u32},
}

impl Event for WindowEvent {}
