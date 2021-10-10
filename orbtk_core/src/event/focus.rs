use dces::prelude::Entity;

use crate::{
    prelude::*,
    proc_macros::{Event, IntoHandler},
};

/// Used to request keyboard focus on the window.
#[derive(Event, Clone)]
pub enum FocusEvent {
    RequestFocus(Entity),
    RemoveFocus(Entity),
}

pub type FocusHandlerFn = dyn Fn(&mut StatesContext, FocusEvent) -> bool + 'static;

/// Structure for the focus handling of an event
#[derive(IntoHandler)]
pub struct FocusEventHandler {
    /// A reference counted handler
    pub handler: Rc<FocusHandlerFn>,
}

impl EventHandler for FocusEventHandler {
    fn handle_event(&self, states: &mut StatesContext, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<FocusEvent>() {
            return (self.handler)(states, event.clone());
        }

        false
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<FocusEvent>()
    }
}
