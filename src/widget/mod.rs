//! Contains concret implementations of OrbTk's default widgets. It contains also layout widgets.

use std::any::{Any, TypeId};
use std::cell::Cell;
use std::rc::Rc;

use dces::{Component, ComponentBox, Entity, EntityComponentManager, NotFound, SharedComponentBox};

use event::EventHandler;
use layout_object::{DefaultLayoutObject, LayoutObject};
use render_object::RenderObject;
use state::State;
use theme::Selector;
use tree::Tree;
use application::Template;

// pub use self::button::*;
// pub use self::center::*;
// pub use self::column::*;
pub use self::container::*;
// pub use self::row::*;
// pub use self::scroll_viewer::*;
// pub use self::stack::*;
pub use self::text_block::*;
// pub use self::text_box::*;

// mod button;
// mod center;
// mod column;
mod container;
// mod macros;
// mod row;
// mod scroll_viewer;
// mod stack;
mod text_block;
//mod text_box;

#[derive(Copy, Clone)]
pub struct Drawable;

/// `Offset` is used to move an widget along the x- and y-axis.
#[derive(Default, Clone, Copy)]
pub struct Offset(pub i32, pub i32);

/// The `Label` struct represents a string used for text drawing.
#[derive(Clone)]
pub struct Label(pub String);

impl From<&str> for Label {
    fn from (s: &str) -> Label {
        Label(s.to_string())
    }
}

impl From<String> for Label {
    fn from (s: String) -> Label {
        Label(s)
    }
}



// pub struct Key(pub String);

pub struct Padding {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
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
        self.ecm
            .borrow_component::<P>(self.tree.parent[&self.current_node])
    }

    pub fn borrow_mut_parent_property<P: Component>(&mut self) -> Result<&mut P, NotFound> {
        self.ecm
            .borrow_mut_component::<P>(self.tree.parent[&self.current_node])
    }

    pub fn borrow_child_property<P: Component>(&self, index: usize) -> Result<&P, NotFound> {
        if index >= self.tree.children[&self.current_node].len() {
            return Result::Err(NotFound::Component(TypeId::of::<P>()));
        }

        self.ecm
            .borrow_component::<P>(self.tree.children[&self.current_node][index])
    }

    pub fn borrow_mut_child_property<P: Component>(
        &mut self,
        index: usize,
    ) -> Result<&mut P, NotFound> {
        if index >= self.tree.children[&self.current_node].len() {
            return Result::Err(NotFound::Component(TypeId::of::<P>()));
        }

        self.ecm
            .borrow_mut_component::<P>(self.tree.children[&self.current_node][index])
    }
}

pub fn add_selector_to_widget(pseudo_class: &str, widget: &mut WidgetContainer) {
    if let Ok(selector) = widget.borrow_mut_property::<Selector>() {
        selector.pseudo_classes.insert(String::from(pseudo_class));
    }
}

pub fn remove_selector_from_widget(pseudo_class: &str, widget: &mut WidgetContainer) {
    if let Ok(selector) = widget.borrow_mut_property::<Selector>() {
        selector.pseudo_classes.remove(pseudo_class);
    }
}

// pub enum PropertyResult {
//     Property(ComponentBox, Rc<Cell<Option<Entity>>>),
//     Source(SharedComponentBox),
//     PropertyNotFound,
// }

// pub struct Property<C>
// where
//     C: Component + Clone,
// {
//     pub source: Rc<Cell<Option<Entity>>>,
//     property: Option<C>,
// }

// impl<C> Property<C>
// where
//     C: Component + Clone,
// {
//     pub fn new(property: C) -> Self {
//         Property {
//             source: Rc::new(Cell::new(None)),
//             property: Some(property),
//         }
//     }

//     pub fn build(&self) -> PropertyResult {
//         if let Some(source) = self.source.get() {
//             return PropertyResult::Source(SharedComponentBox::new::<C>(source));
//         }

//         if let Some(property) = &self.property {
//             return PropertyResult::Property(
//                 ComponentBox::new(property.clone()),
//                 self.source.clone(),
//             );
//         }

//         PropertyResult::PropertyNotFound
//     }
// }

pub struct Property {

}



pub trait Widget {
    fn template() -> Template;
}
