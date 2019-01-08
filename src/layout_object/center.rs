use crate::{
    properties::Bounds, Constraint, Entity, EntityComponentManager, LayoutObject, LayoutResult,
    Theme,
};

pub struct CenterLayoutObject;

impl Into<Box<dyn LayoutObject>> for CenterLayoutObject {
    fn into(self) -> Box<dyn LayoutObject> {
        Box::new(self)
    }
}

impl LayoutObject for CenterLayoutObject {
    fn layout(
        &self,
        _entity: Entity,
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

            if let Ok(bounds) = ecm.borrow_mut_component::<Bounds>(children[0]) {
                bounds.x = (center_size.0 - size.0) as i32 / 2;
                bounds.y = (center_size.1 - size.1) as i32 / 2;
            }

            LayoutResult::Size(center_size)
        } else {
            if children.is_empty() {
                return LayoutResult::Size((constraint.max_width, constraint.max_height));
            }

            LayoutResult::RequestChild(children[0], *constraint)
        }
    }
}
