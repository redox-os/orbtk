//! This module contains all resources to call and handle events.

use std::{any::Any, cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::entity::Entity;

pub use self::event_handler::*;
pub use self::event_queue::*;
pub use self::key::*;
pub use self::mouse::*;
pub use self::system::*;
pub use self::window::*;

mod event_handler;
mod event_queue;
mod key;
mod mouse;
mod system;
mod window;

/// Defines the strategy a event moves through the tree.
#[derive(Debug, Clone, PartialEq)]
pub enum EventStrategy {
    /// From root to leaf.
    TopDown,

    /// From leaf to root.
    BottomUp,

    /// Occures direct.
    Direct,
}

/// Used to define an event.
pub trait Event: Any {
    fn strategy(&self) -> EventStrategy {
        EventStrategy::BottomUp
    }
}

pub type EventHandlerMap = Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>;
