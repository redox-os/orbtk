use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    layout::Layout,
    properties::{Constraint, Bounds},
    structs::{Position, Size},
    theme::{Selector, Theme},
    LayoutResult,
};

pub struct RootLayout;

impl Into<Box<dyn Layout>> for RootLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}

impl Layout for RootLayout {
    fn layout(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        parent_constraint: &Constraint,
        children: &[Entity],
        size: Option<(f64, f64)>,
    ) -> LayoutResult {
        let bounds = {
            if let Ok(bounds) = ecm.borrow_component::<Bounds>(entity) {
                *bounds
            } else {
                Bounds::default()
            }
        };

        if let Some(_size) = size {
            // Expands the root element always to the size of the window.
            if let Ok(c_bounds) = ecm.borrow_mut_component::<Bounds>(children[0]) {
                c_bounds.set_width(bounds.width());
                c_bounds.set_height(bounds.height());
            }

            LayoutResult::Size((parent_constraint.width(), parent_constraint.height()))
        } else {
            if children.is_empty() {
                return LayoutResult::Size((parent_constraint.width(), parent_constraint.height()));
            }

            LayoutResult::RequestChild(children[0], *parent_constraint)
        }
    }
}
