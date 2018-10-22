use std::collections::HashMap;

use {Constraint, Entity, EntityComponentManager, Label, LayoutObject, LayoutResult, Theme};

pub struct TextSizeLayoutObject;

impl LayoutObject for TextSizeLayoutObject {
    fn layout(
        &self,
        entity: Entity,
        ecm: &EntityComponentManager,
        constraint: &Constraint,
        _children: &[Entity],
        _children_pos: &mut Option<HashMap<Entity, (i32, i32)>>,
        _size: Option<(u32, u32)>,
        _theme: &Theme,
    ) -> LayoutResult {
        if let Ok(label) = ecm.borrow_component::<Label>(entity) {
            return LayoutResult::Size((label.0.len() as u32 * 8 + 2, 18));
        }

        LayoutResult::Size((constraint.min_width, constraint.min_height))
    }
}
