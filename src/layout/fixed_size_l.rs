use std::cell::Cell;

use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    backend::{FontMeasure, FONT_MEASURE},
    layout::{Layout, get_margin},
    properties::{Constraint, FontIcon, PrimaryFontIcon, SecondaryFontIcon},
    theme::{Selector, Theme},
    LayoutResult
};

#[derive(Default)]
pub struct FixedSizeLayout;

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
        let mut size = (0.0, 0.0);

        if let Ok(constraint) = ecm.borrow_component::<Constraint>(entity) {
            size.0 = constraint.width();
            size.1 = constraint.height();
        }

        LayoutResult::Size(size)
    }
}
