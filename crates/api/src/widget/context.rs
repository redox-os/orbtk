use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, EntityComponentManager};

use crate::{css_engine::*, prelude::*, render::*, shell::WindowShell, tree::Tree};

use super::{MessageBox, WidgetContainer};

/// The `Context` is provides access for the states to objects they could work with.
pub struct Context<'a> {
    ecm: &'a mut EntityComponentManager<Tree, StringComponentStore>,
    window_shell: &'a mut WindowShell<WindowAdapter>,
    pub entity: Entity,
    pub theme: &'a ThemeValue,
    render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    new_states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
}

impl<'a> Drop for Context<'a> {
    fn drop(&mut self) {
        self.states
            .borrow_mut()
            .append(&mut self.new_states.borrow_mut());
    }
}

impl<'a> Context<'a> {
    /// Creates a new container.
    pub fn new(
        entity: Entity,
        ecm: &'a mut EntityComponentManager<Tree, StringComponentStore>,
        window_shell: &'a mut WindowShell<WindowAdapter>,
        theme: &'a ThemeValue,
        render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
        layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
        states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    ) -> Self {
        Context {
            entity,
            ecm,
            window_shell,
            theme,
            render_objects,
            layouts,
            handlers,
            states,
            new_states: Rc::new(RefCell::new(BTreeMap::new())),
        }
    }

    /// Returns a specific widget.
    pub fn get_widget(&mut self, entity: Entity) -> WidgetContainer<'_> {
        WidgetContainer::new(entity, self.ecm, self.theme)
    }

    /// Returns the widget of the current state context.
    pub fn widget(&mut self) -> WidgetContainer<'_> {
        self.get_widget(self.entity)
    }

    /// Returns the current build context.
    pub fn build_context(&mut self) -> BuildContext {
        BuildContext::new(
            self.ecm,
            self.render_objects.clone(),
            self.layouts.clone(),
            self.handlers.clone(),
            self.new_states.clone(),
        )
    }

    /// Appends a child widget to the given parent.
    pub fn append_child_to<W: Widget>(&mut self, child: W, parent: Entity) {
        let mut build_context = self.build_context();
        let child = child.build(&mut build_context);
        build_context.append_child(parent, child);
    }

    /// Appends a child to the current widget.
    pub fn append_child<W: Widget>(&mut self, child: W) {
        self.append_child_to(child, self.entity);
    }

    /// Clears all children of the current widget.
    pub fn clear_children(&mut self) {
        self.clear_children_of(self.entity);
    }

    /// Clears all children of the given widget.
    pub fn clear_children_of(&mut self, parent: Entity) {
        loop {
            if self.ecm.entity_store().children[&parent].len() == 0 {
                break;
            }

            let child = self.ecm.entity_store().children[&parent][0];

            self.ecm.remove_entity(child);
        }
    }

    /// Returns the window widget.
    pub fn window(&mut self) -> WidgetContainer<'_> {
        let root = self.ecm.entity_store().root;
        self.get_widget(root)
    }

    /// Returns the entity id of an child by the given name.
    pub fn entity_of_child(&mut self, id: impl Into<String>) -> Option<Entity> {
        let id = id.into();

        let mut current_node = self.entity;

        loop {
            if let Ok(selector) = self
                .ecm
                .component_store()
                .get::<Selector>("selector", current_node)
            {
                if let Some(child_id) = &selector.id {
                    if child_id.eq(&id) {
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

    /// Returns a child of the widget of the current state referenced by css `id`.
    /// If the no id is defined None will returned.
    pub fn child_by_id(&mut self, id: impl Into<String>) -> Option<WidgetContainer<'_>> {
        if let Some(child) = self.entity_of_child(id) {
            return Some(self.get_widget(child));
        }

        None
    }

    /// Returns the entity of the parent referenced by css `element`.
    /// If the no id is defined None will returned.
    pub fn parent_entity_by_element(&mut self, element: impl Into<String>) -> Option<Entity> {
        let mut current = self.entity;
        let element = element.into();

        loop {
            if let Some(parent) = self.ecm.entity_store().parent[&current] {
                if let Ok(selector) = self
                    .ecm
                    .component_store()
                    .get::<Selector>("selector", parent)
                {
                    if let Some(parent_element) = &selector.element {
                        if parent_element.eq(&element) {
                            if self
                                .ecm
                                .component_store()
                                .is_origin::<Selector>("selector", parent)
                            {
                                return Some(parent);
                            }
                        }
                    }
                }

                current = parent;
            } else {
                break;
            }
        }

        None
    }

    /// Returns a parent of the widget of the current state referenced by css `id`.
    /// If the no id is defined None will returned.
    pub fn parent_by_id(&mut self, id: impl Into<String>) -> Option<WidgetContainer<'_>> {
        let mut current = self.entity;
        let id = id.into();

        loop {
            if let Some(parent) = self.ecm.entity_store().parent[&current] {
                if let Ok(selector) = self
                    .ecm
                    .component_store()
                    .get::<Selector>("selector", parent)
                {
                    if let Some(parent_id) = &selector.id {
                        if parent_id.eq(&id) {
                            return Some(self.get_widget(parent));
                        }
                    }
                }

                current = parent;
            } else {
                break;
            }
        }

        None
    }

    /// Returns the child of the given widget.
    /// If the index is out of the children index bounds or the widget has no children None will be returned.
    pub fn child_of_parent(&mut self, parent: Entity, index: usize) -> Option<WidgetContainer<'_>> {
        if index >= self.ecm.entity_store().children[&parent].len() {
            return None;
        }

        let entity = self.ecm.entity_store().children[&parent][index];

        Some(self.get_widget(entity))
    }

    /// Returns the child of the current widget.
    /// If the index is out of the children index bounds or the widget has no children None will be returned.
    pub fn widget_from_child_index(&mut self, index: usize) -> Option<WidgetContainer<'_>> {
        self.child_of_parent(self.entity, index)
    }

    /// Returns the entity of the parent.
    pub fn entity_of_parent(&mut self) -> Option<Entity> {
        self.ecm.entity_store().parent[&self.entity]
    }

    /// Returns the parent of the current widget.
    /// If the current widget is the root None will be returned.
    pub fn parent_widget(&mut self) -> Option<WidgetContainer<'_>> {
        if self.ecm.entity_store().parent[&self.entity] == None {
            return None;
        }

        let entity = self.ecm.entity_store().parent[&self.entity].unwrap();

        Some(self.get_widget(entity))
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
            if !self.window_shell.adapter().messages.contains_key(&entity) {
                self.window_shell.adapter().messages.insert(entity, vec![]);
            }
            self.window_shell
                .adapter()
                .messages
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

    /// Returns a mutable reference of the 2d render context.
    pub fn render_context_2_d(&mut self) -> &mut RenderContext2D {
        self.window_shell.render_context_2_d()
    }
}
