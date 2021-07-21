use std::rc::Rc;

use super::*;

use crate::{proc_macros::*, widget_base::*};

/// The enumeration of valid window events.
#[derive(Clone, Event)]
pub enum WindowEvent {
    Resize { width: f64, height: f64 },
    ActiveChanged(bool),
    None,
}

pub type WindowHandlerFn = dyn Fn(&mut StatesContext, WindowEvent) -> bool + 'static;

/// The structure handling windows events.
#[derive(IntoHandler)]
pub struct WindowEventHandler {
    /// A reference counted handler.
    pub handler: Rc<WindowHandlerFn>,
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
