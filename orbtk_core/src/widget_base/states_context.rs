use std::{any::Any, collections::BTreeMap};

use dces::prelude::{Component, Entity, EntityComponentManager};

use crate::{tree::Tree, widget_base::MessageAdapter};

use super::State;

/// The `StatesContext` provides access to the widget states.
pub struct StatesContext<'a> {
    states: &'a mut BTreeMap<Entity, Box<dyn State>>,
    ecm: &'a mut EntityComponentManager<Tree>,
    message_adapter: &'a MessageAdapter,
}

impl<'a> StatesContext<'a> {
    /// Creates a new state context.
    pub fn new(
        states: &'a mut BTreeMap<Entity, Box<dyn State>>,
        ecm: &'a mut EntityComponentManager<Tree>,
        message_adapter: &'a MessageAdapter,
    ) -> Self {
        StatesContext {
            states,
            ecm,
            message_adapter,
        }
    }

    // Mark the widget as dirty.
    fn mark_as_dirty(&mut self, entity: Entity) {
        *self
            .ecm
            .component_store_mut()
            .get_mut::<bool>("dirty", entity)
            .unwrap() = true;

        let root = self.ecm.entity_store().root();

        if let Ok(dirty_widgets) = self
            .ecm
            .component_store_mut()
            .get_mut::<Vec<Entity>>("dirty_widgets", root)
        {
            // don't add the same widget twice in a row
            if dirty_widgets.is_empty() || *dirty_widgets.last().unwrap() != entity {
                dirty_widgets.push(entity);
            }
        }
    }

    /// Gets the state of the given widget.
    ///
    /// # Panics
    ///
    /// Panics if the there is no state for the given entity or the
    /// given state type is wrong.
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
    /// Panics if the there is no state for the given entity or the
    /// given state type is wrong.
    pub fn get_mut<S: Component>(&mut self, entity: Entity) -> &mut S {
        self.mark_as_dirty(entity);
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
        self.mark_as_dirty(entity);
        if let Some(e) = self.states.get_mut(&entity) {
            if let Some(r) = e.as_any_mut().downcast_mut() {
                return Some(r);
            }
        }

        None
    }

    /// Send a message to the given target widget.
    pub fn send_message<M: Any + Send>(&self, message: M, target: Entity) {
        self.message_adapter.send_message(message, target);
    }
}
