use dces::{Entity, EntityComponentManager};
use layout_object::{LayoutObject, LayoutResult};
use structs::Constraint;
use theme::Theme;
use widget::Offset;

pub struct ScrollLayoutObject;

impl LayoutObject for ScrollLayoutObject {
    fn layout(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        constraint: &Constraint,
        children: &[Entity],
        size: Option<(u32, u32)>,
        _theme: &Theme,
    ) -> LayoutResult {
        if let Some(size) = size {
            if let Ok(offset) = ecm.borrow_mut_component::<Offset>(entity) {
                offset.0 = (constraint.max_width as i32 - size.0 as i32).min(0);
                offset.1 = (constraint.max_height as i32 - size.1 as i32).min(0);
            }

            LayoutResult::Size((constraint.max_width, constraint.max_height))
        } else {
            if children.is_empty() {
                return LayoutResult::Size((constraint.max_width, constraint.max_height));
            }
            LayoutResult::RequestChild(children[0], *constraint)
        }
    }
}
