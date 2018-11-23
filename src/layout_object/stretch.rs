use dces::{Entity, EntityComponentManager};
use layout_object::{LayoutObject, LayoutResult};
use structs::Constraint;
use theme::Theme;

pub struct StretchLayoutObject;

impl Into<Box<LayoutObject>> for StretchLayoutObject {
    fn into(self) -> Box<LayoutObject> {
        Box::new(self)
    }
}

impl LayoutObject for StretchLayoutObject {
    fn layout(
        &self,
        _entity: Entity,
        _ecm: &mut EntityComponentManager,
        constraint: &Constraint,
        children: &[Entity],
        size: Option<(u32, u32)>,
        _theme: &Theme,
    ) -> LayoutResult {
        if let Some(_size) = size {
            LayoutResult::Size((constraint.max_width, constraint.max_height))
        } else {
            if children.is_empty() {
                return LayoutResult::Size((constraint.max_width, constraint.max_height));
            }
            LayoutResult::RequestChild(children[0], *constraint)
        }
    }
}
