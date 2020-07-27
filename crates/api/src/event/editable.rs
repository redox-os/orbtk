use super::{Event, EventBox, EventHandler};
use crate::prelude::*;

use std::rc::Rc;

crate::trigger_event!(
    ChangedEvent,
    ChangedEventHandler,
    ChangedHandler,
    on_changed
);
crate::trigger_event!(
    ActivateEvent,
    ActivateEventHandler,
    ActivateHandler,
    on_activate
);

#[derive(Clone, Event)]
pub struct SelectionChangedEvent(pub Entity, pub Vec<usize>);

pub type WindowHandlerFn = dyn Fn(&mut StatesContext, Entity, Vec<usize>) + 'static;

#[derive(IntoHandler)]
pub struct SelectionChangedEventHandler {
    pub handler: Rc<WindowHandlerFn>,
}

impl EventHandler for SelectionChangedEventHandler {
    fn handle_event(&self, states: &mut StatesContext, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<SelectionChangedEvent>() {
            (self.handler)(states, event.0, event.1.clone());
            return true;
        }

        false
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<SelectionChangedEvent>()
    }
}

pub trait SelectionChangedHandler: Sized + Widget {
    /// Inserts a click handler.
    fn on_selection_changed<H: Fn(&mut StatesContext, Entity, Vec<usize>) + 'static>(
        self,
        handler: H,
    ) -> Self {
        self.insert_handler(SelectionChangedEventHandler {
            handler: Rc::new(handler),
        })
    }
}
