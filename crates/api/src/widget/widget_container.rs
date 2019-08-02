use std::any::TypeId;

use crate::tree::Tree;
use dces::prelude::{Component, Entity, EntityComponentManager};

/// The `WidgetContainer` wraps the entity of a widget and provides access to its properties, its children properties and its parent properties.
pub struct WidgetContainer<'a> {
    ecm: &'a mut EntityComponentManager<Tree>,
    current_node: Entity,
}

impl<'a> WidgetContainer<'a> {
    /// Creates a new widget container for the given `entity`.
    pub fn new(root: Entity, ecm: &'a mut EntityComponentManager<Tree>) -> Self {
        WidgetContainer {
            ecm,
            current_node: root,
        }
    }

    /// Gets the property.
    ///
    /// # Panics
    ///
    /// Panics if the widget does not contains the property.
    pub fn get<P>(&self) -> &P
    where
        P: Clone + Component + Default,
    {
        if let Ok(property) = self
            .ecm
            .component_store()
            .borrow_component::<P>(self.current_node)
        {
            return property;
        }

        panic!(
            "Entity {} does not contain property type {:?}",
            self.current_node.0,
            TypeId::of::<P>()
        );
    }

    /// Gets a mutable reference of the property of type `P`.
    ///
    /// # Panics
    ///
    /// Panics if the widget does not contains the property.
    pub fn get_mut<P>(&mut self) -> &mut P
    where
        P: Clone + Component + Default,
    {
        if let Ok(property) = self
            .ecm
            .component_store_mut()
            .borrow_mut_component::<P>(self.current_node)
        {
            return property;
        }

        panic!(
            "Entity {} does not contain property type {:?}",
            self.current_node.0,
            TypeId::of::<P>()
        );
    }

    /// Clones the property. If the property does not exists for the widget the
    /// default of the property value will be returned,
    pub fn clone_or_default<P>(&self) -> P
    where
        P: Clone + Component + Default,
    {
        if let Ok(property) = self
            .ecm
            .component_store()
            .borrow_component::<P>(self.current_node)
        {
            return property.clone();
        }

        P::default()
    }

    /// Clones the property.
    ///
    /// # Panics
    ///
    /// Panics if the widget does not contains the property.
    pub fn clone<P>(&self) -> P
    where
        P: Clone + Component + Default,
    {
        if let Ok(property) = self
            .ecm
            .component_store()
            .borrow_component::<P>(self.current_node)
        {
            return property.clone();
        }

        panic!(
            "Entity {} does not contain property type {:?}",
            self.current_node.0,
            TypeId::of::<P>()
        );
    }

    /// Clones the property of type `P` from the given widget entity. If the entity does
    /// not exists or it doesn't have a component of type `P` `None` will be returned.
    pub fn try_clone<P>(&self) -> Option<P>
    where
        P: Clone + Component + Default,
    {
        if let Ok(property) = self
            .ecm
            .component_store()
            .borrow_component::<P>(self.current_node)
        {
            return Some(property.clone());
        }

        None
    }

    /// Sets the property of type `P`.
    ///
    /// # Panics
    ///
    /// Panics if the widget does not contains the property.
    pub fn set<P>(&mut self, value: P)
    where
        P: Component + Default + Clone,
    {
        if let Ok(property) = self
            .ecm
            .component_store_mut()
            .borrow_mut_component::<P>(self.current_node)
        {
            *property = value;
            return;
        }

        panic!(
            "Entity {} does not contain property type {:?}",
            self.current_node.0,
            TypeId::of::<P>()
        );
    }

    /// Returns `true` if the widget has a property of type `P` otherwise `false`.
    pub fn has<P>(&self) -> bool
    where
        P: Clone + Component + Default,
    {
        self.ecm
            .component_store()
            .borrow_component::<P>(self.current_node)
            .is_ok()
    }

    /// Returns a reference of a property of type `P` from the given widget entity. If the entity does
    /// not exists or it doesn't have a component of type `P` `None` will be returned.
    pub fn try_get<P: Component + Default>(&self) -> Option<&P> {
        self.ecm
            .component_store()
            .borrow_component::<P>(self.current_node)
            .ok()
    }

    /// Returns a mutable reference of a property of type `P` from the given widget entity. If the entity does
    /// not exists or it doesn't have a component of type `P` `None` will be returned.
    pub fn try_get_mut<P: Component + Default>(&mut self) -> Option<&mut P> {
        self.ecm
            .component_store_mut()
            .borrow_mut_component::<P>(self.current_node)
            .ok()
    }
}
