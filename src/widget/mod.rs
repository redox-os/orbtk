use std::any::Any;
use std::sync::Arc;

use {Property, RenderObject, LayoutObject, DefaultLayoutObject};

pub use self::button::*;
pub use self::column::*;
pub use self::container::*;
pub use self::row::*;
pub use self::text_block::*;

mod button;
mod column;
mod container;
mod row;
mod text_block;

pub struct Drawable;

pub struct Padding {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

pub enum Template {
    Empty,
    Single(Arc<Widget>),
    Mutli(Vec<Arc<Widget>>),
}

pub trait Widget: Any + Send + Sync {
    fn template(&self) -> Template {
        Template::Empty
    }

    fn all_properties(&self) -> Vec<Property> {
        let mut properties = self.properties();
        if let Some(_) = self.render_object() {
            properties.push(Property::new(Drawable));
        }
        properties
    }

    fn properties(&self) -> Vec<Property> {
        vec![]
    }
    fn render_object(&self) -> Option<Arc<RenderObject>> {
        None
    }
    fn layout_object(&self) -> Box<LayoutObject> {
        Box::new(DefaultLayoutObject)
    }
}
