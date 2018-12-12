use dces::{Entity, EntityComponentManager};

use layout_object::LayoutObject;
use properties::{Constraint, Padding, Bounds};
use systems::LayoutResult;
use theme::{Selector, Theme};

pub struct PaddingLayoutObject;

impl Into<Box<LayoutObject>> for PaddingLayoutObject {
    fn into(self) -> Box<LayoutObject> {
        Box::new(self)
    }
}

impl LayoutObject for PaddingLayoutObject {
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
                        .with_left(theme.uint("padding-left", selector) as i32)
                        .with_top(theme.uint("padding-top", selector) as i32)
                        .with_right(theme.uint("padding-right", selector) as i32)
                        .with_bottom(theme.uint("padding-bottom", selector) as i32)
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
                    .with_min_width(constraint.min_width as i32 - (padding.left + padding.right))
                    .with_max_width(constraint.max_width as i32 - (padding.left + padding.right))
                    .with_width(constraint.width as i32 - (padding.left + padding.right))
                    .with_min_height(constraint.min_height as i32 - (padding.top + padding.bottom))
                    .with_max_height(constraint.max_height as i32 - (padding.top + padding.bottom))
                    .with_height(constraint.height as i32 - (padding.top + padding.bottom)),
            )
        }
    }
}
