//! This module contains all resources to call and handle events.

use std::{any::Any, collections::BTreeMap, rc::Rc};

use dces::entity::Entity;

pub use crate::widget::StatesContext;

pub use self::editable::*;
pub use self::event_handler::*;
pub use self::event_queue::*;
pub use self::focus::*;
pub use self::key::*;
pub use self::mouse::*;
pub use self::system::*;
pub use self::window::*;

mod editable;
mod event_handler;
mod event_queue;
mod focus;
mod key;
mod mouse;
mod system;
mod window;

/// Defines the strategy of an event how it moves through the tree.
#[derive(Debug, Clone, PartialEq)]
pub enum EventStrategy {
    // /// From root to leaf.
    // TopDown,
    /// From leaf to root.
    BottomUp,

    /// Occurs direct.
    Direct,
}

/// Used to define an event.
pub trait Event: Any {
    fn strategy(&self) -> EventStrategy {
        EventStrategy::BottomUp
    }
}

pub type EventHandlerMap = BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>;

pub type TriggerHandler = dyn Fn(&mut StatesContext, Entity) + 'static;

#[macro_export]
macro_rules! trigger_event {
    ($event:ident, $event_handler:ident, $trait:ident, $method:tt) => {
        pub struct $event(pub Entity);

        impl Event for $event {}

        pub struct $event_handler(Rc<TriggerHandler>);

        impl EventHandler for $event_handler {
            fn handle_event(&self, states: &mut StatesContext, event: &EventBox) -> bool {
                if let Ok(event) = event.downcast_ref::<$event>() {
                    (self.0)(states, event.0);
                }

                false
            }

            fn handles_event(&self, event: &EventBox) -> bool {
                event.is_type::<$event>()
            }
        }

        impl From<$event_handler> for Rc<dyn EventHandler> {
            fn from(event: $event_handler) -> Self {
                Rc::new(event)
            }
        }

        pub trait $trait: Sized + Widget {
            fn $method<H: Fn(&mut StatesContext, Entity) + 'static>(self, handler: H) -> Self {
                self.insert_handler($event_handler(Rc::new(handler)))
            }
        }
    };
}
