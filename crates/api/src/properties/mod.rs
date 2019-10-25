//! This module contains non visual structures like point, rectangle, color and thickness.

use std::fmt::Debug;

use dces::prelude::{Component, Entity, StringComponentStore};

pub use self::layout::*;
pub use self::styling::*;
pub use self::widget::*;
use crate::{prelude::*, utils, utils::String16};

mod layout;
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
pub fn get_property_or_value<T>(
    key: &str,
    entity: Entity,
    store: &StringComponentStore,
    value: T,
) -> T
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

pub struct AttachedProperty<P>
where
    P: Component + Debug,
{
    pub key: String,
    pub property_source: PropertySource<P>,
}

impl<P> AttachedProperty<P>
where
    P: Component + Debug,
{
    pub fn new(key: impl Into<String>, property_source: impl IntoPropertySource<P>) -> Self {
        AttachedProperty {
            key: key.into(),
            property_source: property_source.into_source(),
        }
    }
}

// Implementation of PropertySource for default types
into_property_source!(bool);
into_property_source!(String: &str);
into_property_source!(String16: &str, String);
into_property_source!(usize);
into_property_source!(f64: i32);

// Implementation of PropertySource for utils types
into_property_source!(utils::Visibility: &str);
