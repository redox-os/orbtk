use std::any::Any;
use std::sync::Arc;

use Property;

pub use self::button::*;
pub use self::container::*;
pub use self::text_block::*;

mod button;
mod container;
mod text_block;

pub struct Padding {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32
}

pub enum Template {
    Empty,
    Single(Arc<Widget>),
    Mutli(Vec<Arc<Widget>>),
}

pub trait Widget: Any {
    fn template(&self) -> Template {
        Template::Empty
    }
    fn properties(&self) -> Vec<Property> {
        vec![]
    }
}
