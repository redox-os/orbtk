use std::any::TypeId;

use {Entity, EntityComponentManager};

pub use self::event_queue::*;
pub use self::mouse::*;
pub use self::system::*;

mod event_queue;
mod mouse;
mod system;

pub trait EventHandler {
    fn emit(&self) -> bool;

    fn event_type(&self) -> TypeId;

    fn check_condition(&self, event: &EventBox, entity: Entity, ecm: &mut EntityComponentManager) -> bool;
}

