use std::fmt;

mod build_context;
mod entity_builder;

pub use self::build_context::*;
pub use self::entity_builder::*;

pub trait Widget: fmt::Debug {
    fn build(self, btx: &mut BuildContext);
}
