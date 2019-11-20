use super::{Event, EventBox, EventHandler};
use crate::prelude::*;

use std::rc::Rc;

pub struct ChangedEvent(pub Entity);

impl Event for ChangedEvent {}

pub type ChangedHandlerFn = dyn Fn(Entity) + 'static;

pub struct ChangedEventHandler(Rc<ChangedHandlerFn>);

impl EventHandler for ChangedEventHandler {
    fn handle_event(&self, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<ChangedEvent>() {
            (self.0)(event.0);
        }

        false
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<ChangedEvent>()
    }
}

impl From<ChangedEventHandler> for Rc<dyn EventHandler> {
    fn from(event: ChangedEventHandler) -> Self {
        Rc::new(event)
    }
}

pub trait ChangedHandler: Sized + Widget {
    fn on_changed<H: Fn(Entity) + 'static>(self, handler: H) -> Self {
        self.insert_handler(ChangedEventHandler(Rc::new(handler)))
    }
}
