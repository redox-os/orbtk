use std::{cell::RefCell, collections::BTreeMap};

use dces::prelude::{Entity, EntityComponentManager};

use crate::{css_engine::*, prelude::*, render::*, shell::WindowShell, tree::Tree};

use super::{MessageBox, WidgetContainer};

/// The `Context` is provides access for the states to objects they could work with.
pub struct Context<'a> {
    ecm: &'a mut EntityComponentManager<Tree, StringComponentStore>,
    window_shell: &'a mut WindowShell<WindowAdapter>,
    pub entity: Entity,
    pub theme: &'a ThemeValue,
    render_objects: &'a RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>,
    layouts: &'a mut BTreeMap<Entity, Box<dyn Layout>>,
    handlers: &'a mut EventHandlerMap,
    states: &'a RefCell<BTreeMap<Entity, Box<dyn State>>>,
    new_states: &'a mut BTreeMap<Entity, Box<dyn State>>,
}

impl<'a> Drop for Context<'a> {
    fn drop(&mut self) {
        self.states.borrow_mut().append(&mut self.new_states);
    }
}

impl<'a> Context<'a> {
    /// Creates a new container.
    pub fn new(
        ecs: (
            Entity,
            &'a mut EntityComponentManager<Tree, StringComponentStore>,
        ),
        window_shell: &'a mut WindowShell<WindowAdapter>,
        theme: &'a ThemeValue,
        render_objects: &'a RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>,
        layouts: &'a mut BTreeMap<Entity, Box<dyn Layout>>,
        handlers: &'a mut EventHandlerMap,
        states: &'a RefCell<BTreeMap<Entity, Box<dyn State>>>,
        new_states: &'a mut BTreeMap<Entity, Box<dyn State>>,
    ) -> Self {
        Context {
            entity: ecs.0,
            ecm: ecs.1,
            window_shell,
            theme,
            render_objects,
            layouts,
            handlers,
            states,
            new_states,
        }
    }

    // -- Widgets --

    /// Returns a specific widget.
    pub fn get_widget(&mut self, entity: Entity) -> WidgetContainer<'_> {
        WidgetContainer::new(entity, self.ecm, self.theme)
    }

    /// Returns the widget of the current state ctx.
    pub fn widget(&mut self) -> WidgetContainer<'_> {
        self.get_widget(self.entity)
    }

    /// Returns the window widget.
    pub fn window(&mut self) -> WidgetContainer<'_> {
        let root = self.ecm.entity_store().root;
        self.get_widget(root)
    }

    /// Returns a child of the widget of the current state referenced by css `id`.
    /// If the no id is defined its panics.
    pub fn child<'b>(&mut self, id: impl Into<&'b str>) -> WidgetContainer<'_> {
        self.entity_of_child(id)
            .map(move |child| self.get_widget(child))
            .unwrap()
    }

    /// Returns a child of the widget of the current state referenced by css `id`.
    /// If the no id is defined None will returned.
    pub fn try_child<'b>(&mut self, id: impl Into<&'b str>) -> Option<WidgetContainer<'_>> {
        self.entity_of_child(id)
            .map(move |child| self.get_widget(child))
    }

    /// Returns the parent of the current widget.
    /// Panics if the parent does not exists.
    pub fn parent(&mut self) -> WidgetContainer<'_> {
        let entity = self.ecm.entity_store().parent[&self.entity].unwrap();
        self.get_widget(entity)
    }

    // Returns the parent of the current widget.
    /// If the current widget is the root None will be returned.
    pub fn try_parent(&mut self) -> Option<WidgetContainer<'_>> {
        if self.ecm.entity_store().parent[&self.entity] == None {
            return None;
        }

        let entity = self.ecm.entity_store().parent[&self.entity].unwrap();

        Some(self.get_widget(entity))
    }

    /// Returns a parent of the widget of the current state referenced by css `id`.
    /// Panics if a parent with the given id could not be found
    pub fn parent_from_id<'b>(&mut self, id: impl Into<&'b str>) -> WidgetContainer<'_> {
        let mut current = self.entity;
        let id = id.into();

        while let Some(parent) = self.ecm.entity_store().parent[&current] {
            if let Ok(selector) = self
                .ecm
                .component_store()
                .get::<Selector>("selector", parent)
            {
                if let Some(parent_id) = &selector.id {
                    if parent_id == id {
                        return self.get_widget(parent);
                    }
                }
            }

            current = parent;
        }

        panic!(
            "Parent with id: {}, of child with entity: {} could not be found",
            id, self.entity.0
        );
    }

    /// Returns a parent of the widget of the current state referenced by css `id`.
    /// If the no id is defined None will returned.
    pub fn try_parent_from_id<'b>(
        &mut self,
        id: impl Into<&'b str>,
    ) -> Option<WidgetContainer<'_>> {
        let mut current = self.entity;
        let id = id.into();

        while let Some(parent) = self.ecm.entity_store().parent[&current] {
            if let Ok(selector) = self
                .ecm
                .component_store()
                .get::<Selector>("selector", parent)
            {
                if let Some(parent_id) = &selector.id {
                    if parent_id == id {
                        return Some(self.get_widget(parent));
                    }
                }
            }

            current = parent;
        }

        None
    }

    /// Returns the child of the current widget.
    /// Panics if a child on the given index could not be found.
    pub fn child_from_index(&mut self, index: usize) -> WidgetContainer<'_> {
        let entity = self.ecm.entity_store().children[&self.entity][index];
        self.get_widget(entity)
    }

    /// Returns the child of the current widget.
    /// If the index is out of the children index bounds or the widget has no children None will be returned.
    pub fn try_child_from_index(&mut self, index: usize) -> Option<WidgetContainer<'_>> {
        if index >= self.ecm.entity_store().children[&self.entity].len() {
            return None;
        }

        let entity = self.ecm.entity_store().children[&self.entity][index];

        Some(self.get_widget(entity))
    }

    // -- Widgets --

    // -- Manipulation --

    /// Returns the current build ctx.
    pub fn build_context(&mut self) -> BuildContext {
        BuildContext::new(
            self.ecm,
            self.render_objects,
            self.layouts,
            self.handlers,
            self.new_states,
            self.theme,
        )
    }

    /// Appends a child widget to the given parent.
    pub fn append_child_to<W: Widget>(&mut self, child: W, parent: Entity) {
        let bctx = &mut self.build_context();
        let child = child.build(bctx);
        bctx.append_child(parent, child)
    }

    /// Appends a child widget by entity to the given parent.
    pub fn append_child_entity_to(&mut self, child: Entity, parent: Entity) {
        self.build_context().append_child(parent, child)
    }

    /// Appends a child to the current widget.
    pub fn append_child<W: Widget>(&mut self, child: W) {
        self.append_child_to(child, self.entity);
    }

    /// Appends a child widget by entity to the current widget.
    pub fn append_child_entity(&mut self, child: Entity) {
        self.append_child_entity_to(self.entity, child);
    }

    /// Removes a child from the current widget. If the given entity is not a child
    /// of the given parent nothing will happen.
    pub fn remove_child(&mut self, child: Entity) {
        self.remove_child_from(child, self.entity);
    }

    /// Removes a child from the given parent. If the given entity is not a child
    /// of the given parent nothing will happen.
    pub fn remove_child_from(&mut self, child: Entity, parent: Entity) {
        if self.ecm.entity_store().children[&parent].contains(&child) {
            self.ecm.remove_entity(child);
        }
    }

    /// Clears all children of the current widget.
    pub fn clear_children(&mut self) {
        self.clear_children_of(self.entity);
    }

    /// Clears all children of the given widget.
    pub fn clear_children_of(&mut self, parent: Entity) {
        while !self.ecm.entity_store().children[&parent].is_empty() {
            let child = self.ecm.entity_store().children[&parent][0];

            self.ecm.remove_entity(child);
        }
    }

    // -- Manipulation --

    /// Returns the entity id of an child by the given name.
    pub fn entity_of_child<'b>(&mut self, id: impl Into<&'b str>) -> Option<Entity> {
        let id = id.into();

        let mut current_node = self.entity;

        loop {
            if let Ok(selector) = self
                .ecm
                .component_store()
                .get::<Selector>("selector", current_node)
            {
                if let Some(child_id) = &selector.id {
                    if child_id == id {
                        return Some(current_node);
                    }
                }
            }

            let mut it = self.ecm.entity_store().start_node(current_node).into_iter();
            it.next();

            if let Some(node) = it.next() {
                current_node = node;
            } else {
                break;
            }
        }

        None
    }

    /// Returns the entity of the parent referenced by css `element`.
    /// If the no id is defined None will returned.
    pub fn parent_entity_by_element<'b>(&mut self, element: impl Into<&'b str>) -> Option<Entity> {
        let mut current = self.entity;
        let element = element.into();

        while let Some(parent) = self.ecm.entity_store().parent[&current] {
            if let Ok(selector) = self
                .ecm
                .component_store()
                .get::<Selector>("selector", parent)
            {
                if let Some(parent_element) = &selector.element {
                    if parent_element == element
                        && self
                            .ecm
                            .component_store()
                            .is_origin::<Selector>("selector", parent)
                    {
                        return Some(parent);
                    }
                }
            }

            current = parent;
        }

        None
    }

    /// Returns the entity of the parent.
    pub fn entity_of_parent(&mut self) -> Option<Entity> {
        self.ecm.entity_store().parent[&self.entity]
    }

    /// Returns the child index of the current entity.
    pub fn index_as_child(&mut self, entity: Entity) -> Option<usize> {
        if let Some(parent) = self.ecm.entity_store().parent[&entity] {
            return self.ecm.entity_store().children[&parent]
                .iter()
                .position(|e| *e == entity);
        }

        None
    }

    /// Sends a message to the widget with the given id over the message channel.
    pub fn send_message(&mut self, target_widget: &str, message: impl Into<MessageBox>) {
        let mut entity = None;
        if let Ok(global) = self.ecm.component_store().get::<Global>("global", 0.into()) {
            if let Some(en) = global.id_map.get(target_widget) {
                entity = Some(*en);
            }
        }

        if let Some(entity) = entity {
            self.window_shell
                .adapter()
                .messages
                .entry(entity)
                .or_insert_with(Vec::new)
                .push(message.into());
        } else {
            println!(
                "Context send_message: widget id {} not found.",
                target_widget
            );
        }
    }

    /// Pushes an event to the event queue with the given `strategy`.
    pub fn push_event_strategy<E: Event>(&mut self, event: E, strategy: EventStrategy) {
        self.window_shell
            .adapter()
            .event_queue
            .register_event_with_strategy(event, strategy, self.entity);
    }

    /// Pushes an event to the event queue.
    pub fn push_event<E: Event>(&mut self, event: E) {
        self.window_shell
            .adapter()
            .event_queue
            .register_event(event, self.entity);
    }

    /// Pushes an event to the event queue.
    pub fn push_event_by_entity<E: Event>(&mut self, event: E, entity: Entity) {
        self.window_shell
            .adapter()
            .event_queue
            .register_event(event, entity);
    }

    /// Returns a mutable reference of the 2d render ctx.
    pub fn render_context_2_d(&mut self) -> &mut RenderContext2D {
        self.window_shell.render_context_2_d()
    }
}
