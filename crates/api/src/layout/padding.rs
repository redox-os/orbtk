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
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,

        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &ThemeValue,
    ) -> DirtySize {
        if Visibility::get("visibility", entity, ecm.component_store()) == VisibilityValue::Collapsed {
            self.desired_size.borrow_mut().set_size(0.0, 0.0);
            return self.desired_size.borrow().clone();
        }

        let horizontal_alignment = HorizontalAlignment::get("horizontal_alignment", entity, ecm.component_store());
        let vertical_alignment = VerticalAlignment::get("vertical_alignment", entity, ecm.component_store());

        if horizontal_alignment != self.old_alignment.get().1
            || vertical_alignment != self.old_alignment.get().0
        {
            self.desired_size.borrow_mut().set_dirty(true);
        }

        let constraint = Constraint::get("constraint", entity, ecm.component_store());
        if constraint.width() > 0.0 {
            self.desired_size.borrow_mut().set_width(constraint.width());
        }

        if constraint.height() > 0.0 {
            self.desired_size
                .borrow_mut()
                .set_height(constraint.height());
        }

        let padding = Padding::get("padding", entity, ecm.component_store());

        if ecm.entity_store().children[&entity].len() > 0 {
            let mut index = 0;

            loop {
                let child = ecm.entity_store().children[&entity][index];

                if let Some(child_layout) = layouts.borrow().get(&child) {
                    let child_desired_size =
                        child_layout.measure(render_context_2_d, child, ecm, layouts, theme);
                    let mut desired_size = self.desired_size.borrow().size();

                    let dirty = child_desired_size.dirty() || self.desired_size.borrow().dirty();
                    self.desired_size.borrow_mut().set_dirty(dirty);

                    let child_margin = Margin::get("margin", child, ecm.component_store());

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

                if index + 1 < ecm.entity_store().children[&entity].len() {
                    index += 1;
                } else {
                    break;
                }
            }
        }

        self.desired_size.borrow().clone()
    }

    fn arrange(
        &self,
        render_context_2_d: &mut RenderContext2D,
        parent_size: (f64, f64),
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,

        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &ThemeValue,
    ) -> (f64, f64) {
        if !self.desired_size.borrow().dirty() {
            return self.desired_size.borrow().size();
        }

        let horizontal_alignment = HorizontalAlignment::get("horizontal_alignment", entity, ecm.component_store());
        let vertical_alignment = VerticalAlignment::get("vertical_alignment", entity, ecm.component_store());
        let margin = Margin::get("margin", entity, ecm.component_store());
        let padding = Padding::get("padding", entity, ecm.component_store());
        let constraint = Constraint::get("constraint", entity, ecm.component_store());

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

        if let Ok(bounds) = ecm
            .component_store_mut()
            .borrow_mut_component::<Bounds>("bounds", entity)
        {
            bounds.set_width(size.0);
            bounds.set_height(size.1);
        }

        let available_size = (
            size.0 - padding.left() - padding.right(),
            size.1 - padding.top() - padding.bottom(),
        );

        if ecm.entity_store().children[&entity].len() > 0 {
            let mut index = 0;

            loop {
                let child = ecm.entity_store().children[&entity][index];

                let child_margin = Margin::get("margin", child, ecm.component_store());

                if let Some(child_layout) = layouts.borrow().get(&child) {
                    child_layout.arrange(
                        render_context_2_d,
                        available_size,
                        child,
                        ecm,
                        layouts,
                        theme,
                    );
                }

                let child_horizontal_alignment =
                    HorizontalAlignment::get("horizontal_alignment", child, ecm.component_store());
                let child_vertical_alignment = VerticalAlignment::get("vertical_alignment", child, ecm.component_store());

                if let Ok(child_bounds) = ecm
                    .component_store_mut()
                    .borrow_mut_component::<Bounds>("bounds", child)
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

                if index + 1 < ecm.entity_store().children[&entity].len() {
                    index += 1;
                } else {
                    break;
                }
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
