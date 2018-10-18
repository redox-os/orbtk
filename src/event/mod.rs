use std::any::{Any, TypeId};

use {Entity, EntityComponentManager, Tree};

pub use self::event_queue::*;
pub use self::mouse::*;
pub use self::system::*;

mod event_queue;
mod mouse;
mod system;

pub trait EventHandler {
    fn emit(&self) -> bool;

    fn update(&self, entity: Entity, tree: &Tree, ecm: &mut EntityComponentManager);

    fn event_type(&self) -> TypeId;

    fn check_condition(
        &self,
        event: &EventBox,
        entity: Entity,
        ecm: &mut EntityComponentManager,
    ) -> bool;
}

#[derive(PartialEq)]
pub enum EventStrategy {
    TopDown,
    BottomUp,
    Direct
}

pub trait Event : Any {
    fn strategy(&self) -> EventStrategy {
        EventStrategy::TopDown
    }
}