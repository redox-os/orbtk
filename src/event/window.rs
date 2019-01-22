use crate::Event;

pub enum WindowEvent {
    Resize { width: f64, height: f64 },
}

impl Event for WindowEvent {}
