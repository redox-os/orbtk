use std::cell::Cell;

use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    layout::Layout,
    properties::{Bounds, Constraint, HorizontalAlignment, Margin, VerticalAlignment},
    structs::{Position, Size, Spacer},
    LayoutResult,
};

#[derive(Default)]
pub struct GridLayout {
    current_child: Cell<usize>,
}

impl Into<Box<dyn Layout>> for GridLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}

impl Layout for GridLayout {
    fn layout(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        parent_constraint: &Constraint,
        children: &[Entity],
        child_size: Option<(f64, f64)>,
    ) -> LayoutResult {

        let mut constraint = {
            if let Ok(constraint) = ecm.borrow_component::<Constraint>(entity) {
                *constraint
            } else {
                Constraint::default()
            }
        };

        let margin = {
            if let Ok(margin) = ecm.borrow_component::<Margin>(entity) {
                *margin
            } else {
                Margin::default()
            }
        };

        let vertical_alignment = get_vertical_alignment(entity, ecm);
        let horizontal_alignment = get_horizontal_alignment(entity, ecm);

        // size is independent from the children
        let size = constraint.perform((
            horizontal_alignment.align_width(parent_constraint.width(), constraint.width(), margin),
            vertical_alignment.align_height(
                parent_constraint.height(),
                constraint.height(),
                margin,
            ),
        ));

        constraint.set_width(size.0);
        constraint.set_height(size.1);

        // todo: this is complete wip, column and rows not implemented at the moment
        if let Some(child_size) = child_size {
            // child margin
            let c_margin = {
                if let Ok(margin) =
                    ecm.borrow_component::<Margin>(children[self.current_child.get()])
                {
                    *margin
                } else {
                    Margin::default()
                }
            };

            let c_vertical_alignment = get_vertical_alignment(children[self.current_child.get()], ecm);
            let c_horizontal_alignment =  get_horizontal_alignment(children[self.current_child.get()], ecm);

            // center child if now row and columns are set
            if let Ok(c_bounds) =
                ecm.borrow_mut_component::<Bounds>(children[self.current_child.get()])
            {
                c_bounds.set_x(
                    c_horizontal_alignment.align_x(size.0, child_size.0, c_margin),
                );
                c_bounds.set_y(
                    c_vertical_alignment.align_y(size.1, child_size.1, c_margin),
                );
            }

            self.current_child.set(self.current_child.get() + 1);

            if self.current_child.get() < children.len() {
               return LayoutResult::RequestChild(children[self.current_child.get()], constraint);
            }

            return LayoutResult::Size(size);
        } else {
            if children.is_empty() {
                return LayoutResult::Size(size);
            }

            self.current_child.set(0);
        }

        LayoutResult::RequestChild(children[self.current_child.get()], constraint)
    }
}

// --- helpers ---

fn get_vertical_alignment(entity: Entity, ecm: &EntityComponentManager) -> VerticalAlignment {
    if let Ok(vertical_alignment) = ecm.borrow_component::<VerticalAlignment>(entity) {
        return *vertical_alignment;
    }

    VerticalAlignment::default()

}

fn get_horizontal_alignment(entity: Entity, ecm: &EntityComponentManager) -> HorizontalAlignment {
    if let Ok(horizontal_alignment) = ecm.borrow_component::<HorizontalAlignment>(entity) {
        return *horizontal_alignment;
    }

    HorizontalAlignment::default()
}

// --- helpers ---