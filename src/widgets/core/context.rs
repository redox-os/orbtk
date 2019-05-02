use std::{cell::RefCell, collections::BTreeMap};

use dces::prelude::{Entity, EntityComponentManager};

use super::{MessageBox, WidgetContainer};

use crate::{
    application::{Global, Tree},
    event::{Event, EventQueue, EventStrategy},
    properties::Selector,
    theme::Theme,
};

/// The `Context` is provides access for the states to objects they could work with.
pub struct Context<'a> {
    ecm: &'a mut EntityComponentManager,
    tree: &'a Tree,
    event_queue: &'a RefCell<EventQueue>,
    messages: Option<&'a RefCell<BTreeMap<Entity, Vec<MessageBox>>>>,
    pub entity: Entity,
    pub theme: &'a Theme,
}

impl<'a> Context<'a> {
    /// Creates a new container.
    pub fn new(
        entity: Entity,
        ecm: &'a mut EntityComponentManager,
        tree: &'a Tree,
        event_queue: &'a RefCell<EventQueue>,
        theme: &'a Theme,
        messages: Option<&'a RefCell<BTreeMap<Entity, Vec<MessageBox>>>>,
    ) -> Self {
        Context {
            entity,
            ecm,
            tree,
            event_queue,
            messages,
            theme,
        }
    }

    /// Returns the widget of the current state context.
    pub fn widget(&mut self) -> WidgetContainer<'_> {
        WidgetContainer::new(self.entity, &mut self.ecm)
    }
    
    /// Returns the window widget.
    pub fn window(&mut self) -> WidgetContainer<'_> {
        WidgetContainer::new(0, &mut self.ecm)
    }

    /// Returns a child of the widget of the current state referenced by css `id`.
    /// If the no id is defined None will returned.
    pub fn child_by_id<S: Into<String>>(&mut self, id: S) -> Option<WidgetContainer<'_>> {
        let id = id.into();
        for child in self.tree.start_node(self.entity).into_iter() {
            if let Ok(selector) = self.ecm.borrow_component::<Selector>(child) {
                if let Some(child_id) = &selector.0.id {
                    if child_id.eq(&id) {
                        return Some(WidgetContainer::new(child, &mut self.ecm));
                    }
                }
            }
        }

        None
    }

    /// Returns the child of the current widget.
    /// If the index is out of the children index bounds or the widget has no children None will be returned.
    pub fn widget_from_child_index(&mut self, index: usize) -> Option<WidgetContainer<'_>> {
        if index >= self.tree.children[&self.entity].len() {
            return None;
        }

        Some(WidgetContainer::new(
            self.tree.children[&self.entity][index],
            &mut self.ecm,
        ))
    }

    /// Returns the parent of the current widget.
    /// If the current widget is the root None will be returned.
    pub fn parent_widget(&mut self) -> Option<WidgetContainer<'_>> {
        if self.tree.parent[&self.entity] == None {
            return None;
        }

        Some(WidgetContainer::new(
            self.tree.parent[&self.entity].unwrap(),
            &mut self.ecm,
        ))
    }

    /// Sends a message to the widget with the given id over the message channel.
    pub fn send_message(&mut self, target_widget: &str, message: impl Into<MessageBox>) {
        if let Some(messages) = &self.messages {
            let mut entity = None;
            if let Ok(global) = self.ecm.borrow_component::<Global>(0) {
                if let Some(en) = global.id_map.get(target_widget) {
                    entity = Some(*en);
                }
            }

            if let Some(entity) = entity {
                if !messages.borrow().contains_key(&entity) {
                    messages.borrow_mut().insert(entity, vec![]);
                }
                messages
                    .borrow_mut()
                    .get_mut(&entity)
                    .unwrap()
                    .push(message.into());
            } else {
                println!(
                    "Context send_message: widget id {} not found.",
                    target_widget
                );
            }
        }
    }

    /// Pushes an event to the event queue with the given `strategy`.
    pub fn push_event_strategy<E: Event>(&mut self, event: E, strategy: EventStrategy) {
        self.event_queue
            .borrow_mut()
            .register_event_with_strategy(event, strategy, self.entity);
    }

    /// Pushes an event to the event queue.
    pub fn push_event<E: Event>(&self, event: E) {
        self.event_queue
            .borrow_mut()
            .register_event(event, self.entity);
    }
}
