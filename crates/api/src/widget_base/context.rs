use std::{collections::BTreeMap, sync::mpsc};

use dces::prelude::*;

use raw_window_handle::RawWindowHandle;

use crate::{
    application::{create_window, ContextProvider},
    prelude::*,
    render::RenderContext2D,
    shell::{ShellRequest, WindowRequest},
    theming::prelude::*,
    tree::Tree,
};

use super::WidgetContainer;

/// The `Context` structure provides access to widget entities handled
/// via the underlying EntityComponentSystem. Functions are offered to
/// handle the position of an entity in the tree, its associated
/// render context and theme. You can emit adaptions to the state of
/// an entity.
pub struct Context<'a> {
    pub(crate) ecm: &'a mut EntityComponentManager<Tree>,
    entity: Entity,
    pub theme: Rc<Theme>,
    pub(crate) provider: &'a ContextProvider,
    new_states: BTreeMap<Entity, Box<dyn State>>,
    remove_widget_list: Vec<Entity>,
    render_context: &'a mut RenderContext2D,
}

impl<'a> Drop for Context<'a> {
    fn drop(&mut self) {
        self.provider
            .states
            .borrow_mut()
            .append(&mut self.new_states);
    }
}

impl<'a> Context<'a> {
    /// Creates a new container.
    pub fn new(
        ecs: (Entity, &'a mut EntityComponentManager<Tree>),
        theme: &Rc<Theme>,
        provider: &'a ContextProvider,
        render_context: &'a mut RenderContext2D,
    ) -> Self {
        Context {
            entity: ecs.0,
            ecm: ecs.1,
            theme: Rc::clone(&theme),
            provider,
            new_states: BTreeMap::new(),
            remove_widget_list: vec![],
            render_context,
        }
    }

    /// Gets the current entity.
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// Switch current `Context` to context of given widget `another`.
    /// Don't forget to change back to the original context once you are done.
    pub fn change_into(&mut self, another: Entity) {
        self.entity = another;
    }

    /// Access the raw window handle. Could be `None` on unsupported raw-window-handle platforms like `Redox`.
    pub fn raw_window_handle(&self) -> Option<RawWindowHandle> {
        if let Some(handle) = self.provider.raw_window_handle {
            return Some(handle);
        }

        None
    }

    // -- Widgets --

    /// Returns a specific widget.
    pub fn get_widget(&mut self, entity: Entity) -> WidgetContainer<'_> {
        WidgetContainer::new(
            entity,
            self.ecm,
            &self.theme,
            Some(&self.provider.event_adapter),
        )
    }

    /// Returns the widget of the current state ctx.
    pub fn widget(&mut self) -> WidgetContainer<'_> {
        self.get_widget(self.entity)
    }

    /// Returns the window widget.
    pub fn window(&mut self) -> WidgetContainer<'_> {
        let root = self.entity_of_window();
        self.get_widget(root)
    }

    /// Returns the entity of the window.
    pub fn entity_of_window(&mut self) -> Entity {
        self.ecm.entity_store().root()
    }

    /// Returns a child of the widget of the current state referenced by css `id`.
    /// If there is no id defined, it will panic.
    pub fn child<'b>(&mut self, id: impl Into<&'b str>) -> WidgetContainer<'_> {
        let id = id.into();
        let result = self
            .entity_of_child(id)
            .map(move |child| self.get_widget(child));

        if result.is_none() {
            panic!("Context::child: Could not find child with id: {}.", id);
        }

        result.unwrap()
    }

    /// Returns a child of the widget of the current state referenced by css `id`.
    /// If there is no id defined, None will returned.
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

    /// Returns the parent of the current widget.
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
            if let Ok(parent_id) = self.ecm.component_store().get::<String>("id", parent) {
                if parent_id == id {
                    return self.get_widget(parent);
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
    /// If there is no id defined None will be returned.
    pub fn try_parent_from_id<'b>(
        &mut self,
        id: impl Into<&'b str>,
    ) -> Option<WidgetContainer<'_>> {
        let mut current = self.entity;
        let id = id.into();

        while let Some(parent) = self.ecm.entity_store().parent[&current] {
            if let Ok(parent_id) = self.ecm.component_store().get::<String>("id", parent) {
                if parent_id == id {
                    return Some(self.get_widget(parent));
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
            &self.provider.render_objects,
            &self.provider.layouts,
            &self.provider.handler_map,
            &mut self.new_states,
            &self.theme,
            self.provider.event_adapter.clone(),
        )
    }

    /// Appends a child widget to the given parent.
    pub fn append_child_to<W: Widget>(&mut self, child: W, parent: Entity) {
        let bctx = &mut self.build_context();
        let child = child.build(bctx);
        bctx.append_child(parent, child);
    }

    /// Appends a child widget to overlay (on the top of the main tree). If the overlay does not
    /// exists an error will be returned.
    pub fn append_child_to_overlay<W: Widget>(&mut self, child: W) -> Result<(), String> {
        if let Some(overlay) = self.ecm.entity_store().overlay {
            let bctx = &mut self.build_context();
            let child = child.build(bctx);
            bctx.append_child(overlay, child);
            return Ok(());
        }

        Err("Context.append_child_to_overlay: Could not find overlay.".to_string())
    }

    /// Appends a child widget by entity to the given parent.
    pub fn append_child_entity_to(&mut self, child: Entity, parent: Entity) {
        self.build_context().append_child(parent, child)
    }

    /// Appends a child entity to overlay (on the top of the main tree). If the overlay does not
    /// exists an error will be returned.
    pub fn append_child_entity_to_overlay(&mut self, child: Entity) -> Result<(), String> {
        if let Some(overlay) = self.ecm.entity_store().overlay {
            self.append_child_entity_to(overlay, child);
            return Ok(());
        }

        Err("Context.append_child_entity_to_overlay: Could not find overlay.".to_string())
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

    /// Removes a child from the overlay. If the given entity is not a child
    /// of the given parent nothing will happen.
    pub fn remove_child_from_overlay(&mut self, child: Entity) -> Result<(), String> {
        if let Some(overlay) = self.ecm.entity_store().overlay {
            self.remove_child_from(child, overlay);
            return Ok(());
        }

        Err("Context.remove_child_from_overlay: Could not find overlay.".to_string())
    }

    /// Removes (recursive) a child from the given parent. If the given entity is not a child
    /// of the given parent nothing will happen.
    pub fn remove_child_from(&mut self, remove_entity: Entity, parent: Entity) {
        let tree = &*self.ecm.entity_store();
        if let Some(parent) = find_parent(tree, remove_entity, parent) {
            self.remove_widget_list.push(remove_entity);

            let index = self.ecm.entity_store().children[&parent]
                .iter()
                .position(|&r| r == remove_entity)
                .unwrap();
            if let Some(parent) = self.ecm.entity_store().children.get_mut(&parent) {
                parent.remove(index);
            }
        }
    }

    /// Returns a mutable reference of the children that should be removed.
    pub fn remove_widget_list(&mut self) -> &mut Vec<Entity> {
        &mut self.remove_widget_list
    }

    /// Clears all children of the current widget.
    pub fn clear_children(&mut self) {
        self.clear_children_of(self.entity);
    }

    /// Clears all children of the given widget.
    pub fn clear_children_of(&mut self, parent: Entity) {
        let root = self.ecm.entity_store().root();
        while !self.ecm.entity_store().children[&parent].is_empty() {
            let child = self.ecm.entity_store().children[&parent][0];

            if let Some(index) = self
                .ecm
                .component_store()
                .get::<Vec<Entity>>("dirty_widgets", root)
                .unwrap()
                .iter()
                .position(|&r| r == child)
            {
                // remove child also from list of dirty widgets
                if let Ok(dirty_widgets) = self
                    .ecm
                    .component_store_mut()
                    .get_mut::<Vec<Entity>>("dirty_widgets", root)
                {
                    dirty_widgets.remove(index);
                }
            }

            self.remove_child_from(child, parent);
        }
    }

    // -- Manipulation --

    /// Returns the entity of a child, identified by its id.
    /// If there is no matching id string, `None` will be returned.
    pub fn entity_of_child<'b>(&mut self, id: impl Into<&'b str>) -> Option<Entity> {
        let id = id.into();

        let mut current_node = self.entity;

        loop {
            if let Ok(child_id) = self.ecm.component_store().get::<String>("id", current_node) {
                if child_id == id {
                    return Some(current_node);
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
    /// If there is no id defined None will be returned.
    pub fn parent_entity_by_style<'b>(&mut self, element: impl Into<&'b str>) -> Option<Entity> {
        let mut current = self.entity;
        let element = element.into();

        while let Some(parent) = self.ecm.entity_store().parent[&current] {
            if let Ok(selector) = self
                .ecm
                .component_store()
                .get::<Selector>("selector", parent)
            {
                if let Some(parent_element) = &selector.style {
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

    /// Creates and show a new window.
    pub fn show_window<F: Fn(&mut BuildContext) -> Entity + 'static>(&mut self, create_fn: F) {
        let (adapter, settings, receiver) = create_window(
            self.provider.application_name.clone(),
            &self.theme,
            self.provider.shell_sender.clone(),
            create_fn,
            self.provider.localization.clone(),
        );
        self.provider
            .shell_sender
            .send(ShellRequest::CreateWindow(adapter, settings, receiver))
            .expect("Context.show_window: Could not send shell request.");
    }

    /// Returns a mutable reference of the 2d render ctx.
    pub fn render_context_2_d(&mut self) -> &mut RenderContext2D {
        self.render_context
    }

    /// Returns a keys collection of new added states.
    pub fn new_states_keys(&self) -> Vec<Entity> {
        self.new_states.keys().cloned().collect()
    }

    /// Switch the current theme.
    pub fn switch_theme(&mut self, theme: Rc<Theme>) {
        self.theme = Rc::clone(&theme);

        *self.window().get_mut::<Rc<Theme>>("theme") = theme;

        for (key, font) in self.theme.fonts() {
            self.render_context.register_font(key, *font);
        }

        // update on window to update all widgets in the tree
        self.window().update_dirty(true);
    }

    /// Sets the current language.
    pub fn set_language(&mut self, key: &str) {
        if let Some(localization) = &self.provider.localization {
            localization.borrow_mut().set_language(key);
        }

        let root = self.ecm.entity_store().root.unwrap();
        self.get_widget(root).update_dirty(true);
    }

    /// Used to localize a text. If there is no localized text for the given key or no localization service the key will be returned as result.
    pub fn localize_text(&self, key: String) -> String {
        if let Some(localization) = &self.provider.localization {
            return localization.borrow().text(key);
        }

        key
    }

    /// Returns a cloned event adapter.
    pub fn event_adapter(&self) -> EventAdapter {
        self.provider.event_adapter.clone()
    }

    /// Returns a cloned message adapter.
    pub fn message_adapter(&self) -> MessageAdapter {
        self.provider.message_adapter.clone()
    }

    /// Sends a message to the given entity.
    pub fn send_message<M: Any + Send>(&self, message: M, entity: Entity) {
        self.provider.message_adapter.send_message(message, entity);
    }

    /// Gets a new sender that allows to communicate with the window shell.
    pub fn send_window_request(&self, request: WindowRequest) {
        self.provider
            .window_sender
            .send(request)
            .expect("Context::send_window_request: could not send request to window.");
    }

    /// Pushes an event to the event queue.
    #[deprecated = "Will be removed on 0.3.1-alpha5. Use EventAdapter instead"]
    pub fn push_event<E: Event + Send>(&mut self, event: E) {
        self.provider.event_adapter.push_event(self.entity, event);
    }

    /// Pushes an event to the event queue.
    #[deprecated = "Will be removed on 0.3.1-alpha5. Use EventAdapter instead"]
    pub fn push_event_by_entity<E: Event + Send>(&mut self, event: E, entity: Entity) {
        self.provider.event_adapter.push_event(entity, event);
    }

    /// Pushes an event to the event queue.
    #[deprecated = "Will be removed on 0.3.1-alpha5. Use EventAdapter instead"]
    pub fn push_event_by_window<E: Event + Send>(&mut self, event: E) {
        self.provider
            .event_adapter
            .push_event(self.entity_of_window(), event);
    }

    /// Gets a window request sender.
    #[deprecated = "Will be removed on 0.3.1-alpha5. Use send_window_request instead"]
    pub fn window_sender(&self) -> mpsc::Sender<WindowRequest> {
        self.provider.window_sender.clone()
    }
}

// -- Helpers --

/// Finds the parent of the `target_child`. The parent of the
/// `target_child` must be the given `parent` or a child of the given
/// parent.
pub fn find_parent(tree: &Tree, target_child: Entity, parent: Entity) -> Option<Entity> {
    if tree.children[&parent].contains(&target_child) {
        return Some(parent);
    }

    for child in &tree.children[&parent] {
        let parent = find_parent(tree, target_child, *child);
        if parent.is_some() {
            return parent;
        }
    }

    None
}

pub fn get_all_children(children: &mut Vec<Entity>, parent: Entity, tree: &Tree) {
    for child in &tree.children[&parent] {
        children.push(*child);
        get_all_children(children, *child, tree);
    }
}

// -- Helpers --
