use std::any::TypeId;

use crate::{
    prelude::*,
    utils::{Brush, String16, Thickness, Value},
};

use dces::prelude::{Component, Entity, EntityComponentManager};

/// The `WidgetContainer` wraps the entity of a widget and provides access to its properties, its children properties and its parent properties.
pub struct WidgetContainer<'a> {
    ecm: &'a mut EntityComponentManager<Tree, StringComponentStore>,
    current_node: Entity,
    theme: &'a Theme,
}

impl<'a> WidgetContainer<'a> {
    /// Creates a new widget container for the given `entity`.
    pub fn new(
        root: Entity,
        ecm: &'a mut EntityComponentManager<Tree, StringComponentStore>,
        theme: &'a Theme,
    ) -> Self {
        WidgetContainer {
            ecm,
            current_node: root,
            theme,
        }
    }

    /// Gets the entity of the widget.
    pub fn entity(&self) -> Entity {
        self.current_node
    }

    /// Gets the property.
    ///
    /// # Panics
    ///
    /// Panics if the widget does not contains the property.
    pub fn get<P>(&self, key: &str) -> &P
    where
        P: Clone + Component,
    {
        if let Ok(property) = self.ecm.component_store().get::<P>(key, self.current_node) {
            return property;
        }

        let name = self
            .ecm
            .component_store()
            .get::<String>("name", self.current_node)
            .unwrap()
            .clone();

        panic!(
        "Widget with name: {} and entity: {} does not contain property width type_id {:?} for key: {}",
        name,
        self.current_node.0,
        TypeId::of::<P>(),
        key
    );
    }

    /// Gets a mutable reference of the property of type `P`.
    ///
    /// # Panics
    ///
    /// Panics if the widget does not contains the property.
    pub fn get_mut<P>(&mut self, key: &str) -> &mut P
    where
        P: Clone + Component,
    {
        if let Ok(property) = self
            .ecm
            .component_store_mut()
            .get_mut::<P>(key, self.current_node)
        {
            return property;
        }

        panic!(
            "Entity {} does not contain property type {:?}, with key: {}",
            self.current_node.0,
            TypeId::of::<P>(),
            key
        );
    }

    /// Clones the property. If the property does not exists for the widget the
    /// default of the property value will be returned,
    pub fn clone_or_default<P>(&self, key: &str) -> P
    where
        P: Clone + Component + Default,
    {
        if let Ok(property) = self.ecm.component_store().get::<P>(key, self.current_node) {
            return property.clone();
        }

        P::default()
    }

    /// Clones the property.
    ///
    /// # Panics
    ///
    /// Panics if the widget does not contains the property.
    pub fn clone<P>(&self, key: &str) -> P
    where
        P: Clone + Component,
    {
        if let Ok(property) = self.ecm.component_store().get::<P>(key, self.current_node) {
            return property.clone();
        }

        let name = self
            .ecm
            .component_store()
            .get::<String>("name", self.current_node)
            .unwrap()
            .clone();

        panic!(
        "Widget with name: {} and entity: {} does not contain property width type_id {:?} for key: {}",
        name,
        self.current_node.0,
        TypeId::of::<P>(),
        key
    );
    }

    /// Clones the property of type `P` from the given widget entity. If the entity does
    /// not exists or it doesn't have a component of type `P` `None` will be returned.
    pub fn try_clone<P>(&self, key: &str) -> Option<P>
    where
        P: Clone + Component,
    {
        if let Ok(property) = self.ecm.component_store().get::<P>(key, self.current_node) {
            return Some(property.clone());
        }

        None
    }

    /// Sets the property of type `P`.
    ///
    /// # Panics
    ///
    /// Panics if the widget does not contains the property.
    pub fn set<P>(&mut self, key: &str, value: P)
    where
        P: Component + Clone,
    {
        if let Ok(property) = self
            .ecm
            .component_store_mut()
            .get_mut::<P>(key, self.current_node)
        {
            *property = value;
            return;
        }

        let name = self
            .ecm
            .component_store()
            .get::<String>("name", self.current_node)
            .unwrap()
            .clone();

        panic!(
            "Widget with name: {} and entity: {} does not contain property width type_id {:?} for key: {}",
            name,
            self.current_node.0,
            TypeId::of::<P>(),
            key
        );
    }

    /// Returns `true` if the widget has a property of type `P` otherwise `false`.
    pub fn has<P>(&self, key: &str) -> bool
    where
        P: Clone + Component,
    {
        self.ecm
            .component_store()
            .get::<P>(key, self.current_node)
            .is_ok()
    }

    /// Returns a reference of a property of type `P` from the given widget entity. If the entity does
    /// not exists or it doesn't have a component of type `P` `None` will be returned.
    pub fn try_get<P: Component>(&self, key: &str) -> Option<&P> {
        self.ecm
            .component_store()
            .get::<P>(key, self.current_node)
            .ok()
    }

    /// Returns a mutable reference of a property of type `P` from the given widget entity. If the entity does
    /// not exists or it doesn't have a component of type `P` `None` will be returned.
    pub fn try_get_mut<P: Component>(&mut self, key: &str) -> Option<&mut P> {
        self.ecm
            .component_store_mut()
            .get_mut::<P>(key, self.current_node)
            .ok()
    }

    /// Checks if the given value is equal to the given property.
    pub fn eq<P: Component + PartialEq>(&self, key: &str, other: &P) -> bool {
        if let Some(value) = self.try_get::<P>(key) {
            return value.eq(other);
        }

        false
    }

    fn update_internal_theme_by_state(&mut self, force: bool, entity: &Entity) {
        let internal_force = if let Ok(selector) = self
            .ecm
            .component_store()
            .get::<Selector>("selector", *entity)
        {
            if selector.dirty() {
                true
            } else {
                false
            }
        } else {
            false
        };

        for child in &(self.ecm.entity_store().children.clone())[&entity] {
            self.update_internal_theme_by_state(internal_force || force, child);
        }

        self.current_node = *entity;

        self.update_properties_by_theme(force);
    }

    /// Updates the theme by the inner state e.g. `selected` or `pressed`.
    pub fn update_theme_by_state(&mut self, force: bool) {
        self.update_internal_theme_by_state(force, &(self.current_node.clone()));
    }

    fn update_constraint(&mut self, key: &str, value: Value) {
        let value = if let Ok(value) = value.0.into_rust::<f64>() {
            value
        } else {
            0.0
        };

        if let Some(mut constraint) = self.try_clone::<Constraint>("constraint") {
            match key {
                "width" => constraint.set_width(value),
                "height" => constraint.set_height(value),
                "min_width" => constraint.set_min_width(value),
                "min_height" => constraint.set_min_height(value),
                "max_width" => constraint.set_max_width(value),
                "max_height" => constraint.set_max_height(value),
                _ => {}
            }
        }
    }

    fn update_value<T, V>(&mut self, key: &str, value: V)
    where
        T: Component + Clone,
        V: Into<T>,
    {
        if self.has::<T>(key) {
            self.set::<T>(key, value.into());
        }
    }

    /// Update all properties for the theme.
    pub fn update_properties_by_theme(&mut self, force: bool) {
        if !self.has::<Selector>("selector") {
            return;
        }

        let selector = self.clone::<Selector>("selector");

        if !selector.dirty() && !force {
            return;
        }

        if let Some(props) = self.theme.properties(&selector) {
            for (key, value) in props {
                match key.as_str() {
                    "foreground" | "background" | "icon_brush" | "border_brush" => {
                        self.update_value::<Brush, Value>(key, Value(value.clone()));
                    }
                    "font_size" | "icon_size" | "spacing" | "border_radius" => {
                        self.update_value::<f64, Value>(key, Value(value.clone()));
                    }
                    "padding" | "border_width" => {
                        self.update_value::<Thickness, Value>(key, Value(value.clone()));
                    }
                    "font_family" | "icon_family" => {
                        self.update_value::<String, Value>(key, Value(value.clone()));
                    }
                    "opacity" => {
                        self.update_value::<f32, Value>(key, Value(value.clone()));
                    }
                    "width" | "height" | "min_width" | "min_height" | "max_width"
                    | "max_height" => self.update_constraint(key, Value(value.clone())),
                    _ => {}
                }
            }
        }

        self.get_mut::<Selector>("selector").set_dirty(true);
    }
}
