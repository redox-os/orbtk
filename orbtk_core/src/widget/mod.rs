use std::{any::Any, fmt, ops::DerefMut};

use legion::*;

mod build_context;
mod node;

pub use self::build_context::*;
pub use self::node::*;

pub trait Widget: fmt::Debug {
    fn build(self) -> Node;
}

impl<W> From<W> for Node
where
    W: Widget,
{
    fn from(widget: W) -> Self {
        widget.build()
    }
}
