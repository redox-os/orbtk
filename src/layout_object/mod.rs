use std::collections::HashMap;
use std::sync::Arc;

use {Entity, EntityComponentManager, Theme, BoxConstraints, LayoutResult};

pub use self::default::*;
pub use self::padding::*;
pub use self::text_size::*;

mod default;
mod padding;
mod text_size;

pub trait LayoutObject {
    fn layout(
        &self,
        entity: Entity,
        ecm: &EntityComponentManager,
        bc: &BoxConstraints,
        children: &[Entity],
        children_pos: &mut HashMap<Entity, (i32, i32)>,
        size: Option<(u32, u32)>,
        theme: &Arc<Theme>,
    ) -> LayoutResult;
}
