//! This module contains non visual structures like point, rectangle, color and thickness.

use std::fmt::Debug;

use dces::prelude::{Component, Entity, EntityComponentManager};
pub use orbgl_api::Color;

pub use self::layout::*;
pub use self::state::*;
pub use self::styling::*;
pub use self::widget::*;

mod layout;
mod state;
mod styling;
mod widget;

/// Used to the a property of a widget.
pub fn get_property<T>(entity: Entity, ecm: &EntityComponentManager) -> T
where
    T: Clone + Component + Default,
{
    ecm.borrow_component::<T>(entity)
        .map(|r| r.clone())
        .unwrap_or_default()
}

/// Use to build a property or to share it.
#[derive(PartialEq, Debug)]
pub enum PropertySource<P: Component + PartialEq + Debug> {
    Source(Entity),
    Value(P),
}

impl<P: Component + PartialEq + Debug> From<Entity> for PropertySource<P> {
    fn from(entity: Entity) -> Self {
        PropertySource::Source(entity)
    }
}
