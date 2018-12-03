use std::cell::Cell;

use dces::{Entity, EntityComponentManager};
use layout_object::{LayoutObject, LayoutResult};
use properties::Constraint;
use theme::{Selector, Theme};

#[derive(Default)]
pub struct StretchLayoutObject {
    width: Cell<u32>,
    height: Cell<u32>,
    current_child: Cell<usize>,
}

impl Into<Box<LayoutObject>> for StretchLayoutObject {
    fn into(self) -> Box<LayoutObject> {
        Box::new(self)
    }
}

impl LayoutObject for StretchLayoutObject {
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
            self.current_child.set(self.current_child.get() + 1);

            if self.current_child.get() == children.len() {
                let width = {
                    if self.width.get() > 0 {
                        self.width.get()
                    } else {
                        size.0
                    }
                };

                let height = {
                    if self.height.get() > 0 {
                        self.height.get()
                    } else {
                        size.1
                    }
                };
                return LayoutResult::Size(constraint.perform((width, height)));
            }
        } else {
            if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
                self.width.set(theme.uint("width", selector) as u32);
                self.height.set(theme.uint("height", selector) as u32);

                if self.width.get() == 0 {
                   self.width.set(constraint.width);
                }

                if self.height.get() == 0 {
                    self.height.set(constraint.height);
                }
            }

            if children.is_empty() {
                return LayoutResult::Size((self.width.get(), self.height.get()));
            }    

              self.current_child.set(0);
        }

        LayoutResult::RequestChild(children[self.current_child.get()], *constraint)
    }
}
