use dces::prelude::{Entity, EntityComponentManager};

use std::cell::Cell;

use crate::{
    enums::Placement,
    layout::Layout,
    properties::{Constraint, Bounds, Canvas},
    theme::{Selector, Theme},
    LayoutResult,
};

#[derive(Default)]
pub struct StretchLayout {
    width: Cell<u32>,
    height: Cell<u32>,
    current_child: Cell<usize>,
}

impl Into<Box<dyn Layout>> for StretchLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}

impl Layout for StretchLayout {
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

            let placement = {
                if let Ok(placement) =
                    ecm.borrow_component::<Placement>(children[self.current_child.get() - 1])
                {
                    Some(*placement)
                } else {
                    None
                }
            };

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

                let p_size = constraint.perform((width, height));

                if let Some(placement) = placement {
                    if let Ok(bounds) =
                        ecm.borrow_mut_component::<Bounds>(children[self.current_child.get() - 1])
                    {
                        if placement == Placement::Right {
                            bounds.x = p_size.0 as i32 - size.0 as i32;
                        } else {
                            bounds.x = 0;
                        }
                    }
                }

                return LayoutResult::Size(p_size);
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

            if let Ok(canvas) = ecm.borrow_component::<Canvas>(entity) {
                self.width.set(canvas.width as u32);
                self.height.set(canvas.height as u32);
            }

            if children.is_empty() {
                return LayoutResult::Size((self.width.get(), self.height.get()));
            }

            self.current_child.set(0);
        }

        LayoutResult::RequestChild(children[self.current_child.get()], *constraint)
    }
}
