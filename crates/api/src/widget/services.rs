use std::{any::Any, collections::HashMap};

use dces::prelude::Component;

/// The struct `Services` represents a global service registry. It is used to register and call
/// global services like settings service oder a database service.
#[derive(Default)]
pub struct Services {
    services: HashMap<String, Box<dyn Any>>,
}

impl Services {
    /// Creates a service registry with an empty services map.
    pub fn new() -> Self {
        Services::default()
    }

    /// Register a new services with the given key.
    pub fn register<C: Component>(&mut self, key: impl Into<String>, service: C) {
        self.services.insert(key.into(), Box::new(service));
    }

    /// Gets a service.
    ///
    /// # Panics
    ///
    /// Panics if the there is no service for the given key or the given service type is wrong.
    pub fn get<C: Component>(&self, key: impl Into<String>) -> &C {
        self.services
            .get(&key.into())
            .unwrap()
            .downcast_ref()
            .unwrap()
    }

    /// Gets a service.
    ///
    /// # Panics
    ///
    /// Panics if the there is no service for the given key or the given service type is wrong.
    // pub fn try_get<C: Component>(&self, key: impl Into<String>) -> Option<&C> {
    //     self.services
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
        self.services
            .get_mut(&key.into())
            .unwrap()
            .downcast_ref()
            .unwrap()
    }
}
