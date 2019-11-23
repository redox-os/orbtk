use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
};

use dces::prelude::Entity;

use crate::{prelude::*, render::RenderContext2D, tree::Tree, utils::prelude::*};

use super::Layout;

/// Stacks visual the children widgets vertical or horizontal.
#[derive(Default)]
pub struct StackLayout {
    desired_size: RefCell<DirtySize>,
    old_alignment: Cell<(Alignment, Alignment)>,
}

impl StackLayout {
    pub fn new() -> Self {
        StackLayout::default()
    }
}

impl Layout for StackLayout {
    fn measure(
        &self,
        render_context_2_d: &mut RenderContext2D,
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,

        layouts: &BTreeMap<Entity, Box<dyn Layout>>,
        theme: &ThemeValue,
    ) -> DirtySize {
        if *ecm
            .component_store()
            .get::<Visibility>("visibility", entity)
            .unwrap()
            == Visibility::Collapsed
        {
            self.desired_size.borrow_mut().set_size(0.0, 0.0);
            return *self.desired_size.borrow();
        }

        let horizontal_alignment: Alignment = *ecm
            .component_store()
            .get("horizontal_alignment", entity)
            .unwrap();
        let vertical_alignment: Alignment = *ecm
            .component_store()
            .get("vertical_alignment", entity)
            .unwrap();

        if horizontal_alignment != self.old_alignment.get().1
            || vertical_alignment != self.old_alignment.get().0
        {
            self.desired_size.borrow_mut().set_dirty(true);
        }

        let orientation: Orientation = *ecm.component_store().get("orientation", entity).unwrap();
        let mut desired_size: (f64, f64) = (0.0, 0.0);
        let nchildren = ecm.entity_store().children[&entity].len();
        let spacing = spacing(ecm, entity);

        for index in 0..nchildren {
            let child = ecm.entity_store().children[&entity][index];

            if let Some(child_layout) = layouts.get(&child) {
                let child_desired_size =
                    child_layout.measure(render_context_2_d, child, ecm, layouts, theme);

                let mut child_margin = {
                    if child_desired_size.width() > 0.0 && child_desired_size.height() > 0.0 {
                        *ecm.component_store()
                            .get::<Thickness>("margin", child)
                            .unwrap()
                    } else {
                        Thickness::default()
                    }
                };

                if spacing != 0.0 && nchildren > 1 {
                    apply_spacing(&mut child_margin, spacing, orientation, index, nchildren);
                }

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
        *self.desired_size.borrow()
    }

    fn arrange(
        &self,
        render_context_2_d: &mut RenderContext2D,
        parent_size: (f64, f64),
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
        layouts: &BTreeMap<Entity, Box<dyn Layout>>,
        theme: &ThemeValue,
    ) -> (f64, f64) {
        if *ecm
            .component_store()
            .get::<Visibility>("visibility", entity)
            .unwrap()
            == Visibility::Collapsed
        {
            self.desired_size.borrow_mut().set_size(0.0, 0.0);
            return (0.0, 0.0);
        }

        if !self.desired_size.borrow().dirty() {
            return self.desired_size.borrow().size();
        }

        let horizontal_alignment: Alignment = *ecm
            .component_store()
            .get("horizontal_alignment", entity)
            .unwrap();
        let vertical_alignment: Alignment = *ecm
            .component_store()
            .get("vertical_alignment", entity)
            .unwrap();
        let margin: Thickness = *ecm.component_store().get("margin", entity).unwrap();
        let constraint = *ecm
            .component_store()
            .get::<Constraint>("constraint", entity)
            .unwrap();
        let orientation: Orientation = *ecm.component_store().get("orientation", entity).unwrap();
        let mut size_counter = 0.0;

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
            .get_mut::<Rectangle>("bounds", entity)
        {
            bounds.set_width(size.0);
            bounds.set_height(size.1);
        }

        let available_size = size;
        let nchildren = ecm.entity_store().children[&entity].len();
        let spacing = spacing(ecm, entity);

        for index in 0..nchildren {
            let child = ecm.entity_store().children[&entity][index];

            let mut child_desired_size = (0.0, 0.0);
            if let Some(child_layout) = layouts.get(&child) {
                match orientation {
                    Orientation::Horizontal => {
                        // set width to 0.0 to shrink width of the child
                        child_desired_size = child_layout.arrange(
                            render_context_2_d,
                            (0.0, size.1),
                            child,
                            ecm,
                            layouts,
                            theme,
                        );
                    }
                    // set height to 0.0 to shrink height of the child
                    Orientation::Vertical => {
                        child_desired_size = child_layout.arrange(
                            render_context_2_d,
                            (size.0, 0.0),
                            child,
                            ecm,
                            layouts,
                            theme,
                        );
                    }
                }
            }

            let mut child_margin = {
                if child_desired_size.0 > 0.0 && child_desired_size.1 > 0.0 {
                    *ecm.component_store()
                        .get::<Thickness>("margin", child)
                        .unwrap()
                } else {
                    Thickness::default()
                }
            };

            if spacing != 0.0 && nchildren > 1 {
                apply_spacing(&mut child_margin, spacing, orientation, index, nchildren);
            }

            let child_horizontal_alignment: Alignment = *ecm
                .component_store()
                .get("horizontal_alignment", child)
                .unwrap();
            let child_vertical_alignment: Alignment = *ecm
                .component_store()
                .get("vertical_alignment", child)
                .unwrap();

            if let Ok(child_bounds) = ecm
                .component_store_mut()
                .get_mut::<Rectangle>("bounds", child)
            {
                match orientation {
                    Orientation::Horizontal => {
                        child_bounds.set_x(
                            size_counter
                                + child_horizontal_alignment.align_position(
                                    available_size.0,
                                    child_bounds.width(),
                                    child_margin.left(),
                                    child_margin.right(),
                                ),
                        );
                        child_bounds.set_y(child_vertical_alignment.align_position(
                            available_size.1,
                            child_bounds.height(),
                            child_margin.top(),
                            child_margin.bottom(),
                        ));
                        size_counter +=
                            child_bounds.width() + child_margin.left() + child_margin.right();
                    }
                    _ => {
                        child_bounds.set_x(child_horizontal_alignment.align_position(
                            available_size.0,
                            child_bounds.width(),
                            child_margin.left(),
                            child_margin.right(),
                        ));
                        child_bounds.set_y(
                            size_counter
                                + child_vertical_alignment.align_position(
                                    available_size.1,
                                    child_bounds.height(),
                                    child_margin.top(),
                                    child_margin.bottom(),
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

/// Applies spacing to widgets in a stack, depending upon their position, and the orientation.
fn apply_spacing(margins: &mut Thickness, spacing: f64, orientation: Orientation, index: usize, nchildren: usize) {
    let start =  if index == 0 { 0.0 } else { spacing / 2.0 };
    let end = if index == nchildren - 1 { 0.0 } else { spacing / 2.0 };

    match orientation {
        Orientation::Vertical => {
            margins.top += start;
            margins.bottom += end;
        }
        Orientation::Horizontal => {
            margins.left += start;
            margins.right += end;
        }
    }
}

/// Fetch the spacing property, which is guaranteed to exist on a stack.
fn spacing(ecm: &mut EntityComponentManager<Tree, StringComponentStore>, entity: Entity) -> f64 {
    ecm.component_store()
        .get::<f64>("spacing", entity)
        .expect("stack layout missing spacing property")
        .clone()
}