use dces::{Entity, EntityComponentManager};
use layout_object::{LayoutObject, LayoutResult};
use properties::{Constraint, Offset};
use theme::Theme;

pub struct ScrollLayoutObject;

impl Into<Box<LayoutObject>> for ScrollLayoutObject {
    fn into(self) -> Box<LayoutObject> {
        Box::new(self)
    }
}

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
             let width = {
                if constraint.width > 0 {
                    constraint.width
                } else {
                    size.0
                }
            };

            let height = {
                if constraint.height > 0 {
                    constraint.height
                } else {
                    size.1
                }
            };

            let center_size = constraint.perform((width, height));

            if let Ok(offset) = ecm.borrow_mut_component::<Offset>(entity) {
                offset.0 = (center_size.0 as i32 - size.0 as i32).min(0);
                offset.1 = (center_size.1 as i32 - size.1 as i32).min(0);
            }

            LayoutResult::Size(constraint.perform(center_size))
        } else {
            if children.is_empty() {
                return LayoutResult::Size((constraint.max_width, constraint.max_height));
            }
            LayoutResult::RequestChild(children[0], *constraint)
        }
    }
}
