use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Component, ComponentBox, Entity, EntityComponentManager, SharedComponentBox};

use crate::{prelude::*, tree::Tree};

use super::State;

pub type WidgetBuildContext = Option<Box<dyn Fn(&mut BuildContext, usize) -> Entity + 'static>>;

// impl Clone<T> for Box<T> where T:  Fn(&mut BuildContext, usize) -> Entity + 'static + Clone {
//     fn clone(&self) -> Self {

//     }
// }

/// Used to create an entity for a widget with its properties as components.
#[derive(Constructor)]
pub struct BuildContext<'a> {
    ecm: &'a mut EntityComponentManager<Tree, StringComponentStore>,
    render_objects: &'a RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>,
    layouts: &'a RefCell<BTreeMap<Entity, Box<dyn Layout>>>,
    handlers: &'a RefCell<EventHandlerMap>,
    states: &'a mut BTreeMap<Entity, Box<dyn State>>,
    theme: &'a Theme,
}

impl<'a> BuildContext<'a> {
    /// Returns a specific widget.
    pub fn get_widget(&mut self, entity: Entity) -> WidgetContainer<'_> {
        WidgetContainer::new(entity, self.ecm, self.theme)
    }

    /// Creates a new entity.
    pub fn create_entity(&mut self) -> Entity {
        self.ecm.create_entity().build()
    }

    /// Update theme by state.
    pub fn update_theme_by_state(&mut self, entity: Entity) {
        self.get_widget(entity).update(true);
    }

    /// Appends a child to a parent.
    pub fn append_child(&mut self, parent: Entity, child: Entity) {
        self.ecm
            .entity_store_mut()
            .append_child(parent, child)
            .unwrap();
    }

    /// Appends a child to overlay (on the top of the main tree). If the overlay does not exists an
    /// error will be returned.
    pub fn append_child_to_overlay(&mut self, child: Entity) -> Result<(), String> {
        if let Some(overlay) = self.ecm.entity_store().overlay {
            self.append_child(overlay.into(), child);
            return Ok(());
        }

        Err("BuildContext.append_child_to_overlay: Could not find overlay.".to_string())
    }

    /// Registers a property as component.
    pub fn register_property<P: Component>(&mut self, key: &str, widget: Entity, property: P) {
        self.ecm
            .component_store_mut()
            .register(key, widget, property);
    }

    /// Registers a property box as component.
    pub fn register_property_box(&mut self, key: &str, widget: Entity, property: ComponentBox) {
        self.ecm
            .component_store_mut()
            .register_box(key, widget, property);
    }

    /// Registers a shared property. Uses the key as source key
    pub fn register_shared_property<P: Component>(
        &mut self,
        key: &str,
        target: Entity,
        source: Entity,
    ) {
        self.register_shared_property_by_source_key::<P>(key, key, target, source);
    }

    /// Registers a shared property.
    pub fn register_shared_property_by_source_key<P: Component>(
        &mut self,
        key: &str,
        source_key: &str,
        target: Entity,
        source: Entity,
    ) {
        self.ecm
            .component_store_mut()
            .register_shared_by_source_key::<P>(key, source_key, target, source);
    }

    /// Registers a shared component box. Uses the key as source key
    pub fn register_property_shared_box(
        &mut self,
        key: &str,
        widget: Entity,
        property: SharedComponentBox,
    ) {
        self.register_property_shared_box_by_source_key(key, key, widget, property);
    }

    /// Registers a shared component box.
    pub fn register_property_shared_box_by_source_key(
        &mut self,
        key: &str,
        source_key: &str,
        widget: Entity,
        property: SharedComponentBox,
    ) {
        self.ecm
            .component_store_mut()
            .register_shared_box_by_source_key(key, source_key, widget, property);
    }

    /// Registers a state with a widget.
    pub fn register_state(&mut self, widget: Entity, state: Box<dyn State>) {
        self.states.insert(widget, state);
    }

    /// Registers a render object with a widget.
    pub fn register_render_object(&mut self, widget: Entity, render_object: Box<dyn RenderObject>) {
        self.render_objects
            .borrow_mut()
            .insert(widget, render_object);
    }

    /// Registers an event handler with a widget.
    pub fn register_handler(&mut self, widget: Entity, handler: Rc<dyn EventHandler>) {
        if !self.handlers.borrow().contains_key(&widget) {
            self.handlers.borrow_mut().insert(widget, vec![]);
        }

        self.handlers
            .borrow_mut()
            .get_mut(&widget)
            .unwrap()
            .push(handler);
    }

    /// Registers a layout object with a widget.
    pub fn register_layout(&mut self, widget: Entity, layout: Box<dyn Layout>) {
        self.layouts.borrow_mut().insert(widget, layout);
    }
}
