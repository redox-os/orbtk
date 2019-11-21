use std::{any::Any, collections::HashMap};

use dces::prelude::Component;

/// The struct `Registry` represents a global service registry. It is used to register and call
/// global Registry like settings service oder a database service.
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

    /// Gets a service.
    ///
    /// # Panics
    ///
    /// Panics if the there is no service for the given key or the given service type is wrong.
    pub fn get<C: Component>(&self, key: impl Into<String>) -> &C {
        let key = key.into();
        self.registry
            .get(&key)
            .expect(format!("Registry.get(): could not found key: {}", key).as_str())
            .downcast_ref()
            .expect(format!("Registry.get(): wrong type for key: {}", key).as_str())
    }

    /// Gets a service.
    ///
    /// # Panics
    ///
    /// Panics if the there is no service for the given key or the given service type is wrong.
    // pub fn try_get<C: Component>(&self, key: impl Into<String>) -> Option<&C> {
    //     self.Registry
    //         .get(&key.into())
    //         .ok_or_else(|| None)
    //         .map(|c| c.downcast_ref())
    // }

    /// Gets a mutable reference of the requested service.
    ///
    /// # Panics
    ///
    /// Panics if the there is no service for the given key or the given service type is wrong.
    pub fn get_mut<C: Component>(&mut self, key: impl Into<String>) -> &C {
        let key = key.into();
        self.registry
            .get_mut(&key)
            .expect(format!("Registry.get(): could not found key: {}", key).as_str())
            .downcast_ref()
            .expect(format!("Registry.get(): wrong type for key: {}", key).as_str())
    }
}
