use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    application::Tree,
    backend::{FontMeasure, FONT_MEASURE},
    properties::{Bounds, Margin, Offset, Text, TextSelection, Visibility},
    structs::{Size, Spacer},
    theme::{Selector, Theme},
};

use super::{get_constraint, get_margin, get_vertical_alignment, get_visibility, Layout};

/// The text selection layout is used to measure and arrange a text selection cursor.
#[derive(Default)]
pub struct TextSelectionLayout {
    desired_size: Cell<(f64, f64)>,
}

impl TextSelectionLayout {
    pub fn new() -> Self {
        TextSelectionLayout::default()
    }
}

impl Into<Box<dyn Layout>> for TextSelectionLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}

impl Layout for TextSelectionLayout {
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

        let constraint = get_constraint(entity, ecm);

        for child in &tree.children[&entity] {
            if let Some(child_layout) = layouts.borrow().get(child) {
                child_layout.measure(*child, ecm, tree, layouts, theme);
            }
        }

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
        theme: &Theme,
    ) -> (f64, f64) {
        if get_visibility(entity, ecm) == Visibility::Collapsed {
            return (0.0, 0.0);
        }

        let mut pos = 0.0;
        let mut desired_size = self.desired_size.get();

        let vertical_alignment = get_vertical_alignment(entity, ecm);
        let margin = get_margin(entity, ecm);

        desired_size.1 = vertical_alignment.align_height(parent_size.1, desired_size.1, margin);

        if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
            if let Ok(text) = ecm.borrow_component::<Text>(entity) {
                if let Ok(selection) = ecm.borrow_component::<TextSelection>(entity) {
                    if let Some(text_part) = text.0.get(0..selection.start_index) {
                        pos = FONT_MEASURE
                            .measure(
                                text_part,
                                &theme.string("font-family", selector),
                                theme.uint("font-size", selector),
                            )
                            .0 as f64;

                        if text_part.ends_with(" ") {
                            pos += (FONT_MEASURE
                                .measure(
                                    "a",
                                    &theme.string("font-family", selector),
                                    theme.uint("font-size", selector),
                                )
                                .0
                                / 2) as f64;
                        }
                    }
                }
            }
        }

        if let Ok(off) = ecm.borrow_component::<Offset>(entity) {
            pos += off.0;
        }

        if let Ok(margin) = ecm.borrow_mut_component::<Margin>(entity) {
            margin.set_left(pos);
        }

        for child in &tree.children[&entity] {
            if let Some(child_layout) = layouts.borrow().get(child) {
                child_layout.arrange(self.desired_size.get(), *child, ecm, tree, layouts, theme);
            }
        }

        if let Ok(bounds) = ecm.borrow_mut_component::<Bounds>(entity) {
            bounds.set_width(desired_size.0);
            bounds.set_height(desired_size.1);
        }

        self.desired_size.set(desired_size);
        self.desired_size.get()
    }
}
