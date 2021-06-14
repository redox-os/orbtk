use std::{collections::HashMap, rc::Rc};

use dces::prelude::*;

use super::*;

use crate::{proc_macros::*, widget_base::*};

crate::trigger_event!(
    ActivateEvent,
    ActivateEventHandler,
    ActivateHandler,
    on_activate
);

/// Structure to handle events that occures, if selection is changed.
#[derive(Clone, Event)]
pub struct SelectionChangedEvent(pub Entity, pub Vec<usize>);

/// Window handler Function
pub type WindowHandlerFn = dyn Fn(&mut StatesContext, Entity, Vec<usize>) + 'static;

/// Structure for the handler, that is used if selection is changed.
#[derive(IntoHandler)]
pub struct SelectionChangedEventHandler {
    /// A reference counted handler.
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

/// Methods for the `SelectionChangedHandler` type.
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

/// Structure to handle event changes, if a property of a widget is updated.
#[derive(Clone, Event)]
pub struct ChangedEvent(pub Entity, pub String);

/// Used to define a property changed callback.
pub type ChangedHandlerFn = dyn Fn(&mut StatesContext, Entity) + 'static;

/// Structure that defines elements used to handle changed events via an event handler.
#[derive(IntoHandler, Default)]
pub struct ChangedEventHandler {
    /// A handler acting on a hashmap
    pub handlers: HashMap<String, Rc<ChangedHandlerFn>>,
}

impl EventHandler for ChangedEventHandler {
    fn handle_event(&self, states: &mut StatesContext, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<ChangedEvent>() {
            if let Some(handler) = self.handlers.get(&event.1) {
                handler(states, event.0);
            }

            return true;
        }

        false
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<ChangedEvent>()
            && self
                .handlers
                .iter()
                .any(|h| h.0 == &event.downcast_ref::<ChangedEvent>().unwrap().1)
    }
}

/// Methods for the `ChangedHandler` type.
pub trait ChangedHandler: Sized + Widget + 'static {
    /// Register a on property changed handler.
    fn on_changed<H: Fn(&mut StatesContext, Entity) + 'static>(
        self,
        key: &str,
        handler: H,
    ) -> Self {
        self.insert_changed_handler(key, Rc::new(handler))
    }
}
