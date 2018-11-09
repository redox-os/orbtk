use dces::{Entity, EntityComponentManager};
use layout_object::{LayoutObject, LayoutResult};
use structs::Constraint;
use theme::Theme;

pub struct DefaultLayoutObject;

impl LayoutObject for DefaultLayoutObject {
    fn layout(
        &self,
        _entity: Entity,
        _ecm: &mut EntityComponentManager,
        constraint: &Constraint,
        children: &[Entity],
        size: Option<(u32, u32)>,
        _theme: &Theme,
    ) -> LayoutResult {
        if let Some(size) = size {
            LayoutResult::Size(size)
        } else {
            if children.len() == 0 {
                return LayoutResult::Size((constraint.min_width, constraint.min_height));
            }
            LayoutResult::RequestChild(children[0], *constraint)
        }
    }
}
