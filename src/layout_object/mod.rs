//! This module contains all layout objects used in OrbTk. Layout objects are used to define the layout of a widget, how
//! to place and order its children.

use dces::{Entity, EntityComponentManager};

use structs::Constraint;
use systems::LayoutResult;
use theme::Theme;

pub use self::center::*;
pub use self::default::*;
pub use self::flex::*;
pub use self::padding::*;
pub use self::scroll::*;
pub use self::stretch::*;
pub use self::text_size::*;

mod center;
mod default;
mod flex;
mod padding;
mod scroll;
mod stretch;
mod text_size;

pub trait LayoutObject {
    fn layout(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        constraint: &Constraint,
        children: &[Entity],
        size: Option<(u32, u32)>,
        theme: &Theme,
    ) -> LayoutResult;
}
