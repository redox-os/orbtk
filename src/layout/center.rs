use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    layout::Layout,
    properties::{Bounds, Constraint},
    theme::{Selector, Theme},
    LayoutResult,
};

pub struct CenterLayout;

impl Into<Box<dyn Layout>> for CenterLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}

impl Layout for CenterLayout {
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
            // If a size is provided, check if the constraint should override it
            // This follows logic found in PaddingLayout and other places but may not be relevant here.
            let mut width = {
                if constraint.width > 0 {
                    constraint.width
                } else {
                    size.0
                }
            };
            let mut height = {
                if constraint.height > 0 {
                    constraint.height
                } else {
                    size.1
                }
            };

            // Requirement: CSS needs to define width and height for centering to work.
            if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
                let w = theme.uint("width", selector);
                let h = theme.uint("height", selector);
                if w > 0 {
                    width = w;
                }
                if h > 0 {
                    height = h;
                }
            }

            // Center the child object within the layout_size.
            let layout_size = constraint.perform((width, height));
            if let Ok(bounds) = ecm.borrow_mut_component::<Bounds>(children[0]) {
                bounds.x = (layout_size.0 - size.0) as i32 / 2;
                bounds.y = (layout_size.1 - size.1) as i32 / 2;
            }

            return LayoutResult::Size((width, height));
        } else {
            if children.is_empty() {
                let mut width = constraint.max_width;
                let mut height = constraint.max_height;

                if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
                    let w = theme.uint("width", selector);
                    let h = theme.uint("height", selector);

                    if w > 0 {
                        width = w;
                    }

                    if h > 0 {
                        height = h;
                    }
                }
                return LayoutResult::Size((width, height));
            }

            LayoutResult::RequestChild(children[0], *constraint)
        }
    }
}
