use std::collections::HashMap;
use std::sync::Arc;

use {
    BoxConstraints, Entity, EntityComponentManager, LayoutObject, LayoutResult, Selector, Theme,
    Thickness,
};

pub struct PaddingLayoutObject;

impl LayoutObject for PaddingLayoutObject {
    fn layout(
        &self,
        entity: Entity,
        ecm: &EntityComponentManager,
        bc: &BoxConstraints,
        children: &[Entity],
        children_pos: &mut HashMap<Entity, (i32, i32)>,
        size: Option<(u32, u32)>,
        theme: &Arc<Theme>,
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
            children_pos.insert(children[0], (padding.left, padding.top));
            LayoutResult::Size((
                size.0 + padding.left as u32 + padding.right as u32,
                size.1 + padding.top as u32 + padding.bottom as u32,
            ))
        } else {
            let child_bc = BoxConstraints {
                min_width: (bc.min_width as i32 - (padding.left + padding.right)).max(0) as u32,
                max_width: (bc.max_width as i32 - (padding.left + padding.right)).max(0) as u32,
                min_height: (bc.min_height as i32 - (padding.top + padding.bottom)).max(0) as u32,
                max_height: (bc.max_height as i32 - (padding.top + padding.bottom)).max(0) as u32,
            };
            LayoutResult::RequestChild(children[0], child_bc)
        }
    }
}
