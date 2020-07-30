use std::rc::Rc;

use crate::{
    prelude::*,
    proc_macros::*,
    shell::{Key, KeyEvent},
};

use super::{EventBox, EventHandler};

#[derive(Event)]
pub struct KeyDownEvent {
    pub event: KeyEvent,
}

#[derive(Event)]
pub struct KeyUpEvent {
    pub event: KeyEvent,
}

pub type KeyHandler = dyn Fn(&mut StatesContext, KeyEvent) -> bool + 'static;

/// Used to handle key down events. Could be attached to a widget.
#[derive(IntoHandler)]
pub struct KeyDownEventHandler {
    handler: Rc<KeyHandler>,
}

impl EventHandler for KeyDownEventHandler {
    fn handle_event(&self, state_context: &mut StatesContext, event: &EventBox) -> bool {
        event
            .downcast_ref::<KeyDownEvent>()
            .ok()
            .map_or(false, |event| {
                (self.handler)(state_context, event.event.clone())
            })
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<KeyDownEvent>()
    }
}

pub trait KeyDownHandler: Sized + Widget {
    /// Inserts a handler.
    fn on_key_down<H: Fn(&mut StatesContext, KeyEvent) -> bool + 'static>(
        self,
        handler: H,
    ) -> Self {
        self.insert_handler(KeyDownEventHandler {
            handler: Rc::new(handler),
        })
    }

    /// Handles events triggered by a specific key.
    fn on_key_down_key<H: Fn() -> bool + 'static>(self, key: Key, handler: H) -> Self {
        self.on_key_down(
            move |_, event| {
                if event.key == key {
                    handler()
                } else {
                    false
                }
            },
        )
    }
}
