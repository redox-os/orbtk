use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    application::Tree,
    properties::{Bounds, Visibility},
    structs::{Position, Size, Spacer},
    theme::Theme,
};

use super::{
    get_constraint, get_horizontal_alignment, get_margin, get_padding, get_vertical_alignment,
    get_visibility, Layout,
};

#[derive(Default)]
pub struct PaddingLayout {
    desired_size: Cell<(f64, f64)>,
}

impl PaddingLayout {
    pub fn new() -> Self {
        PaddingLayout::default()
    }
}

impl Layout for PaddingLayout {
    fn measure(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        tree: &Tree,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &Theme,
    ) -> (f64, f64) {
        if get_visibility(entity, ecm) == Visibility::Collapsed {
            return (0.0, 0.0);
        }

        self.desired_size.set((0.0, 0.0));

        let constraint = get_constraint(entity, ecm);
        self.desired_size
            .set((constraint.width(), constraint.height()));

        let padding = get_padding(entity, ecm);

        for child in &tree.children[&entity] {
            if let Some(child_layout) = layouts.borrow().get(child) {
                let child_desired_size = child_layout.measure(*child, ecm, tree, layouts, theme);
                let mut desired_size = self.desired_size.get();

                let child_margin = get_margin(*child, ecm);

                desired_size.0 = desired_size.0.max(
                    child_desired_size.0
                        + padding.left()
                        + padding.right()
                        + child_margin.left()
                        + child_margin.right(),
                );
                desired_size.1 = desired_size.1.max(
                    child_desired_size.1
                        + padding.top()
                        + padding.bottom()
                        + child_margin.top()
                        + child_margin.left(),
                );

                self.desired_size.set(desired_size);
            }
        }

        self.desired_size.get()
    }

    fn arrange(
        &self,
        parent_size: (f64, f64),
        entity: Entity,
        ecm: &mut EntityComponentManager,
        tree: &Tree,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &Theme,
    ) -> (f64, f64) {
        if get_visibility(entity, ecm) == Visibility::Collapsed {
            return (0.0, 0.0);
        }

        let horizontal_alignment = get_horizontal_alignment(entity, ecm);
        let vertical_alignment = get_vertical_alignment(entity, ecm);
        let margin = get_margin(entity, ecm);
        let padding = get_padding(entity, ecm);
        let constraint = get_constraint(entity, ecm);

        self.desired_size.set(constraint.perform((
            horizontal_alignment.align_width(parent_size.0, self.desired_size.get().0, margin),
            vertical_alignment.align_height(parent_size.1, self.desired_size.get().1, margin),
        )));

        if let Ok(bounds) = ecm.borrow_mut_component::<Bounds>(entity) {
            bounds.set_width(self.desired_size.get().0);
            bounds.set_height(self.desired_size.get().1);
        }

        let available_size = (
            self.desired_size.get().0 - padding.left() - padding.right(),
            self.desired_size.get().1 - padding.top() - padding.bottom(),
        );

        for child in &tree.children[&entity] {
            let child_margin = get_margin(*child, ecm);

            if let Some(child_layout) = layouts.borrow().get(child) {
                child_layout.arrange(available_size, *child, ecm, tree, layouts, theme);
            }

            let child_horizontal_alignment = get_horizontal_alignment(*child, ecm);
            let child_vertical_alignment = get_vertical_alignment(*child, ecm);

            if let Ok(child_bounds) = ecm.borrow_mut_component::<Bounds>(*child) {
                child_bounds.set_x(
                    padding.left()
                        + child_horizontal_alignment.align_x(
                            available_size.0,
                            child_bounds.width(),
                            child_margin,
                        ),
                );
                child_bounds.set_y(
                    padding.top()
                        + child_vertical_alignment.align_y(
                            available_size.1,
                            child_bounds.height(),
                            child_margin,
                        ),
                );
            }
        }

        self.desired_size.get()
    }
}

impl Into<Box<dyn Layout>> for PaddingLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}
