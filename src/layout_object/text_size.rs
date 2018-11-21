use backend::{FontMeasure, FONT_MEASURE};
use dces::{Entity, EntityComponentManager};
use layout_object::LayoutObject;
use structs::Constraint;
use theme::{Selector, Theme};

use {Label, LayoutResult};

pub struct TextSizeLayoutObject;

impl LayoutObject for TextSizeLayoutObject {
    fn layout(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        constraint: &Constraint,
        _children: &[Entity],
        _size: Option<(u32, u32)>,
        theme: &Theme,
    ) -> LayoutResult {
        if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
            if let Ok(label) = ecm.borrow_component::<Label>(entity) {
               return LayoutResult::Size(FONT_MEASURE.measure(
                    &label.0,
                    theme.uint("font-size", selector),
                ))
            }
        }

        LayoutResult::Size((constraint.min_width, constraint.min_height))
    }
}
