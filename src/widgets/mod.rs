use std::sync::Arc;
use std::any::Any;

use super::{Drawable, Node};

pub use self::button::Button;
pub use self::text::Text;

mod button;
mod text;

pub enum Content {
    Zero,
    Single(Arc<Widget>),
    Multi(Vec<Arc<Widget>>),
}

pub enum WidgetType {
    Empty,
    Drawable(Arc<Drawable>),
    SingleChildLayout(Arc<Fn(&Arc<Node>)>),
    MultiChildrenLayout(Arc<Fn(Vec<&Arc<Node>>)>),
    EventHandler(/* todo: event */),
}

pub trait Widget : Any {
    fn types(&self) -> Vec<WidgetType> {
        vec![WidgetType::Empty]
    }
    fn build(&self) -> Content {
        Content::Zero
    }
    fn element(&self) -> &str;
}