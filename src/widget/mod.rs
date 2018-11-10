//! Contains concret implementations of OrbTk's default widgets. It contains also layout widgets.

use std::any::{Any, TypeId};
use std::cell::Cell;
use std::rc::Rc;

use dces::{Component, ComponentBox, Entity, EntityComponentManager, NotFound, SharedComponentBox};

use state::State;
use layout_object::{DefaultLayoutObject, LayoutObject};
use render_object::RenderObject;
use theme::Selector;
use event::EventHandler;
use tree::Tree;

pub use self::button::*;
pub use self::center::*;
pub use self::column::*;
pub use self::container::*;
pub use self::row::*;
pub use self::scroll_viewer::*;
pub use self::stack::*;
pub use self::text_block::*;
pub use self::text_box::*;

mod button;
mod center;
mod column;
mod container;
mod macros;
mod row;
mod scroll_viewer;
mod stack;
mod text_block;
mod text_box;

#[derive(Copy, Clone)]
pub struct Drawable;

// pub struct Key(pub String);

pub struct Padding {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

pub enum Template {
    Empty,
    Single(Rc<Widget>),
    Mutli(Vec<Rc<Widget>>),
}

pub trait Widget: Any {
    fn template(&self) -> Template {
        Template::Empty
    }

    fn all_properties(&self) -> Vec<PropertyResult> {
        let mut properties = self.properties();
        if self.render_object().is_some() {
            properties.push(Property::new(Drawable).build());
        }

        properties
    }

    fn properties(&self) -> Vec<PropertyResult> {
        vec![]
    }

    fn render_object(&self) -> Option<Box<RenderObject>> {
        None
    }

    fn layout_object(&self) -> Box<LayoutObject> {
        Box::new(DefaultLayoutObject)
    }

    fn event_handlers(&self) -> Vec<Rc<EventHandler>> {
        vec![]
    }

    fn state(&self) -> Option<Rc<State>> {
        None
    }
}

pub struct WidgetContainer<'a> {
    tree: &'a Tree,
    ecm: &'a mut EntityComponentManager,
    current_node: Entity,
}

impl<'a> WidgetContainer<'a> {
    pub fn new(root: Entity, ecm: &'a mut EntityComponentManager, tree: &'a Tree) -> Self {
        WidgetContainer {
            tree,
            ecm,
            current_node: root,
        }
    }

    pub fn borrow_property<P: Component>(&self) -> Result<&P, NotFound> {
        self.ecm.borrow_component::<P>(self.current_node)
    }

    pub fn borrow_mut_property<P: Component>(&mut self) -> Result<&mut P, NotFound> {
        self.ecm.borrow_mut_component::<P>(self.current_node)
    }

    pub fn borrow_parent_property<P: Component>(&self) -> Result<&P, NotFound> {
        self.ecm.borrow_component::<P>(self.tree.parent[&self.current_node])
    }

    pub fn borrow_mut_parent_property<P: Component>(&mut self) -> Result<&mut P, NotFound> {
        self.ecm.borrow_mut_component::<P>(self.tree.parent[&self.current_node])
    }

    pub fn borrow_child_property<P: Component>(&self, index: usize) -> Result<&P, NotFound> {
        if index >= self.tree.children[&self.current_node].len() {
            return Result::Err(NotFound::Component(TypeId::of::<P>()))
        }

        self.ecm.borrow_component::<P>(self.tree.children[&self.current_node][index])
    }

    pub fn borrow_mut_child_property<P: Component>(&mut self, index: usize) -> Result<&mut P, NotFound> {
         if index >= self.tree.children[&self.current_node].len() {
            return Result::Err(NotFound::Component(TypeId::of::<P>()))
        }

        self.ecm.borrow_mut_component::<P>(self.tree.children[&self.current_node][index])
    }
}

pub fn add_selector_to_widget(pseudo_class: &str, widget: &mut WidgetContainer) {
    if let Ok(selector) = widget.borrow_mut_property::<Selector>() {
        selector.pseudo_classes.insert(String::from(pseudo_class));
    }
}

pub fn remove_selector_from_widget(pseudo_class: &str, widget: &mut WidgetContainer) {
    if let Ok(selector) = widget.borrow_mut_property::<Selector>() {
        selector.pseudo_classes.remove (pseudo_class);
    }
}

pub enum PropertyResult {
    Property(ComponentBox, Rc<Cell<Option<Entity>>>),
    Source(SharedComponentBox),
    PropertyNotFound,
}

pub struct Property<C>
where
    C: Component + Clone,
{
    pub source: Rc<Cell<Option<Entity>>>,
    property: Option<C>,
}

impl<C> Property<C>
where
    C: Component + Clone,
{
    pub fn new(property: C) -> Self {
        Property {
            source: Rc::new(Cell::new(None)),
            property: Some(property),
        }
    }

    pub fn build(&self) -> PropertyResult {
        if let Some(source) = self.source.get() {
            return PropertyResult::Source(SharedComponentBox::new::<C>(source));
        }

        if let Some(property) = &self.property {
            return PropertyResult::Property(ComponentBox::new(property.clone()), self.source.clone())
        }

        PropertyResult::PropertyNotFound
    }
}

impl<C> Clone for Property<C>
where
    C: Component + Clone,
{
    fn clone(&self) -> Self {
        Property {
            source: self.source.clone(),
            property: None,
        }
    }
}
