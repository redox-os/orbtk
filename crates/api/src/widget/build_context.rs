use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Component, ComponentBox, Entity, EntityComponentManager, SharedComponentBox};

use crate::{prelude::*, tree::Tree};

use super::State;

/// Used to create an entity for a widget with its properties as components.
pub struct BuildContext<'a> {
    ecm: &'a mut EntityComponentManager<Tree, StringComponentStore>,
    render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
}

impl<'a> BuildContext<'a> {
    /// Creates a new `BuildContext`.
    pub fn new(
        ecm: &'a mut EntityComponentManager<Tree, StringComponentStore>,
        render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
        layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
        states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    ) -> Self {
        BuildContext {
            ecm,
            render_objects,
            layouts,
            handlers,
            states,
        }
    }

    /// Creates a new entity.
    pub fn create_entity(&mut self) -> Entity {
        self.ecm.create_entity().build()
    }

    /// Appends a child to a parent.
    pub fn append_child(&mut self, parent: Entity, child: Entity) {
        self.ecm
            .entity_store_mut()
            .append_child(parent, child)
            .unwrap();
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
    pub fn register_state(&mut self, widget: Entity, state: Rc<dyn State>) {
        self.states.borrow_mut().insert(widget, state);
    }

    /// Registers a render object with a widget.
    pub fn register_render_object(&self, widget: Entity, render_object: Box<dyn RenderObject>) {
        self.render_objects
            .borrow_mut()
            .insert(widget, render_object);
    }

    /// Registers a event handler with a widget.
    pub fn register_handler(&self, widget: Entity, handler: Rc<dyn EventHandler>) {
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
    pub fn register_layout(&self, widget: Entity, layout: Box<dyn Layout>) {
        self.layouts.borrow_mut().insert(widget, layout);
    }
}
