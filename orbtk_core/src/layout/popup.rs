use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
};

use dces::prelude::*;

use crate::{
    prelude::*, proc_macros::IntoLayout, render::RenderContext2D, theming::*, tree::Tree,
    utils::prelude::*,
};

use super::{component, component_try_mut, try_component, Layout};

/// Add padding to the widget.
#[derive(Default, IntoLayout)]
pub struct PopupLayout {
    desired_size: RefCell<DirtySize>,
    old_alignment: Cell<(Alignment, Alignment)>,
}

impl PopupLayout {
    /// Preset the defaults.
    pub fn new() -> Self {
        PopupLayout::default()
    }
}

impl Layout for PopupLayout {
    fn measure(
        &self,
        render_context_2_d: &mut RenderContext2D,
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree>,
        layouts: &BTreeMap<Entity, Box<dyn Layout>>,
        theme: &Theme,
    ) -> DirtySize {
        if component::<Visibility>(ecm, entity, "visibility") == Visibility::Collapsed {
            self.desired_size.borrow_mut().set_size(0.0, 0.0);
            return *self.desired_size.borrow();
        }

        if let Some(target) = try_component::<u32>(ecm, entity, "target") {
            let target_bounds = component::<Rectangle>(ecm, target.into(), "bounds");
            component_try_mut::<Constraint>(ecm, entity, "constraint")
                .unwrap()
                .set_width(target_bounds.width());
        }

        let horizontal_alignment: Alignment = component(ecm, entity, "h_align");
        let vertical_alignment: Alignment = component(ecm, entity, "v_align");

        if horizontal_alignment != self.old_alignment.get().1
            || vertical_alignment != self.old_alignment.get().0
        {
            self.desired_size.borrow_mut().set_dirty(true);
        }

        let constraint: Constraint = component(ecm, entity, "constraint");
        if constraint.width() > 0.0 {
            self.desired_size.borrow_mut().set_width(constraint.width());
        }

        if constraint.height() > 0.0 {
            self.desired_size
                .borrow_mut()
                .set_height(constraint.height());
        }

        let padding: Thickness = component(ecm, entity, "padding");

        for index in 0..ecm.entity_store().children[&entity].len() {
            let child = ecm.entity_store().children[&entity][index];

            if let Some(child_layout) = layouts.get(&child) {
                let child_desired_size =
                    child_layout.measure(render_context_2_d, child, ecm, layouts, theme);
                let mut desired_size = self.desired_size.borrow().size();

                let dirty = child_desired_size.dirty() || self.desired_size.borrow().dirty();
                self.desired_size.borrow_mut().set_dirty(dirty);

                let child_margin = *ecm
                    .component_store()
                    .get::<Thickness>("margin", child)
                    .unwrap();

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

        *self.desired_size.borrow()
    }

    fn arrange(
        &self,
        render_context_2_d: &mut RenderContext2D,
        parent_size: (f64, f64),
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree>,
        layouts: &BTreeMap<Entity, Box<dyn Layout>>,
        theme: &Theme,
    ) -> (f64, f64) {
        if component::<Visibility>(ecm, entity, "visibility") == Visibility::Collapsed {
            self.desired_size.borrow_mut().set_size(0.0, 0.0);
            return (0.0, 0.0);
        }

        if !self.desired_size.borrow().dirty() {
            return self.desired_size.borrow().size();
        }

        let horizontal_alignment: Alignment = component(ecm, entity, "h_align");
        let vertical_alignment: Alignment = component(ecm, entity, "v_align");
        let margin = *ecm
            .component_store()
            .get::<Thickness>("margin", entity)
            .unwrap();
        let padding: Thickness = component(ecm, entity, "padding");
        let constraint: Constraint = component(ecm, entity, "constraint");

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

        if let Some(bounds) = component_try_mut::<Rectangle>(ecm, entity, "bounds") {
            bounds.set_width(size.0);
            bounds.set_height(size.1);
        }

        mark_as_dirty("bounds", entity, ecm);

        let available_size = (
            size.0 - padding.left() - padding.right(),
            size.1 - padding.top() - padding.bottom(),
        );

        for index in 0..ecm.entity_store().children[&entity].len() {
            let child = ecm.entity_store().children[&entity][index];

            let child_margin: Thickness = component(ecm, entity, "margin");

            if let Some(child_layout) = layouts.get(&child) {
                child_layout.arrange(
                    render_context_2_d,
                    available_size,
                    child,
                    ecm,
                    layouts,
                    theme,
                );
            }

            let child_horizontal_alignment: Alignment =
                *ecm.component_store().get("h_align", child).unwrap();
            let child_vertical_alignment: Alignment =
                *ecm.component_store().get("v_align", child).unwrap();
            if let Ok(child_bounds) = ecm
                .component_store_mut()
                .get_mut::<Rectangle>("bounds", child)
            {
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

            mark_as_dirty("bounds", child, ecm);
        }

        self.desired_size.borrow_mut().set_dirty(false);
        size
    }
}
