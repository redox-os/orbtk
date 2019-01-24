use std::cell::Cell;

use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    backend::{FontMeasure, FONT_MEASURE},
    layout::Layout,
    properties::{Constraint, FontIcon, PrimaryFontIcon, SecondaryFontIcon},
    theme::{Selector, Theme},
    LayoutResult
};

#[derive(Default)]
pub struct FixedSizeLayout {
    width: Cell<u32>,
    height: Cell<u32>,
}

impl Into<Box<dyn Layout>> for FixedSizeLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}

impl Layout for FixedSizeLayout {
    fn layout(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        parent_constraint: &Constraint,
        children: &[Entity],
        child_size: Option<(f64, f64)>,
    ) -> LayoutResult {
//        if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
//            let icon = if let Ok(icon) = ecm.borrow_component::<FontIcon>(entity) {
//                Some(&icon.0)
//            } else if let Ok(icon) = ecm.borrow_component::<PrimaryFontIcon>(entity) {
//                Some(&icon.0)
//            } else if let Ok(icon) = ecm.borrow_component::<SecondaryFontIcon>(entity) {
//                Some(&icon.0)
//            } else {
//                None
//            };
//
//            if let Some(icon) = icon {
//                let size = {
//                    if icon.is_empty() {
//                        (0, 0)
//                    } else {
//                        let mut size = FONT_MEASURE.measure(
//                            icon,
//                            &theme.string("icon-font-family", selector),
//                            theme.uint("icon-size", selector),
//                        );
//                        if size.0 == 0 {
//                            size = (0, 0);
//                        }
//                        size.0 = size.0 + theme.uint("icon-margin", selector);
//                        size
//                    }
//                };
//
//                return LayoutResult::Size((size.0 as f64, size.1 as f64));
//            }
//        }

        LayoutResult::Size((0.0, 0.0))
    }
}
