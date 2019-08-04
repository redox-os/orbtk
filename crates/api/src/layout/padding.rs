use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::Entity;

use crate::{prelude::*, render::RenderContext2D, tree::Tree, utils::prelude::*};

use super::Layout;

/// Add padding to the widget.
#[derive(Default)]
pub struct PaddingLayout {
    desired_size: RefCell<DirtySize>,
    old_alignment: Cell<(Alignment, Alignment)>,
}

impl PaddingLayout {
    pub fn new() -> Self {
        PaddingLayout::default()
    }
}

impl Layout for PaddingLayout {
    fn measure(
        &self,
        render_context_2_d: &mut RenderContext2D,
        entity: Entity,
        tree: &Tree,
        store: &mut ComponentStore,

        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &ThemeValue,
    ) -> DirtySize {
        if Visibility::get(entity, store) == VisibilityValue::Collapsed {
            self.desired_size.borrow_mut().set_size(0.0, 0.0);
            return self.desired_size.borrow().clone();
        }

        let horizontal_alignment = HorizontalAlignment::get(entity, store);
        let vertical_alignment = VerticalAlignment::get(entity, store);

        if horizontal_alignment != self.old_alignment.get().1
            || vertical_alignment != self.old_alignment.get().0
        {
            self.desired_size.borrow_mut().set_dirty(true);
        }

        let constraint = Constraint::get(entity, store);
        if constraint.width() > 0.0 {
            self.desired_size.borrow_mut().set_width(constraint.width());
        }

        if constraint.height() > 0.0 {
            self.desired_size
                .borrow_mut()
                .set_height(constraint.height());
        }

        let padding = Padding::get(entity, store);

        for child in &tree.children[&entity] {
            if let Some(child_layout) = layouts.borrow().get(child) {
                let child_desired_size =
                    child_layout.measure(render_context_2_d, *child, tree, store, layouts, theme);
                let mut desired_size = self.desired_size.borrow().size();

                let dirty = child_desired_size.dirty() || self.desired_size.borrow().dirty();
                self.desired_size.borrow_mut().set_dirty(dirty);

                let child_margin = Margin::get(*child, store);

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
        render_context_2_d: &mut RenderContext2D,
        parent_size: (f64, f64),
        entity: Entity,
        tree: &Tree,
        store: &mut ComponentStore,

        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &ThemeValue,
    ) -> (f64, f64) {
        if !self.desired_size.borrow().dirty() {
            return self.desired_size.borrow().size();
        }

        let horizontal_alignment = HorizontalAlignment::get(entity, store);
        let vertical_alignment = VerticalAlignment::get(entity, store);
        let margin = Margin::get(entity, store);
        let padding = Padding::get(entity, store);
        let constraint = Constraint::get(entity, store);

        let size = constraint.perform((
            horizontal_alignment.align_measure(
                parent_size.0,
                self.desired_size.borrow().width(),
                margin.left(),
                margin.right(),
            ),
            vertical_alignment.align_measure(
                parent_size.1,
                self.desired_size.borrow().height(),
                margin.top(),
                margin.bottom(),
            ),
        ));

        if let Ok(bounds) = store.borrow_mut_component::<Bounds>(entity) {
            bounds.set_width(size.0);
            bounds.set_height(size.1);
        }

        let available_size = (
            size.0 - padding.left() - padding.right(),
            size.1 - padding.top() - padding.bottom(),
        );

        for child in &tree.children[&entity] {
            let child_margin = Margin::get(*child, store);

            if let Some(child_layout) = layouts.borrow().get(child) {
                child_layout.arrange(
                    render_context_2_d,
                    available_size,
                    *child,
                    tree,
                    store,
                    layouts,
                    theme,
                );
            }

            let child_horizontal_alignment = HorizontalAlignment::get(*child, store);
            let child_vertical_alignment = VerticalAlignment::get(*child, store);

            if let Ok(child_bounds) = store.borrow_mut_component::<Bounds>(*child) {
                child_bounds.set_x(
                    padding.left()
                        + child_horizontal_alignment.align_position(
                            available_size.0,
                            child_bounds.width(),
                            child_margin.left(),
                            child_margin.right(),
                        ),
                );
                child_bounds.set_y(
                    padding.top()
                        + child_vertical_alignment.align_position(
                            available_size.1,
                            child_bounds.height(),
                            child_margin.top(),
                            child_margin.bottom(),
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
