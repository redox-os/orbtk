use std::{cell::{Cell, RefCell}, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    application::Tree,
    properties::{Bounds, Visibility, VerticalAlignment, HorizontalAlignment},
    structs::{DirtySize, Position, Size, Spacer},
    theme::Theme,
};

use super::{
    get_constraint, get_horizontal_alignment, get_margin, get_padding, get_vertical_alignment,
    get_visibility, Layout,
};

/// Add padding to the widget.
#[derive(Default)]
pub struct PaddingLayout {
    desired_size: RefCell<DirtySize>,
    old_alignment: Cell<(VerticalAlignment, HorizontalAlignment)>,
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
    ) -> DirtySize {
        if get_visibility(entity, ecm) == Visibility::Collapsed {
            self.desired_size.borrow_mut().set_size(0.0, 0.0);
            return self.desired_size.borrow().clone();
        }

        let horizontal_alignment = get_horizontal_alignment(entity, ecm);
        let vertical_alignment = get_vertical_alignment(entity, ecm);

         if horizontal_alignment != self.old_alignment.get().1
            || vertical_alignment != self.old_alignment.get().0
        {
            self.desired_size.borrow_mut().set_dirty(true);
        }

        let constraint = get_constraint(entity, ecm);
        if constraint.width() > 0.0 {
            self.desired_size.borrow_mut().set_width(constraint.width());
        }

        if constraint.height() > 0.0 {
            self.desired_size.borrow_mut().set_height(constraint.height());
        }

        let padding = get_padding(entity, ecm);

        for child in &tree.children[&entity] {
            if let Some(child_layout) = layouts.borrow().get(child) {
                let child_desired_size = child_layout.measure(*child, ecm, tree, layouts, theme);
                let mut desired_size = self.desired_size.borrow().size();

                let dirty = child_desired_size.dirty() || self.desired_size.borrow().dirty();
                self.desired_size
                    .borrow_mut()
                    .set_dirty(dirty);

                let child_margin = get_margin(*child, ecm);

                desired_size.0 = desired_size.0.max(
                    child_desired_size.width()
                        + padding.left()
                        + padding.right()
                        + child_margin.left()
                        + child_margin.right(),
                );
                desired_size.1 = desired_size.1.max(
                    child_desired_size.height()
                        + padding.top()
                        + padding.bottom()
                        + child_margin.top()
                        + child_margin.left(),
                );

                self.desired_size
                    .borrow_mut()
                    .set_size(desired_size.0, desired_size.1);
            }
        }

        self.desired_size.borrow().clone()
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
        if !self.desired_size.borrow().dirty() {
            return self.desired_size.borrow().size();
        }

        let horizontal_alignment = get_horizontal_alignment(entity, ecm);
        let vertical_alignment = get_vertical_alignment(entity, ecm);
        let margin = get_margin(entity, ecm);
        let padding = get_padding(entity, ecm);
        let constraint = get_constraint(entity, ecm);

        let size = constraint.perform((
            horizontal_alignment.align_width(
                parent_size.0,
                self.desired_size.borrow().width(),
                margin,
            ),
            vertical_alignment.align_height(
                parent_size.1,
                self.desired_size.borrow().height(),
                margin,
            ),
        ));


        if let Ok(bounds) = ecm.borrow_mut_component::<Bounds>(entity) {
            bounds.set_width(size.0);
            bounds.set_height(size.1);
        }

        let available_size = (
            size.0 - padding.left() - padding.right(),
            size.1 - padding.top() - padding.bottom(),
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

        self.desired_size.borrow_mut().set_dirty(false);
        size
    }
}

impl Into<Box<dyn Layout>> for PaddingLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}
