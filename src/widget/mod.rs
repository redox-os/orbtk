use state::State;
use std::any::Any;
use std::rc::Rc;

use dces::{Component, NotFound, Entity, EntityComponentManager};

use super::Property;
use render_object::RenderObject;
use layout_object::{DefaultLayoutObject, LayoutObject};
use tree::Tree;

pub use self::button::*;
pub use self::column::*;
pub use self::container::*;
pub use self::row::*;
pub use self::text_block::*;
pub use self::text_box::*;

mod button;
mod column;
mod container;
mod row;
mod text_block;
mod text_box;

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

    fn all_properties(&self) -> Vec<Property> {
        let mut properties = self.properties();
        if let Some(_) = self.render_object() {
            properties.push(Property::new(Drawable));
        }

        if let Some(state) = self.state() {
            properties.append(&mut state.properties());
        }
        properties
    }

    fn properties(&self) -> Vec<Property> {
        vec![]
    }

    fn render_object(&self) -> Option<Box<RenderObject>> {
        None
    }

    fn layout_object(&self) -> Box<LayoutObject> {
        Box::new(DefaultLayoutObject)
    }

    fn state(&self) -> Option<Rc<State>> {
        None
    }
}

pub struct WidgetWrapper<'a> {
    entity: Entity,
    ecm: &'a mut EntityComponentManager,
    _tree: &'a Tree,
    _current_child: Entity,
}

impl<'a> WidgetWrapper<'a> {
    pub fn new(entity: Entity, ecm: &'a mut EntityComponentManager, _tree: &'a Tree) -> Self {
        WidgetWrapper {
            entity, 
            ecm,
            _tree,
            _current_child: entity,
        }
    }

    pub fn borrow_property<P: Component>(&self) -> Result<&P, NotFound> {
        self.ecm.borrow_component::<P>(self.entity)
    }

    pub fn borrow_mut_property<P: Component>(&mut self) -> Result<&mut P, NotFound> {
        self.ecm.borrow_mut_component::<P>(self.entity)
    }

    pub fn next(&mut self) {
        // todo: set to child
    }

    pub fn reset(&mut self) {
        // todo: reset to root
    }
}