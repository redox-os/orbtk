use std::cell::Cell;

use dces::{Entity, EntityComponentManager};

use application::Global;
use layout_object::{LayoutObject, LayoutResult};
use properties::Constraint;
use theme::Selector;

#[derive(Default)]
pub struct FixedSizeLayoutObject {
    width: Cell<u32>,
    height: Cell<u32>,
}

impl Into<Box<LayoutObject>> for FixedSizeLayoutObject {
    fn into(self) -> Box<LayoutObject> {
        Box::new(self)
    }
}

impl LayoutObject for FixedSizeLayoutObject {
    fn layout(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        constraint: &Constraint,
        children: &[Entity],
        size: Option<(u32, u32)>,
    ) -> LayoutResult {
        if let Some(_size) = size {
            LayoutResult::Size((self.width.get(), self.height.get()))
        } else {
            if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
                if let Ok(global) = ecm.borrow_component::<Global>(0) {
                    self.width.set(global.theme.uint("width", selector) as u32);
                    self.height
                        .set(global.theme.uint("height", selector) as u32);
                }
            }

            if children.is_empty() {
                return LayoutResult::Size((self.width.get(), self.height.get()));
            }

            LayoutResult::RequestChild(children[0], *constraint)
        }
    }
}
