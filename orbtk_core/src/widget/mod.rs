use std::fmt;

use legion::*;

mod build_context;
mod entity_builder;

pub use self::build_context::*;
pub use self::entity_builder::*;

pub trait Widget: fmt::Debug {
    fn build(self, btx: &mut BuildContext);
}

impl<W> From<W> for BuildContext
where
    W: Widget,
{
    fn from(widget: W) -> Self {
        let mut btx = BuildContext::new();
        widget.build(&mut btx);
        btx
    }
}
