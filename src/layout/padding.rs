use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    layout::Layout,
    properties::{Bounds, Constraint, Padding},
    theme::{Selector, Theme},
    LayoutResult,
};

pub struct PaddingLayout;

impl Into<Box<dyn Layout>> for PaddingLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}

impl Layout for PaddingLayout {
    fn layout(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        constraint: &Constraint,
        children: &[Entity],
        size: Option<(u32, u32)>,
        theme: &Theme,
    ) -> LayoutResult {
        let padding = {
            let padding = Padding::default();
            if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
                let pad = theme.uint("padding", selector) as i32;

                if pad > 0 {
                    padding.with(pad)
                } else {
                    padding
                        .left(theme.uint("padding-left", selector) as i32)
                        .top(theme.uint("padding-top", selector) as i32)
                        .right(theme.uint("padding-right", selector) as i32)
                        .bottom(theme.uint("padding-bottom", selector) as i32)
                }
            } else {
                padding
            }
        };

        if let Some(size) = size {
            if let Ok(bounds) = ecm.borrow_mut_component::<Bounds>(children[0]) {
                bounds.x = padding.left;
                bounds.y = padding.top;
            }

            let width = {
                if constraint.width > 0 {
                    constraint.width
                } else {
                    size.0 + padding.left as u32 + padding.right as u32
                }
            };

            let height = {
                if constraint.height > 0 {
                    constraint.height
                } else {
                    size.1 + padding.top as u32 + padding.bottom as u32
                }
            };

            LayoutResult::Size(constraint.perform((width, height)))
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

            LayoutResult::RequestChild(
                children[0],
                Constraint::default()
                    .min_width(constraint.min_width as i32 - (padding.left + padding.right))
                    .max_width(constraint.max_width as i32 - (padding.left + padding.right))
                    .width(constraint.width as i32 - (padding.left + padding.right))
                    .min_height(constraint.min_height as i32 - (padding.top + padding.bottom))
                    .max_height(constraint.max_height as i32 - (padding.top + padding.bottom))
                    .height(constraint.height as i32 - (padding.top + padding.bottom)),
            )
        }
    }
}
