use std::collections::HashMap;

use {Constraint, Entity, EntityComponentManager, LayoutObject, LayoutResult, Theme};

pub struct StretchLayoutObject;

impl LayoutObject for StretchLayoutObject {
    fn layout(
        &self,
        _entity: Entity,
        _ecm: &EntityComponentManager,
        constraint: &Constraint,
        children: &[Entity],
        _children_pos: &mut Option<HashMap<Entity, (i32, i32)>>,
        size: Option<(u32, u32)>,
        _theme: &Theme,
    ) -> LayoutResult {
        if let Some(_size) = size {
            LayoutResult::Size((constraint.max_width, constraint.max_height))
        } else {
            if children.len() == 0 {
                return LayoutResult::Size((constraint.max_width, constraint.max_height));
            }
            LayoutResult::RequestChild(children[0], *constraint)
        }
    }
}
