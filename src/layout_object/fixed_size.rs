use std::cell::Cell;

use dces::{Entity, EntityComponentManager};

use crate::{
    layout_object::{LayoutObject, LayoutResult},
    properties::Constraint,
    theme::{Selector, Theme},
};

#[derive(Default)]
pub struct FixedSizeLayoutObject {
    width: Cell<u32>,
    height: Cell<u32>,
}

impl Into<Box<dyn LayoutObject>> for FixedSizeLayoutObject {
    fn into(self) -> Box<dyn LayoutObject> {
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
        theme: &Theme,
    ) -> LayoutResult {
        if let Some(_size) = size {
            LayoutResult::Size((self.width.get(), self.height.get()))
        } else {
            if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
                self.width.set(theme.uint("width", selector) as u32);
                self.height.set(theme.uint("height", selector) as u32);
            }

            if children.is_empty() {
                return LayoutResult::Size((self.width.get(), self.height.get()));
            }

            LayoutResult::RequestChild(children[0], *constraint)
        }
    }
}
