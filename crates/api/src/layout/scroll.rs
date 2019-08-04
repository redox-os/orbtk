use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    f64,
    rc::Rc,
};

use dces::prelude::Entity;

use crate::{prelude::*, render::RenderContext2D, tree::Tree, utils::prelude::*};

use super::Layout;

/// IMPORTANT: The scroll layout will only work for the text box now. A update will follow!!!!
#[derive(Default)]
pub struct ScrollLayout {
    old_child_size: Cell<(f64, f64)>,
    desired_size: RefCell<DirtySize>,
    old_offset: Cell<(f64, f64)>,
    old_alignment: Cell<(Alignment, Alignment)>,
}

impl ScrollLayout {
    pub fn new() -> Self {
        ScrollLayout::default()
    }
}

impl Layout for ScrollLayout {
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

        for child in &tree.children[&entity] {
            if let Some(child_layout) = layouts.borrow().get(child) {
                let dirty = child_layout
                    .measure(render_context_2_d, *child, tree, store, layouts, theme)
                    .dirty()
                    || self.desired_size.borrow().dirty();

                self.desired_size.borrow_mut().set_dirty(dirty);
            }
        }

        let off = Offset::get(entity, store);

        if self.old_offset.get().0 != off.x || self.old_offset.get().1 != off.y {
            self.old_offset.set((off.x, off.y));
            self.desired_size.borrow_mut().set_dirty(true);
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
        let _padding = Padding::get(entity, store);
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

        // let mut vertical_scroll_mode = ScrollMode::default();
        // let mut horizontal_scroll_mode = ScrollMode::default();

        // if let Ok(mode) = ecm.borrow_component::<ScrollViewerMode>(entity) {
        //     vertical_scroll_mode = mode.vertical;
        //     horizontal_scroll_mode = mode.horizontal;
        // }

        let off = Offset::get(entity, store);
        let mut offset = (off.x, off.y);

        let old_child_size = self.old_child_size.get();

        for child in &tree.children[&entity] {
            // let child_margin = get_margin(*child, store);
            let mut child_size = old_child_size;
            let child_vertical_alignment = VerticalAlignment::get(*child, store);
            let child_margin = Margin::get(*child, store);

            if let Some(child_layout) = layouts.borrow().get(child) {
                child_size = child_layout.arrange(
                    render_context_2_d,
                    (f64::MAX, f64::MAX),
                    *child,
                    tree,
                    store,
                    layouts,
                    theme,
                );
            }

            if child_size.0 > size.0 {
                offset.0 = (offset.0 + old_child_size.0 - child_size.0).min(0.0);
            } else {
                offset.0 = 0.0;
            }

            if let Ok(child_bounds) = store.borrow_mut_component::<Bounds>(*child) {
                child_bounds.set_x(offset.0);
                child_bounds.set_y(child_vertical_alignment.align_position(
                    size.1,
                    child_bounds.height(),
                    child_margin.top(),
                    child_margin.bottom(),
                ));
            }

            if let Ok(off) = store.borrow_mut_component::<Offset>(entity) {
                (off.0).x = offset.0;
                (off.0).y = offset.1;
            }

            self.old_child_size.set(child_size);
        }

        self.desired_size.borrow_mut().set_dirty(false);
        size
    }
}

impl Into<Box<dyn Layout>> for ScrollLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}
