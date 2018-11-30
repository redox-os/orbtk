use dces::{Entity, EntityComponentManager};
use layout_object::{LayoutObject, LayoutResult};
use properties::Constraint;
use theme::{Selector, Theme};

pub struct RootLayoutObject;

impl Into<Box<LayoutObject>> for RootLayoutObject {
    fn into(self) -> Box<LayoutObject> {
        Box::new(self)
    }
}

impl LayoutObject for RootLayoutObject {
    fn layout(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        constraint: &Constraint,
        children: &[Entity],
        size: Option<(u32, u32)>,
        theme: &Theme,
    ) -> LayoutResult {
        if let Some(size) = size {
            LayoutResult::Size(size)
        } else {
            if children.is_empty() {
                return LayoutResult::Size((constraint.min_width, constraint.min_height));
            }

            let child_constraint = {
                let child_constraint = Constraint::default();

                if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
                    child_constraint
                        .with_min_width(theme.uint("min-width", selector) as i32)
                        .with_max_width(theme.uint("max-width", selector) as i32)
                        .with_width(theme.uint("width", selector) as i32)
                        .with_min_height(theme.uint("min_height", selector) as i32)
                        .with_max_height(theme.uint("max_height", selector) as i32)
                        .with_height(theme.uint("height", selector) as i32)
                } else {
                    child_constraint
                }
            };

            LayoutResult::RequestChild(children[0], child_constraint)
        }
    }
}
