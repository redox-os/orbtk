use std::sync::Arc;
use std::any::Any;
use std::fmt;

use super::{Constraint, Drawable};
use theme::Selector;

pub use self::button::Button;
pub use self::label::Label;

mod button;
mod label;

pub trait Property: Any + Sized {}

impl<T: Any> Property for T {}

pub enum Content {
    None,
    Single(Arc<Widget>),
    Multi(Vec<Arc<Widget>>),
}

pub enum WidgetType {
    Empty,
    Drawable(Arc<Drawable>),
    Layout(Arc<Fn(&Arc<Widget>, &Option<Constraint>) -> Constraint>),
    EventHandler(/* todo: event */),
    Styleable(Selector),
    FixedSized { width: u32, height: u32 },
}

pub trait Widget: Any {
    fn types(&self) -> Vec<WidgetType> {
        vec![WidgetType::Empty]
    }
    
    fn build(&self) -> Content {
        Content::None
    }
    fn element(&self) -> &str;
}

impl fmt::Debug for Widget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Widget {}", self.element())
    }
}
