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

            if let Ok(bounds) = ecm.borrow_mut_component::<Rect>(children[0]) {
                bounds.x = (requested_size.0 - size.0) as i32 / 2;
                bounds.y = (requested_size.1 - size.1) as i32 / 2;
            }

            LayoutResult::Size(requested_size)
        } else {
            if children.is_empty() {
                return LayoutResult::Size((constraint.max_width, constraint.max_height));
            }
            LayoutResult::RequestChild(children[0], *constraint)
        }
    }
}
