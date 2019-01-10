use dces::{Entity, EntityComponentManager};

use crate::{
    core::{FontMeasure, FONT_MEASURE},
    layout::Layout,
    properties::{Constraint, Label},
    theme::{Selector, Theme},
    LayoutResult,
};

pub struct TextSizeLayout;

impl Into<Box<dyn Layout>> for TextSizeLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}

impl Layout for TextSizeLayout {
    fn layout(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        _constraint: &Constraint,
        _children: &[Entity],
        _size: Option<(u32, u32)>,
        theme: &Theme,
    ) -> LayoutResult {
        if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
            if let Ok(label) = ecm.borrow_component::<Label>(entity) {
                let size = {
                    if label.0.is_empty() {
                        (0, 0)
                    } else {
                        FONT_MEASURE.measure(
                            &label.0,
                            &theme.string("font-family", selector),
                            theme.uint("font-size", selector),
                        )
                    }
                };
                return LayoutResult::Size(size);
            }
        }

        LayoutResult::Size((0, 0))
    }
}
