use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    application::Tree,
    properties::{Bounds, HorizontalAlignment, Margin, Orientation, VerticalAlignment, Visibility},
    structs::{DirtySize, Position, Size, Spacer},
    theme::Theme,
};

use super::{
    get_constraint, get_horizontal_alignment, get_margin, get_vertical_alignment, get_visibility,
    Layout,
};

/// Stacks visual the children widgets vertical or horizontal.
#[derive(Default)]
pub struct StackLayout {
    desired_size: RefCell<DirtySize>,
    old_alignment: Cell<(VerticalAlignment, HorizontalAlignment)>,
}

impl StackLayout {
    pub fn new() -> Self {
        StackLayout::default()
    }
}

impl Layout for StackLayout {
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

        let orientation = get_orientation(entity, ecm);
        let mut desired_size: (f64, f64) = (0.0, 0.0);

        for child in &tree.children[&entity] {
            if let Some(child_layout) = layouts.borrow().get(child) {
                let child_desired_size = child_layout.measure(*child, ecm, tree, layouts, theme);
                let child_margin = {
                    if child_desired_size.width() > 0.0 && child_desired_size.height() > 0.0 {
                        get_margin(*child, ecm)
                    } else {
                        Margin::new()
                    }
                };

                match orientation {
                    Orientation::Horizontal => {
                        desired_size.0 +=
                            child_desired_size.width() + child_margin.left() + child_margin.right();
                        desired_size.1 = desired_size.1.max(
                            child_desired_size.height()
                                + child_margin.top()
                                + child_margin.bottom(),
                        );
                    }
                    _ => {
                        desired_size.0 = desired_size.0.max(
                            child_desired_size.width() + child_margin.left() + child_margin.right(),
                        );
                        desired_size.1 += child_desired_size.height()
                            + child_margin.top()
                            + child_margin.bottom();
                    }
                }

                let dirty = child_desired_size.dirty() || self.desired_size.borrow().dirty();
                self.desired_size.borrow_mut().set_dirty(dirty);
            }
        }

        self.desired_size
            .borrow_mut()
            .set_size(desired_size.0, desired_size.1);
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
        let constraint = get_constraint(entity, ecm);
        let orientation = get_orientation(entity, ecm);
        let mut size_counter = 0.0;

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

        let available_size = size;

        for child in &tree.children[&entity] {
            let mut child_desired_size = (0.0, 0.0);
            if let Some(child_layout) = layouts.borrow().get(child) {
                child_desired_size = child_layout.arrange(size, *child, ecm, tree, layouts, theme);
            }

            let child_margin = {
                if child_desired_size.0 > 0.0 && child_desired_size.1 > 0.0 {
                    get_margin(*child, ecm)
                } else {
                    Margin::new()
                }
            };

            let child_horizontal_alignment = get_horizontal_alignment(*child, ecm);
            let child_vertical_alignment = get_vertical_alignment(*child, ecm);

            if let Ok(child_bounds) = ecm.borrow_mut_component::<Bounds>(*child) {
                match orientation {
                    Orientation::Horizontal => {
                        child_bounds.set_x(
                            size_counter
                                + child_horizontal_alignment.align_x(
                                    available_size.0,
                                    child_bounds.width(),
                                    child_margin,
                                ),
                        );
                        child_bounds.set_y(child_vertical_alignment.align_y(
                            available_size.1,
                            child_bounds.height(),
                            child_margin,
                        ));
                        size_counter +=
                            child_bounds.width() + child_margin.left() + child_margin.right();
                    }
                    _ => {
                        child_bounds.set_x(child_horizontal_alignment.align_x(
                            available_size.0,
                            child_bounds.width(),
                            child_margin,
                        ));
                        child_bounds.set_y(
                            size_counter
                                + child_vertical_alignment.align_y(
                                    available_size.1,
                                    child_bounds.height(),
                                    child_margin,
                                ),
                        );
                        size_counter +=
                            child_bounds.height() + child_margin.top() + child_margin.bottom();
                    }
                }
            }
        }

        self.desired_size.borrow_mut().set_dirty(false);
        size
    }
}

impl Into<Box<dyn Layout>> for StackLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}

// --- helpers ---

fn get_orientation(entity: Entity, ecm: &EntityComponentManager) -> Orientation {
    if let Ok(orientation) = ecm.borrow_component::<Orientation>(entity) {
        return *orientation;
    }

    Orientation::default()
}

// --- helpers ---
