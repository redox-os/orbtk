use std::collections::BTreeMap;

use dces::prelude::{Component, Entity};

use super::State;

/// The `StatesContext` provides access to the widget states.
pub struct StatesContext<'a> {
    states: &'a mut BTreeMap<Entity, Box<dyn State>>,
}

impl<'a> StatesContext<'a> {
    pub fn new(states: &'a mut BTreeMap<Entity, Box<dyn State>>) -> Self {
        StatesContext { states }
    }

    /// Gets the state of the given widget.
    ///
    /// # Panics
    ///
    /// Panics if the there is no state for the given entity or the given state type is wrong.
    pub fn get<S: Component>(&self, entity: Entity) -> &S {
        self.states
            .get(&entity)
            .unwrap_or_else(|| {
                panic!(
                    "StatesContext.get(): state for entity: {:?} could not be found.",
                    entity
                )
            })
            .as_any()
            .downcast_ref()
            .unwrap_or_else(|| {
                panic!(
                    "StatesContext.get(): wrong type of state for entity: {:?}",
                    entity
                )
            })
    }

    /// Gets a mutable state of the given widget.
    ///
    /// # Panics
    ///
    /// Panics if the there is no state for the given entity or the given state type is wrong.
    pub fn get_mut<S: Component>(&mut self, entity: Entity) -> &mut S {
        self.states
            .get_mut(&entity)
            .unwrap_or_else(|| {
                panic!(
                    "StatesContext.get(): state for entity: {:?} could not be found.",
                    entity
                )
            })
            .as_any_mut()
            .downcast_mut()
            .unwrap_or_else(|| {
                panic!(
                    "StatesContext.get(): wrong type of state for entity: {:?}",
                    entity
                )
            })
    }

    /// Try to get the state of the given widget.
    pub fn try_get<S: Component>(&self, entity: Entity) -> Option<&S> {
        if let Some(e) = self.states.get(&entity) {
            if let Some(r) = e.as_any().downcast_ref() {
                return Some(r);
            }
        }

        None
    }

    /// Try to get a mutable reference of the state of the given widget.
    pub fn try_get_mut<S: Component>(&mut self, entity: Entity) -> Option<&mut S> {
        if let Some(e) = self.states.get_mut(&entity) {
            if let Some(r) = e.as_any_mut().downcast_mut() {
                return Some(r);
            }
        }

        None
    }
}
