use {Constraint, Entity, EntityComponentManager, LayoutObject, LayoutResult, Theme};

use structs::Rect;

pub struct CenterLayoutObject;

impl Into<Box<LayoutObject>> for CenterLayoutObject {
    fn into(self) -> Box<LayoutObject> {
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
            let requested_size = constraint.perform((size.0, size.1));

            let width = {
                if constraint.width > 0 {
                    constraint.max_width
                } else {
                    size.0
                }
            };

            let height = {
                if constraint.height > 0 {
                    constraint.max_height
                } else {
                    size.1
                }
            };

            if let Ok(bounds) = ecm.borrow_mut_component::<Rect>(children[0]) {
                bounds.x = (width - requested_size.0) as i32 / 2;
                bounds.y = (height - requested_size.1) as i32 / 2;
            }

            LayoutResult::Size((width, height))
        } else {
            if children.is_empty() {
                return LayoutResult::Size((constraint.min_width, constraint.min_height));
            }
            LayoutResult::RequestChild(children[0], *constraint)
        }
    }
}
