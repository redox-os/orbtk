use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    application::Tree,
    properties::{Bounds, Constraint, GridColumn, HorizontalAlignment, Margin, VerticalAlignment},
    structs::Size,
    systems::LayoutResult,
};

use super::{get_constraint, Layout};

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
    ) -> (f64, f64) {
        self.desired_size.set((0.0, 0.0));

        let constraint = get_constraint(entity, ecm);
        self.desired_size
            .set((constraint.width(), constraint.height()));

        for child in &tree.children[&entity] {
            if let Some(child_layout) = layouts.borrow().get(child) {
                child_layout.measure(*child, ecm, tree, layouts);
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
