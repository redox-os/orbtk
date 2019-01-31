use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    application::Tree,
    backend::{FontMeasure, FONT_MEASURE},
    properties::{Bounds, Constraint, FontIcon, Image, Text, Visibility, WaterMark},
    structs::Size,
    theme::{Selector, Theme},
};

use super::{get_constraint, get_visibility, Layout};

/// Fixed size layout is defined by fixed bounds like the size of an image or the size of a text.
#[derive(Default)]
pub struct FixedSizeLayout {
    desired_size: Cell<(f64, f64)>,
}

impl Into<Box<dyn Layout>> for FixedSizeLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}

impl FixedSizeLayout {
    pub fn new() -> Self {
        FixedSizeLayout::default()
    }
}

impl Layout for FixedSizeLayout {
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

        // -- todo will be removed after orbgl merge --

        let size = {
            if let Ok(image) = ecm.borrow_component::<Image>(entity) {
                Some((image.width(), image.height()))
            } else if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
                if let Ok(text) = ecm.borrow_component::<Text>(entity) {
                    if text.0.is_empty() {
                        if let Ok(water_mark) = ecm.borrow_component::<WaterMark>(entity) {
                            if water_mark.0.is_empty() {
                                None
                            } else {
                                Some(FONT_MEASURE.measure(
                                    &text.0,
                                    &theme.string("font-family", selector),
                                    theme.uint("font-size", selector),
                                ))
                            }
                        } else {
                            None
                        }
                    } else {
                        Some(FONT_MEASURE.measure(
                            &text.0,
                            &theme.string("font-family", selector),
                            theme.uint("font-size", selector),
                        ))
                    }
                } else if let Ok(font_icon) = ecm.borrow_component::<FontIcon>(entity) {
                    if font_icon.0.is_empty() {
                        None
                    } else {
                        Some(FONT_MEASURE.measure(
                            &font_icon.0,
                            &theme.string("icon-font-family", selector),
                            theme.uint("icon-size", selector),
                        ))
                    }
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(size) = size {
            if let Ok(constraint) = ecm.borrow_mut_component::<Constraint>(entity) {
                constraint.set_width(size.0 as f64);
                constraint.set_height(size.1 as f64);
            }
        }

        // -- todo will be removed after orbgl merge --

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
        _parent_size: (f64, f64),
        entity: Entity,
        ecm: &mut EntityComponentManager,
        tree: &Tree,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    ) -> (f64, f64) {
        if get_visibility(entity, ecm) == Visibility::Collapsed {
            return (0.0, 0.0);
        }

        if let Ok(bounds) = ecm.borrow_mut_component::<Bounds>(entity) {
            bounds.set_width(self.desired_size.get().0);
            bounds.set_height(self.desired_size.get().1);
        }

        for child in &tree.children[&entity] {
            if let Some(child_layout) = layouts.borrow().get(child) {
                child_layout.arrange(self.desired_size.get(), *child, ecm, tree, layouts);
            }
        }

        self.desired_size.get()
    }
}
