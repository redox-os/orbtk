use super::{CloneCell, Rect};

use std::sync::Arc;
use std::any::Any;

pub use self::drawable::*;
pub use self::layouts::*;
pub use self::widgets::*;

mod drawable;
mod layouts;
mod widgets;

pub enum Content {
    Zero,
    Single(Arc<Widget>),
    Multi(Vec<Arc<Widget>>),
}

pub enum WidgetType {
    Empty,
    Drawable(Arc<Drawable>),
    SingleChildLayout(Arc<Fn(&Arc<Widget>)>),
    MultiChildrenLayout(Arc<Fn(Vec<&Arc<Widget>>)>),
    EventHandler(/* todo: event */),
}

pub trait Widget : Any {
    fn types(&self) -> Vec<WidgetType> {
        vec![WidgetType::Empty]
    }
    fn build(&self) -> Content {
        Content::Zero
    }
}