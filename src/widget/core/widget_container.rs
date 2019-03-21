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

    /// Returns a reference of a property of type `P` from the given widget entity. If the entity does
    /// not exists or it doesn't have a component of type `P` `NotFound` will be returned.
    pub fn borrow_property<P: Component + Default>(&self) -> Result<&P, NotFound> {
        self.ecm.borrow_component::<P>(self.current_node)
    }

    /// Returns a mutable reference of a property of type `P` from the given widget entity. If the entity does
    /// not exists or it doesn't have a component of type `P` `NotFound` will be returned.
    pub fn borrow_mut_property<P: Component + Default>(&mut self) -> Result<&mut P, NotFound> {
        self.ecm.borrow_mut_component::<P>(self.current_node)
    }

    /// Gets the property.
    pub fn property<P>(&self) -> P
        where
            P: Clone + Component + Default,
    {
        self.ecm
            .borrow_component::<P>(self.current_node)
            .map(|r| r.clone())
            .unwrap_or_default()
    }

    /// Sets the property.
    pub fn set_property<P>(&mut self, value: P)
        where
            P: Clone + Component + Default,
    {
        if let Ok(property) = self.ecm.borrow_mut_component::<P>(self.current_node) {
            *property = value;
            return;
        }

        self.ecm.register_component(self.current_node, value);
    }

    /// Returns `true` if the widget has a property of type `P` otherwise `false`.
    pub fn has_property<P>(&self) -> bool
        where
            P: Clone + Component + Default,
    {
        if let Ok(_) = self.borrow_property::<P>() {
            return true;
        }

        false
    }
}
