//! This module contains non visual structures like point, rectangle, color and thickness.

use dces::prelude::{Component, Entity, EntityComponentManager};

pub use orbclient::color::Color;
pub use orbclient::Renderer as OrbRenderer;

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
pub enum PropertySource<P: Component> {
    Source(Entity),
    Value(P),
}