use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    f64,
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    application::Tree,
    layout::Layout,
    properties::{Bounds, Offset, ScrollMode, ScrollViewerMode, Visibility},
    structs::{Size, Position},
    theme::Theme,
};

use super::{
    get_constraint, get_horizontal_alignment, get_margin, get_vertical_alignment, get_visibility,
};

// todo: not finished yet!!!!

#[derive(Default)]
pub struct ScrollLayout {
    old_child_size: Cell<(f64, f64)>,
    desired_size: Cell<(f64, f64)>,
}

impl ScrollLayout {
    pub fn new() -> Self {
        ScrollLayout::default()
    }
}

impl Into<Box<dyn Layout>> for ScrollLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}

impl Layout for ScrollLayout {
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

        for child in &tree.children[&entity] {
            if let Some(child_layout) = layouts.borrow().get(child) {
                child_layout.measure(*child, ecm, tree, layouts, theme);
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
    ) -> (f64, f64) {
        if get_visibility(entity, ecm) == Visibility::Collapsed {
            return (0.0, 0.0);
        }

        let horizontal_alignment = get_horizontal_alignment(entity, ecm);
        let vertical_alignment = get_vertical_alignment(entity, ecm);
        let margin = get_margin(entity, ecm);
        let constraint = get_constraint(entity, ecm);

        self.desired_size.set(constraint.perform((
            horizontal_alignment.align_width(parent_size.0, self.desired_size.get().0, margin),
            vertical_alignment.align_height(parent_size.1, self.desired_size.get().1, margin),
        )));

        if let Ok(bounds) = ecm.borrow_mut_component::<Bounds>(entity) {
            bounds.set_width(self.desired_size.get().0);
            bounds.set_height(self.desired_size.get().1);
        }

        let mut vertical_scroll_mode = ScrollMode::default();
        let mut horizontal_scroll_mode = ScrollMode::default();

        if let Ok(mode) = ecm.borrow_component::<ScrollViewerMode>(entity) {
            vertical_scroll_mode = mode.vertical;
            horizontal_scroll_mode = mode.horizontal;
        }

        let mut offset = (0.0, 0.0);

        let old_child_size = self.old_child_size.get();

        if let Ok(off) = ecm.borrow_component::<Offset>(entity) {
            // off.0 = (center_size.0 as i32 - size.0 as i32).min(0);
            // off.1 = (center_size.1 as i32 - size.1 as i32).min(0);

            offset = (off.0, off.1);
        }

        for child in &tree.children[&entity] {
            let child_margin = get_margin(*child, ecm);
            let mut child_size = old_child_size;

            if let Some(child_layout) = layouts.borrow().get(child) {
                child_size = child_layout.arrange((f64::MAX, f64::MAX), *child, ecm, tree, layouts);
            }

            if vertical_scroll_mode != ScrollMode::None
                && horizontal_scroll_mode != ScrollMode::None
            {
                if child_size.0 <= self.desired_size.get().0 {
                    offset.0 = 0.0;
                } else {
                    let offset_width = old_child_size.0 - child_size.0;

                    if offset_width != 0.0 {
                        offset.0 = (offset.0 + offset_width).min(0.0);
                    }
                }

                if child_size.1 <= self.desired_size.get().1 {
                    offset.1 = 0.0;
                }

                // todo: vertical scrolling
            }

            if let Ok(child_bounds) = ecm.borrow_mut_component::<Bounds>(*child) {
                child_bounds.set_x(offset.0);   
                child_bounds.set_y(offset.1);   
            }

            self.old_child_size.set(child_size);
        }

        self.desired_size.get()
    }
}
