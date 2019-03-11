use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Component, ComponentBox, Entity, SharedComponentBox, World};

use crate::application::Tree;

use crate::theme::Selector;

use crate::event::EventHandler;

use crate::{Layout, RenderObject};

pub use self::context::Context;
pub use self::message::{MessageBox, StringMessage};
pub use self::property::{
    get_property, Property, PropertyResult, PropertySource, WipProperty, WipPropertyBuilder,
};
pub use self::state::State;
pub use self::template::{Template, TemplateBase};
pub use self::widget_container::WidgetContainer;

mod context;
mod message;
mod property;
mod state;
mod template;
mod widget_container;

/// The `Widget` trait is used to define a new widget.
pub trait Widget {
    /// Returns a new widget.
    fn create() -> Self;
}

/// Adds the given `pseudo_class` to the css selector of the given `widget`.
pub fn add_selector_to_widget(pseudo_class: &str, widget: &mut WidgetContainer<'_>) {
    if let Ok(selector) = widget.borrow_mut_property::<Selector>() {
        selector.pseudo_classes.insert(String::from(pseudo_class));
        selector.set_dirty(true);
    }
}

/// Removes the given `pseudo_class` from the css selector of the given `widget`.
pub fn remove_selector_from_widget(pseudo_class: &str, widget: &mut WidgetContainer<'_>) {
    if let Ok(selector) = widget.borrow_mut_property::<Selector>() {
        selector.pseudo_classes.remove(pseudo_class);
        selector.set_dirty(true);
    }
}

pub struct WipTemplate {
    id: Entity,
    children: Vec<WipTemplate>,
    // todo parent type
}

impl WipTemplate {
    pub fn new(id: Entity) -> Self {
        WipTemplate {
            id,
            children: vec![],
        }
    }
}

/// The `TemplateBuilder` trait provides the method for the widget template creation.
pub trait WipTemplateBuilder: Sized {
    /// Creates the template of the widget and returns it.
    fn template(self, _id: Entity, _context: &mut WipBuildContext) {}
}

/// The `Widget` trait is used to define a new widget.
pub trait WipWidget: WipTemplateBuilder {
    /// Creates a new widget.
    fn create() -> Self;

    /// Builds the widget and returns the template of the widget.
    fn build(self, context: &mut WipBuildContext) -> Entity;

    /// Inerts a new event handler.
    fn insert_handler(self, handler: impl Into<Rc<dyn EventHandler>>) -> Self;

    fn child(self, child: Entity) -> Self;
}

pub struct WipBuildContext<'a> {
    root: Entity,
    world: &'a mut World<Tree>,
    render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
    states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
}

impl<'a> WipBuildContext<'a> {
    pub fn new(
        root: Entity,
        world: &'a mut World<Tree>,
        render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
        layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
        states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    ) -> Self {
        WipBuildContext { root, world, render_objects, layouts, handlers, states }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.world.create_entity().build()
    }

    pub fn append_child(&mut self, parent: Entity, child: Entity) {
        self.world.entity_container().append_child(parent, child).unwrap();
    }

    pub fn register_property<P: Component>(&mut self, entity: Entity, property: P) {
        self.world
            .entity_component_manager()
            .register_component(entity, property);
    }

    pub fn register_property_box(&mut self, entity: Entity, property: ComponentBox) {
        self.world
            .entity_component_manager()
            .register_component_box(entity, property);
    }

    pub fn register_property_shared_box(&mut self, entity: Entity, property: SharedComponentBox) {
        self.world
            .entity_component_manager()
            .register_shared_component_box(entity, property);
    }

    pub fn register_shared_property<P: Component>(&mut self, target: Entity, source: Entity) {
        self.world
            .entity_component_manager()
            .register_shared_component::<P>(target, source);
    }
}
