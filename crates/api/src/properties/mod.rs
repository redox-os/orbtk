//! This module contains non visual structures like point, rectangle, color and thickness.

use std::fmt::Debug;

use dces::prelude::{Component, StringComponentStore, Entity};

pub use self::layout::*;
pub use self::state::*;
pub use self::styling::*;
pub use self::widget::*;
use crate::{prelude::*, utils::String16};

mod layout;
mod state;
mod styling;
mod widget;

/// Used to the a property of a widget.
pub fn get_property<T>(key: &str, entity: Entity, store: &StringComponentStore) -> T
where
    T: Clone + Component,
{
    store
        .borrow_component::<T>(key, entity)
        .map(|r| r.clone())
        .unwrap()
}

/// Returns the value of a property of a widget if it exists otherwise the given value.
pub fn get_property_or_value<T>(key: &str, entity: Entity, store: &StringComponentStore, value: T) -> T
where
    T: Clone + Component,
{
    if let Ok(property) = store.borrow_component::<T>(key, entity).map(|r| r.clone()) {
        return property;
    }
    value
}

/// Use to build a property or to share it.
#[derive(PartialEq, Debug)]
pub enum PropertySource<P: Component + Debug> {
    Source(Entity),
    Value(P),
}

impl<P: Component + Debug> From<Entity> for PropertySource<P> {
    fn from(entity: Entity) -> Self {
        PropertySource::Source(entity)
    }
}

pub trait IntoPropertySource<P: Component + Debug> {
    fn into_source(self) -> PropertySource<P>;
}

// Implementation of PropertySource for default types
into_property_source!(bool);
into_property_source!(String);
into_property_source!(String16: &str, String);