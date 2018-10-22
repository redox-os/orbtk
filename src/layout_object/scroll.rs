use std::collections::HashMap;

use dces::{Entity, EntityComponentManager};

use layout_object::LayoutObject;
use structs::Constraint;
use systems::LayoutResult;
use theme::Theme;
use widget::{HorizontalOffset, VerticalOffset};

pub struct ScrollLayoutObject;

impl LayoutObject for ScrollLayoutObject {
    fn layout(
        &self,
        entity: Entity,
        ecm: &EntityComponentManager,
        constraint: &Constraint,
        children: &[Entity],
        children_pos: &mut Option<HashMap<Entity, (i32, i32)>>,
        size: Option<(u32, u32)>,
        _theme: &Theme,
    ) -> LayoutResult {
        if let Some(_size) = size {
            let vertical_offset = {
                if let Ok(offset) = ecm.borrow_component::<VerticalOffset>(entity) {
                    offset.0
                } else {
                    0
                }
            };

            let horizonal_offset = {
                if let Ok(offset) = ecm.borrow_component::<HorizontalOffset>(entity) {
                    offset.0
                } else {
                    0
                }
            };

            if let None = children_pos {
                *children_pos = Some(HashMap::new());
            }
            if let Some(children_pos) = children_pos {
                children_pos.insert(children[0], (horizonal_offset, vertical_offset));
            }

            LayoutResult::Size((constraint.min_width, constraint.min_height))
        } else {
            if children.len() == 0 {
                return LayoutResult::Size((constraint.min_width, constraint.min_height));
            }
            LayoutResult::RequestChild(children[0], *constraint)
        }
    }
}
