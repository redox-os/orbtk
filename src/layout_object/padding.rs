use dces::{Entity, EntityComponentManager};

use layout_object::LayoutObject;
use structs::{Constraint, Rect, Thickness};
use systems::LayoutResult;
use theme::{Selector, Theme};

pub struct PaddingLayoutObject;

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
            let mut padding = Thickness::new(0, 0, 0, 0);
            if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
                let pad = theme.uint("padding", selector) as i32;

                if pad > 0 {
                    padding = Thickness::new(pad, pad, pad, pad)
                } else {
                    padding = Thickness::new(
                        theme.uint("padding-left", selector) as i32,
                        theme.uint("padding-top", selector) as i32,
                        theme.uint("padding-right", selector) as i32,
                        theme.uint("padding-bottom", selector) as i32,
                    )
                }
            };

            padding
        };

        if let Some(size) = size {
            if let Ok(bounds) = ecm.borrow_mut_component::<Rect>(children[0]) {
                bounds.x = padding.left;
                bounds.y = padding.top;
            }

            LayoutResult::Size(constraint.perform((
                size.0 + padding.left as u32 + padding.right as u32,
                size.1 + padding.top as u32 + padding.bottom as u32,
            )))
        } else {
            let child_bc = Constraint {
                min_width: (constraint.min_width as i32 - (padding.left + padding.right)).max(0)
                    as u32,
                max_width: (constraint.max_width as i32 - (padding.left + padding.right)).max(0)
                    as u32,
                min_height: (constraint.min_height as i32 - (padding.top + padding.bottom)).max(0)
                    as u32,
                max_height: (constraint.max_height as i32 - (padding.top + padding.bottom)).max(0)
                    as u32,
            };
            LayoutResult::RequestChild(children[0], child_bc)
        }
    }
}
