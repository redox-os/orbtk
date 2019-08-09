use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, EntityComponentManager};

use crate::{prelude::*, render::*, shell::WindowShell, tree::Tree, utils::*};

use super::{MessageBox, WidgetContainer};

/// The `Context` is provides access for the states to objects they could work with.
pub struct Context<'a> {
    ecm: &'a mut EntityComponentManager<Tree>,
    window_shell: &'a mut WindowShell<WindowAdapter>,
    pub entity: Entity,
    pub theme: &'a ThemeValue,
    render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
}

impl<'a> Context<'a> {
    /// Creates a new container.
    pub fn new(
        entity: Entity,
        ecm: &'a mut EntityComponentManager<Tree>,
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
        }
    }

    /// Returns the widget of the current state context.
    pub fn widget(&mut self) -> WidgetContainer<'_> {
        WidgetContainer::new(self.entity, self.ecm)
    }

    /// Returns the current build context.
    pub fn build_context(&mut self) -> BuildContext {
        BuildContext::new(
            self.ecm,
            self.render_objects.clone(),
            self.layouts.clone(),
            self.handlers.clone(),
            self.states.clone(),
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
        WidgetContainer::new(self.ecm.entity_store().root, self.ecm)
    }

    /// Returns the entity id of an child by the given name.
    pub fn entity_of_child(&mut self, id: impl Into<String>) -> Option<Entity> {
        let id = id.into();

        let mut current_node = self.entity;

        loop {
            if let Ok(selector) = self
                .ecm
                .component_store()
                .borrow_component::<Selector>(current_node)
            {
                if let Some(child_id) = &selector.0.id {
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
            return Some(WidgetContainer::new(child, self.ecm));
        }

        None
    }

    /// Returns the child of the current widget.
    /// If the index is out of the children index bounds or the widget has no children None will be returned.
    pub fn widget_from_child_index(&mut self, index: usize) -> Option<WidgetContainer<'_>> {
        if index >= self.ecm.entity_store().children[&self.entity].len() {
            return None;
        }

        let entity = self.ecm.entity_store().children[&self.entity][index];

        Some(WidgetContainer::new(entity, self.ecm))
    }

    /// Returns the parent of the current widget.
    /// If the current widget is the root None will be returned.
    pub fn parent_widget(&mut self) -> Option<WidgetContainer<'_>> {
        if self.ecm.entity_store().parent[&self.entity] == None {
            return None;
        }

        let entity = self.ecm.entity_store().parent[&self.entity].unwrap();

        Some(WidgetContainer::new(entity, self.ecm))
    }

    /// Sends a message to the widget with the given id over the message channel.
    pub fn send_message(&mut self, target_widget: &str, message: impl Into<MessageBox>) {
        let mut entity = None;
        if let Ok(global) = self
            .ecm
            .component_store()
            .borrow_component::<Global>(0.into())
        {
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

    /// Update all css properties of the current widget by the current theme.
    pub fn update_theme_properties(&mut self) {
        if !self.widget().has::<Selector>() {
            return;
        }

        let selector = self.widget().clone::<Selector>();

        if !selector.0.dirty() {
            return;
        }

        if self.widget().has::<Foreground>() {
            if let Some(color) = self.theme.brush("color", &selector.0) {
                self.widget().set::<Foreground>(Foreground::from(color));
            }
        }

        if self.widget().has::<Background>() {
            if let Some(background) = self.theme.brush("background", &selector.0) {
                self.widget()
                    .set::<Background>(Background::from(background));
            }
        }

        if self.widget().has::<BorderBrush>() {
            if let Some(border_color) = self.theme.brush("border-color", &selector.0) {
                self.widget()
                    .set::<BorderBrush>(BorderBrush::from(border_color));
            }
        }

        if self.widget().has::<BorderRadius>() {
            if let Some(radius) = self.theme.float("border-radius", &selector.0) {
                self.widget()
                    .set::<BorderRadius>(BorderRadius::from(radius as f64));
            }
        }

        if self.widget().has::<BorderThickness>() {
            if let Some(border_width) = self.theme.uint("border-width", &selector.0) {
                self.widget()
                    .set::<BorderThickness>(BorderThickness::from(border_width as f64));
            }
        }

        if self.widget().has::<FontSize>() {
            if let Some(size) = self.theme.uint("font-size", &selector.0) {
                self.widget().set::<FontSize>(FontSize::from(size as f64));
            }
        }

        if self.widget().has::<Font>() {
            if let Some(font_family) = self.theme.string("font-family", &selector.0) {
                self.widget().set::<Font>(Font::from(font_family));
            }
        }

        if self.widget().has::<IconBrush>() {
            if let Some(color) = self.theme.brush("icon-color", &selector.0) {
                self.widget().set::<IconBrush>(IconBrush::from(color));
            }
        }

        if self.widget().has::<IconSize>() {
            if let Some(size) = self.theme.uint("icon-size", &selector.0) {
                self.widget().set::<IconSize>(IconSize::from(size as f64));
            }
        }

        if self.widget().has::<IconFont>() {
            if let Some(font_family) = self.theme.string("icon-family", &selector.0) {
                self.widget().set::<IconFont>(IconFont::from(font_family));
            }
        }

        if let Some(padding) = self.widget().try_clone::<Padding>() {
            if let Some(pad) = self.theme.uint("padding", &selector.0) {
                let mut padding = padding;
                padding.set_thickness(pad as f64);
                self.widget().set::<Padding>(padding);
            }
        }

        if let Some(padding) = self.widget().try_clone::<Padding>() {
            if let Some(left) = self.theme.uint("padding-left", &selector.0) {
                let mut padding = padding;
                padding.set_left(left as f64);
                self.widget().set::<Padding>(padding);
            }
        }

        if let Some(padding) = self.widget().try_clone::<Padding>() {
            if let Some(top) = self.theme.uint("padding-top", &selector.0) {
                let mut padding = padding;
                padding.set_top(top as f64);
                self.widget().set::<Padding>(padding);
            }
        }

        if let Some(padding) = self.widget().try_clone::<Padding>() {
            if let Some(right) = self.theme.uint("padding-right", &selector.0) {
                let mut padding = padding;
                padding.set_right(right as f64);
                self.widget().set::<Padding>(padding);
            }
        }

        if let Some(padding) = self.widget().try_clone::<Padding>() {
            if let Some(bottom) = self.theme.uint("padding-bottom", &selector.0) {
                let mut padding = padding;
                padding.set_bottom(bottom as f64);
                self.widget().set::<Padding>(padding);
            }
        }

        // todo padding, icon_margin

        self.widget().get_mut::<Selector>().0.set_dirty(true);
    }

    pub fn render_context_2_d(&mut self) -> &mut RenderContext2D {
        self.window_shell.render_context_2_d()
    }
}
