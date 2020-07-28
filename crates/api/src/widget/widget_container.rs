use std::any::type_name;

use crate::{
    event::ChangedEvent,
    prelude::*,
    utils::{Brush, Thickness, Value},
};

use dces::prelude::{Component, Entity, EntityComponentManager};

/// Mark the widget and shared widgets as dirty.
pub fn mark_as_dirty(
    key: &str,
    entity: Entity,
    ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
) {
    let root = ecm.entity_store().root();

    for entity in ecm.component_store().entities_of_component(key, entity) {
        *ecm.component_store_mut()
            .get_mut::<bool>("dirty", entity)
            .unwrap() = true;

        if let Ok(dirty_widgets) = ecm
            .component_store_mut()
            .get_mut::<Vec<Entity>>("dirty_widgets", root)
        {
            // don't add the same widget twice in a row
            if dirty_widgets.is_empty() || *dirty_widgets.last().unwrap() != entity {
                dirty_widgets.push(entity);
            }
        }
    }
}

/// The `WidgetContainer` wraps the entity of a widget and provides access to its properties, its children properties and its parent properties.
pub struct WidgetContainer<'a> {
    ecm: &'a mut EntityComponentManager<Tree, StringComponentStore>,
    current_node: Entity,
    theme: &'a Theme,
    event_queue: Option<&'a Rc<RefCell<EventQueue>>>,
}

impl<'a> WidgetContainer<'a> {
    /// Creates a new widget container for the given `entity`.
    pub fn new(
        root: Entity,
        ecm: &'a mut EntityComponentManager<Tree, StringComponentStore>,
        theme: &'a Theme,
        event_queue: Option<&'a Rc<RefCell<EventQueue>>>,
    ) -> Self {
        WidgetContainer {
            ecm,
            current_node: root,
            theme,
            event_queue,
        }
    }

    fn mark_as_dirty(&mut self, key: &str) {
        mark_as_dirty(key, self.current_node, self.ecm);
    }

    /// Gets the entity of the widget.
    pub fn entity(&self) -> Entity {
        self.current_node
    }

    /// Remove the dirty flag from the current widget.
    pub fn clear_dirty(&mut self) {
        let root = self.ecm.entity_store().root();
        *self
            .ecm
            .component_store_mut()
            .get_mut::<bool>("dirty", self.current_node)
            .unwrap() = false;

        let index = self
            .ecm
            .component_store()
            .get::<Vec<Entity>>("dirty_widgets", root)
            .unwrap()
            .iter()
            .position(|&r| r == self.current_node)
            .unwrap();

        if let Ok(dirty_widgets) = self
            .ecm
            .component_store_mut()
            .get_mut::<Vec<Entity>>("dirty_widgets", root)
        {
            dirty_widgets.remove(index);
        }
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

        let name = self.get_name();

        panic!(
            "Widget: {} with entity: {} does not contain property with type {:?} for key: {}",
            name,
            self.current_node.0,
            type_name::<P>(),
            key
        );
    }

    /// Gets a mutable reference of the property of type `P`.
    ///
    /// # Panics
    ///
    /// Panics if the widget does not contain the property.
    pub fn get_mut<P>(&mut self, key: &str) -> &mut P
    where
        P: Clone + Component,
    {
        self.mark_as_dirty(key);

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
            type_name::<P>(),
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

        let name = self.get_name();

        panic!(
            "Widget: {} with entity: {} does not contain property with type {:?} for key: {}",
            name,
            self.current_node.0,
            type_name::<P>(),
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

    /// Sets the property of type `P`. Sets the `dirty` flag of the widget to `true`.
    ///
    /// # Panics
    ///
    /// Panics if the widget does not contains the property.
    pub fn set<P>(&mut self, key: &str, value: P)
    where
        P: Component + Clone + PartialEq,
    {
        if self
            .ecm
            .component_store()
            .get::<P>(key, self.current_node)
            .unwrap()
            == &value
        {
            return;
        }
        self.mark_as_dirty(key);

        if let Some(event_queue) = self.event_queue {
            event_queue.borrow_mut().register_event_with_strategy(
                ChangedEvent(self.current_node, String::from(key)),
                EventStrategy::Direct,
                self.current_node,
            );
        }

        self.set_non_dirty(key, value);
    }

    /// Sets the property of type `P` without setting the widget dirty.
    ///
    /// # Panics
    ///
    /// Panics if the widget does not contains the property.
    pub fn set_non_dirty<P>(&mut self, key: &str, value: P)
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

        let name = self.get_name();

        panic!(
            "Widget: {} with entity: {} does not contain property with type {:?} for key: {}",
            name,
            self.current_node.0,
            type_name::<P>(),
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
        mark_as_dirty(key, self.current_node, self.ecm);
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

    fn update_constraint(&mut self, key: &str, value: Value) {
        let value = if let Ok(value) = value.0.into_rust::<f64>() {
            value
        } else {
            0.0
        };

        if let Ok(constraint) = self
            .ecm
            .component_store_mut()
            .get_mut::<Constraint>("constraint", self.current_node)
        {
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

    fn update_padding(&mut self, key: &str, value: Value) {
        let value = if let Ok(value) = value.0.into_rust::<f64>() {
            value
        } else {
            0.0
        };

        if let Some(padding) = self.try_get_mut::<Thickness>("padding") {
            match key {
                "padding_left" => padding.set_left(value),
                "padding_top" => padding.set_top(value),
                "padding_right" => padding.set_right(value),
                "padding_bottom" => padding.set_bottom(value),
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
            *self
                .ecm
                .component_store_mut()
                .get_mut::<T>(key, self.current_node)
                .unwrap() = value.into();
        }
    }

    /// Update all properties from theme for the current widget.
    pub fn update(&mut self, force: bool) {
        self.update_widget(self.current_node, force, false);
    }

    /// Update all properties from theme for the current widget and mark the widget as dirty.
    pub fn update_dirty(&mut self, force: bool) {
        self.update_widget(self.current_node, force, true);
    }

    /// Update all properties from theme for the given widget.
    pub fn update_widget(&mut self, entity: Entity, force: bool, should_mark_as_dirty: bool) {
        self.current_node = entity;
        if !self.has::<Selector>("selector") {
            return;
        }

        if force {
            // direct access to prevent initial setting of dirty flag on widget
            self.ecm
                .component_store_mut()
                .get_mut::<Selector>("selector", self.current_node)
                .unwrap()
                .set_dirty(true);
        }

        let selector = self.clone::<Selector>("selector");

        if !selector.dirty() {
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
                    "padding_left" | "padding_top" | "padding_right" | "padding_bottom" => {
                        self.update_padding(key, Value(value.clone()));
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

        let force = selector.dirty() || force;

        for child in &(self.ecm.entity_store().children.clone())[&entity] {
            self.update_widget(*child, force, should_mark_as_dirty);
        }

        self.current_node = entity;

        // direct access to prevent initial setting of dirty flag on widget
        self.ecm
            .component_store_mut()
            .get_mut::<Selector>("selector", self.current_node)
            .unwrap()
            .set_dirty(false);

        if should_mark_as_dirty {
            mark_as_dirty("selector", self.current_node, self.ecm);
        }
    }

    fn get_name(&self) -> String {
        if self.has::<String>("name") {
            self.ecm
                .component_store()
                .get::<String>("name", self.current_node)
                .unwrap()
                .clone()
        } else {
            String::from("unknown")
        }
    }
}
