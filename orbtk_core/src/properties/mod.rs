//! This sub module contains extra structs used as widget properties.

use std::{collections::HashSet, fmt::Debug, path::PathBuf};

use dces::prelude::{Component, ComponentStore, Entity};

pub use self::layout::*;
pub use self::widget::*;
use crate::{into_property_source, render, theming, utils};

mod layout;
mod widget;

/// Get the property of a widget.
pub fn get_property<T>(key: &str, entity: Entity, store: &ComponentStore) -> T
where
    T: Clone + Component,
{
    store.get::<T>(key, entity).map(|r| r.clone()).unwrap()
}

/// Handles the key of a given entity.
/// It returns its property, if it does exits. If not, it returns the value of the key.
pub fn get_property_or_value<T>(key: &str, entity: Entity, store: &ComponentStore, value: T) -> T
where
    T: Clone + Component,
{
    if let Ok(property) = store.get::<T>(key, entity).map(|r| r.clone()) {
        return property;
    }
    value
}

/// Use to build a property or to share it.
#[derive(PartialEq, Debug)]
pub enum PropertySource<P: Component + Debug> {
    Source(Entity),
    KeySource(String, Entity),
    Value(P),
}

impl<P: Component + Debug> From<Entity> for PropertySource<P> {
    fn from(entity: Entity) -> Self {
        PropertySource::Source(entity)
    }
}

/// Used to convert components / properties into a PropertySource object.
pub trait IntoPropertySource<P: Component + Debug> {
    fn into_source(self) -> PropertySource<P>;
}

/// Used ot generate attached properties.
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
    /// Create a new attached property.
    pub fn new(key: impl Into<String>, property_source: impl IntoPropertySource<P>) -> Self {
        AttachedProperty {
            key: key.into(),
            property_source: property_source.into_source(),
        }
    }
}

// Implementation of PropertySource for default types
into_property_source!(bool);
into_property_source!(char);
into_property_source!(String: &str, utils::Value);
into_property_source!(usize);
into_property_source!(u32);
into_property_source!(f32: utils::Value);
into_property_source!(f64: i32, f32, utils::Value);
into_property_source!(i32);
into_property_source!(i64);

into_property_source!(PathBuf);

// Implementation of PropertySource for utils types
into_property_source!(utils::Alignment: &str);
into_property_source!(utils::Brush: &str, utils::Color, utils::Value);
into_property_source!(utils::Orientation: &str);
into_property_source!(utils::Point: f64, i32, (i32, i32), (f64, f64));
into_property_source!(utils::Size: f64, i32, (i32, i32), (f64, f64));
into_property_source!(
    utils::Rectangle: (i32, i32, i32, i32),
    (f64, f64, f64, f64),
    (utils::Point, utils::Size)
);
into_property_source!(
    utils::Thickness: i32,
    f64,
    (i32, i32),
    (f64, f64),
    (i32, i32, i32, i32),
    (f64, f64, f64, f64),
    utils::Value
);
into_property_source!(utils::SelectionMode: &str);
into_property_source!(utils::Visibility: &str);
into_property_source!(Vec<String>);
into_property_source!(utils::Filter: &str, String, Vec<String>, Vec<&str>);

// Implementation of css types
into_property_source!(theming::Selector: &str, String);
into_property_source!(theming::Theme);

// Implementation of render property types
into_property_source!(render::Image: &str, String, (u32, u32, Vec<u32>));

// Implementation of custom property types
into_property_source!(Blocks: BlocksBuilder, &str, String);
into_property_source!(utils::Constraint: utils::ConstraintBuilder);
into_property_source!(DefaultRenderPipeline);
into_property_source!(ScrollViewerMode: (&str, &str));
into_property_source!(SelectedEntities: HashSet<Entity>);
into_property_source!(SelectedIndices: HashSet<usize>);
into_property_source!(TextSelection: (usize, usize));
into_property_source!(FocusState);
into_property_source!(KeyboardState);
