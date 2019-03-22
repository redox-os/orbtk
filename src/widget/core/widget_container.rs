use std::any::TypeId;

use dces::prelude::{Component, Entity, EntityComponentManager, NotFound};

/// The `WidgetContainer` wraps the entity of a widget and provides access to its properties, its children properties and its parent properties.
pub struct WidgetContainer<'a> {
    ecm: &'a mut EntityComponentManager,
    current_node: Entity,
}

impl<'a> WidgetContainer<'a> {
    /// Creates a new widget container for the given `entity`.
    pub fn new(root: Entity, ecm: &'a mut EntityComponentManager) -> Self {
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
    pub fn get<P>(&self) -> P
        where
            P: Clone + Component + Default,
    {
        if !self.has::<P>() {
            panic!(
                "Entity {} does not contain property type {:?}",
                self.current_node,
                TypeId::of::<P>()
            );
        }

        self.ecm.borrow_component::<P>(self.current_node).unwrap().clone()
    }

    /// Gets the property. If the property does not exists for the widget the
    /// default of the property value will be returned,
    pub fn get_or_default<P>(&self) -> P
        where
            P: Clone + Component + Default,
    {
        if !self.has::<P>() {
            return P::default();
        }

        self.ecm.borrow_component::<P>(self.current_node).unwrap().clone()
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
        if !self.has::<P>() {
            panic!(
                "Entity {} does not contain property type {:?}",
                self.current_node,
                TypeId::of::<P>()
            );
        }

        self.ecm
            .borrow_mut_component::<P>(self.current_node)
            .unwrap()
    }

    /// Sets the property of type `P`.
    ///
    /// # Panics
    ///
    /// Panics if the widget does not contains the property.
    pub fn set<P>(&mut self, value: P)
        where P: Component + Default + Clone,

    {
        if !self.has::<P>() {
            panic!(
                "Entity {} does not contain property type {:?}",
                self.current_node,
                TypeId::of::<P>()
            );
        }

        *(self.get_mut::<P>()) = value;
    }

    /// Returns `true` if the widget has a property of type `P` otherwise `false`.
    pub fn has<P>(&self) -> bool
        where
            P: Clone + Component + Default,
    {
        if let Ok(_) = self.borrow::<P>() {
            return true;
        }

        false
    }

    /// Returns a reference of a property of type `P` from the given widget entity. If the entity does
    /// not exists or it doesn't have a component of type `P` `NotFound` will be returned.
    pub fn borrow<P: Component + Default>(&self) -> Result<&P, NotFound> {
        self.ecm.borrow_component::<P>(self.current_node)
    }

    /// Returns a mutable reference of a property of type `P` from the given widget entity. If the entity does
    /// not exists or it doesn't have a component of type `P` `NotFound` will be returned.
    pub fn borrow_mut<P: Component + Default>(&mut self) -> Result<&mut P, NotFound> {
        self.ecm.borrow_mut_component::<P>(self.current_node)
    }
}
