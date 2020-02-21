use std::rc::Rc;

use crate::prelude::*;

#[derive(Clone, Event)]
pub enum WindowEvent {
    Resize { width: f64, height: f64 },
    ActiveChanged(bool),
    None,
}

pub type WindowHandlerFn = dyn Fn(&mut StatesContext, WindowEvent) -> bool + 'static;

pub struct WindowEventHandler {
    pub handler: Rc<WindowHandlerFn>,
}

impl Into<Rc<dyn EventHandler>> for WindowEventHandler {
    fn into(self) -> Rc<dyn EventHandler> {
        Rc::new(self)
    }
}

impl EventHandler for WindowEventHandler {
    fn handle_event(&self, states: &mut StatesContext, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<WindowEvent>() {
            return (self.handler)(states, event.clone());
        }

        false
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<WindowEvent>()
    }
}
