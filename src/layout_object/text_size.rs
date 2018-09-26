use std::collections::HashMap;
use std::sync::Arc;

use {BoxConstraints, Entity, EntityComponentManager, Label, LayoutObject, LayoutResult, Theme};

pub struct TextSizeLayoutObject;

impl LayoutObject for TextSizeLayoutObject {
    fn layout(
        &self,
        entity: Entity,
        ecm: &EntityComponentManager,
        bc: &BoxConstraints,
        _children: &[Entity],
        _children_pos: &mut HashMap<Entity, (i32, i32)>,
        _size: Option<(u32, u32)>,
        _theme: &Arc<Theme>,
    ) -> LayoutResult {
        if let Ok(label) = ecm.borrow_component::<Label>(entity) {
            return LayoutResult::Size(bc.constrain((label.0.len() as u32 * 8 + 2, 18)));
        }

        LayoutResult::Size((bc.min_width, bc.min_height))
    }
}
