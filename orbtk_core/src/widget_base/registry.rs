use std::{any::Any, collections::HashMap};

use dces::prelude::Component;

/// The struct `Registry` represents a global registry. It is used to
/// register and call global elements like services.
#[derive(Default)]
pub struct Registry {
    registry: HashMap<String, Box<dyn Any>>,
}

impl Registry {
    /// Creates a service registry with an empty Registry map.
    pub fn new() -> Self {
        Registry::default()
    }

    /// Register a new Registry with the given key.
    pub fn register<C: Component>(&mut self, key: impl Into<String>, service: C) {
        self.registry.insert(key.into(), Box::new(service));
    }

    /// Gets an element from the registry.
    ///
    /// # Panics
    ///
    /// Panics if the there is no element for the given key or the
    /// given service type is wrong.
    pub fn get<C: Component>(&self, key: &str) -> &C {
        self.registry
            .get(&key.to_string())
            .unwrap_or_else(|| panic!("Registry.get(): key: {} could not be found.", key))
            .downcast_ref()
            .unwrap_or_else(|| panic!("Registry.get(): wrong type for key: {}", key))
    }

    /// Gets a mutable reference of the requested element.
    ///
    /// # Panics
    ///
    /// Panics if the there is no service for the given key or the
    /// given service type is wrong.
    pub fn get_mut<C: Component>(&mut self, key: &str) -> &mut C {
        self.registry
            .get_mut(&key.to_string())
            .unwrap_or_else(|| panic!("Registry.get(): key: {} could not be found.", key))
            .downcast_mut()
            .unwrap_or_else(|| panic!("Registry.get(): wrong type for key: {}", key))
    }

    /// Try to get an element from the registry.
    pub fn try_get<C: Component>(&self, key: &str) -> Option<&C> {
        if let Some(e) = self.registry.get(&key.to_string()) {
            if let Some(r) = e.downcast_ref() {
                return Some(r);
            }
        }

        None
    }

    /// Try to get an element from the registry.
    pub fn try_get_mut<C: Component>(&mut self, key: &str) -> Option<&mut C> {
        if let Some(e) = self.registry.get_mut(&key.to_string()) {
            if let Some(r) = e.downcast_mut() {
                return Some(r);
            }
        }

        None
    }

    /// Returns the number of elements in the registry.
    pub fn len(&self) -> usize {
        self.registry.len()
    }

    /// Returns true if the registry contains no elements.
    pub fn is_empty(&self) -> bool {
        self.registry.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ServiceOne;
    struct ServiceTwo;

    #[test]
    fn register() {
        let mut registry = Registry::new();
        registry.register("one", ServiceOne);
        registry.register("two", ServiceTwo);

        assert!(registry.try_get::<ServiceOne>("one").is_some());
        assert!(registry.try_get::<ServiceTwo>("two").is_some());
    }

    #[test]
    fn try_get_mut() {
        let mut registry = Registry::new();
        registry.register("one", ServiceOne);
        registry.register("two", ServiceTwo);

        assert!(registry.try_get_mut::<ServiceOne>("one").is_some());
        assert!(registry.try_get_mut::<ServiceTwo>("two").is_some());
    }

    #[test]
    fn len() {
        let mut registry = Registry::new();
        assert_eq!(registry.len(), 0);

        registry.register("one", ServiceOne);
        assert_eq!(registry.len(), 1);

        registry.register("two", ServiceTwo);
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn is_empty() {
        let mut registry = Registry::new();
        assert!(registry.is_empty());

        registry.register("one", ServiceOne);
        assert!(!registry.is_empty());
    }
}
