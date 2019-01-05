use application::Global;
use dces::{Entity, EntityComponentManager};
use layout_object::{LayoutObject, LayoutResult};
use properties::Constraint;
use theme::Selector;

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
                    if let Ok(global) = ecm.borrow_component::<Global>(0) {
                        child_constraint
                            .with_min_width(global.theme.uint("min-width", selector) as i32)
                            .with_max_width(global.theme.uint("max-width", selector) as i32)
                            .with_width(global.theme.uint("width", selector) as i32)
                            .with_min_height(global.theme.uint("min_height", selector) as i32)
                            .with_max_height(global.theme.uint("max_height", selector) as i32)
                            .with_height(global.theme.uint("height", selector) as i32)
                    } else {
                        child_constraint
                    }
                } else {
                    child_constraint
                }
            };

            LayoutResult::RequestChild(children[0], child_constraint)
        }
    }
}
