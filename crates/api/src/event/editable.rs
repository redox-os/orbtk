use std::rc::Rc;

use dces::prelude::*;

use super::*;

use crate::{proc_macros::*, widget_base::*};

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

#[derive(Clone, Event)]
/// This event occurs when a property of a widget is updated.
pub struct ChangedEvent(pub Entity, pub String);

/// Used to define a property changed callback.
pub type ChangedHandlerFn = dyn Fn(&mut StatesContext, Entity, &str) + 'static;

#[derive(IntoHandler)]
pub struct ChangedEventHandler {
    pub handler: Rc<ChangedHandlerFn>,
}

impl EventHandler for ChangedEventHandler {
    fn handle_event(&self, states: &mut StatesContext, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<ChangedEvent>() {
            (self.handler)(states, event.0, event.1.as_str());
            return true;
        }

        false
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<ChangedEvent>()
    }
}

pub trait ChangedHandler: Sized + Widget {
    /// Register a on property changed handler.
    fn on_changed<H: Fn(&mut StatesContext, Entity, &str) + 'static>(self, handler: H) -> Self {
        self.insert_handler(ChangedEventHandler {
            handler: Rc::new(handler),
        })
    }
}
