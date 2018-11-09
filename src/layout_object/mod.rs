use std::collections::HashMap;

use dces::{Entity, EntityComponentManager};

use theme::Theme;
use structs::Constraint;
use systems::LayoutResult;

pub use self::center::*;
pub use self::default::*;
pub use self::flex::*;
pub use self::padding::*;
pub use self::stretch::*;
pub use self::text_size::*;

mod center;
mod default;
mod flex;
mod padding;
mod stretch;
mod text_size;

pub trait LayoutObject {
    fn layout(
        &self,
        entity: Entity,
        ecm: &EntityComponentManager,
        constraint: &Constraint,
        children: &[Entity],
        children_pos: &mut Option<HashMap<Entity, (i32, i32)>>,
        size: Option<(u32, u32)>,
        theme: &Theme,
    ) -> LayoutResult;
}
