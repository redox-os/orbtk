use std::collections::HashMap;

use {Constraint, Entity, EntityComponentManager, LayoutObject, LayoutResult, Theme};

pub struct CenterLayoutObject;

impl LayoutObject for CenterLayoutObject {
    fn layout(
        &self,
        _entity: Entity,
        _ecm: &EntityComponentManager,
        constraint: &Constraint,
        children: &[Entity],
        children_pos: &mut Option<HashMap<Entity, (i32, i32)>>,
        size: Option<(u32, u32)>,
        _theme: &Theme,
    ) -> LayoutResult {
        if let Some(size) = size {
            if let None = children_pos {
                *children_pos = Some(HashMap::new());
            }

            let requested_size = constraint.perform((size.0, size.1));

            if let Some(children_pos) = children_pos {
                children_pos.insert(
                    children[0],
                    (
                        (requested_size.0 - size.0) as i32 / 2,
                        (requested_size.1 - size.1) as i32 / 2,
                    ),
                );
            }

            LayoutResult::Size(requested_size)
        } else {
            if children.len() == 0 {
                return LayoutResult::Size((constraint.max_width, constraint.max_height));
            }
            LayoutResult::RequestChild(children[0], *constraint)
        }
    }
}
