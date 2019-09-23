use crate::event::Event;

pub enum WindowEvent {
    Resize { width: f64, height: f64 },
    None,
}

impl Event for WindowEvent {}
