use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Component, ComponentBox, Entity, SharedComponentBox, World};

use crate::application::Tree;

use crate::theme::Selector;

use crate::event::EventHandler;

use crate::{Layout, RenderObject, PaddingLayout};

pub use self::context::Context;
pub use self::message::{MessageBox, StringMessage};
pub use self::property::{
    get_property, Property, PropertyResult, PropertySource,
};
pub use self::state::State;
pub use self::widget_container::WidgetContainer;

mod context;
mod message;
mod property;
mod state;
mod template;
mod widget_container;

/// Adds the given `pseudo_class` to the css selector of the given `widget`.
pub fn add_selector_to_widget(pseudo_class: &str, widget: &mut WidgetContainer<'_>) {
    if let Ok(selector) = widget.borrow_mut_property::<Selector>() {
        selector.0.pseudo_classes.insert(String::from(pseudo_class));
        selector.0.set_dirty(true);
    }
}

/// Removes the given `pseudo_class` from the css selector of the given `widget`.
pub fn remove_selector_from_widget(pseudo_class: &str, widget: &mut WidgetContainer<'_>) {
    if let Ok(selector) = widget.borrow_mut_property::<Selector>() {
        selector.0.pseudo_classes.remove(pseudo_class);
        selector.0.set_dirty(true);
    }
}

/// The `Template` trait provides the method for the widget template creation.
pub trait Template: Sized {
    /// Creates the template of the widget and returns it.
    fn template(self, _id: Entity, _context: &mut BuildContext) -> Self {
        self
    }

    fn render_object(&self) -> Option<Box<dyn RenderObject>> {
        None
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(PaddingLayout::new())
    }
}

/// The `Widget` trait is used to define a new widget.
pub trait Widget: Template {
    /// Creates a new widget.
    fn create() -> Self;

    /// Builds the widget and returns the template of the widget.
    fn build(self, context: &mut BuildContext) -> Entity;

    /// Inerts a new event handler.
    fn insert_handler(self, handler: impl Into<Rc<dyn EventHandler>>) -> Self;

   

    fn state(self) -> Option<Rc<State>> {
        None
    }

    fn child(self, child: Entity) -> Self;
}

use std::any::TypeId;
use std::collections::HashMap;

#[derive(Default)]
pub struct LayoutStorage {
    layouts: HashMap<TypeId, Box<dyn Layout>>,
    map: HashMap<Entity, TypeId>,
}

impl LayoutStorage {
    pub fn new() -> Self {
        LayoutStorage::default()
    }

    pub fn register_layout<L: Layout + Default>(&mut self, widget: Entity) {
        let type_id = TypeId::of::<L>();

        if !self.layouts.contains_key(&type_id) {
            self.layouts.insert(type_id, Box::new(L::default()));
        }

        self.map.insert(widget, type_id);
    }

    pub fn layout(&self, widget: Entity) -> Option<&Box<Layout>> {
        if let Some(type_id) = self.map.get(&widget) {
            return self.layouts.get(type_id);
        }

        None
    }
}

#[derive(Default)]
pub struct RenderObjectStorage {
    render_objects: HashMap<TypeId, Box<dyn RenderObject>>,
    map: HashMap<Entity, TypeId>,
}

impl RenderObjectStorage {
    pub fn new() -> Self {
        RenderObjectStorage::default()
    }

    pub fn register_render_object<R: RenderObject + Default>(&mut self, widget: Entity) {
        let type_id = TypeId::of::<R>();

        if !self.render_objects.contains_key(&type_id) {
            self.render_objects.insert(type_id, Box::new(R::default()));
        }

        self.map.insert(widget, type_id);
    }

    pub fn render_object(&self, widget: Entity) -> Option<&Box<RenderObject>> {
        if let Some(type_id) = self.map.get(&widget) {
            return self.render_objects.get(type_id);
        }

        None
    }
}

pub struct BuildContext<'a> {
    world: &'a mut World<Tree>,
    render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
}

impl<'a> BuildContext<'a> {
    pub fn new(
        world: &'a mut World<Tree>,
        render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
        layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
        states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    ) -> Self {
        BuildContext {
            world,
            render_objects,
            layouts,
            handlers,
            states,
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.world.create_entity().build()
    }

    pub fn append_child(&mut self, parent: Entity, child: Entity) {
        self.world
            .entity_container()
            .append_child(parent, child)
            .unwrap();
    }

    pub fn register_property<P: Component>(&mut self, widget: Entity, property: P) {
        self.world
            .entity_component_manager()
            .register_component(widget, property);
    }

    pub fn register_property_box(&mut self, widget: Entity, property: ComponentBox) {
        self.world
            .entity_component_manager()
            .register_component_box(widget, property);
    }

    pub fn register_property_shared_box(&mut self, widget: Entity, property: SharedComponentBox) {
        self.world
            .entity_component_manager()
            .register_shared_component_box(widget, property);
    }

    pub fn register_shared_property<P: Component>(&mut self, target: Entity, source: Entity) {
        self.world
            .entity_component_manager()
            .register_shared_component::<P>(target, source);
    }

    pub fn register_state(&self, widget: Entity, state: Rc<State>) {
        self.states.borrow_mut().insert(widget, state);
    }

    pub fn register_render_object(&self, widget: Entity, render_object: Box<dyn RenderObject>) {
        self.render_objects
            .borrow_mut()
            .insert(widget, render_object);
    }

    pub fn register_layout(&self, widget: Entity, layout: Box<dyn Layout>) {
        self.layouts.borrow_mut().insert(widget, layout);
    }
}

// todo: improvement use only one layout and one render object of the same type.
