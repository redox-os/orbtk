use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
};

use dces::prelude::*;

use crate::{
    prelude::*, proc_macros::IntoLayout, render::RenderContext2D, theming::*, tree::Tree,
    utils::prelude::*,
};

use super::{component, component_or_default, component_try_mut, Layout};

/// Stacks visual the children widgets vertical or horizontal.
#[derive(Default, IntoLayout)]
pub struct StackLayout {
    desired_size: RefCell<DirtySize>,
    old_alignment: Cell<(Alignment, Alignment)>,
}

impl StackLayout {
    /// Presets the defaults.
    pub fn new() -> Self {
        StackLayout::default()
    }

    pub fn set_dirty(&self, dirty: bool) {
        self.desired_size.borrow_mut().set_dirty(dirty);
    }
}

impl Layout for StackLayout {
    fn measure(
        &self,
        render_context_2_d: &mut RenderContext2D,
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree>,
        layouts: &BTreeMap<Entity, Box<dyn Layout>>,
        theme: &Theme,
    ) -> DirtySize {
        if component::<Visibility>(ecm, entity, "visibility") == Visibility::Collapsed {
            let mut desired = self.desired_size.borrow_mut();
            desired.set_size(0.0, 0.0);
            return *desired;
        }

        let halign: Alignment = component(ecm, entity, "h_align");
        let valign: Alignment = component(ecm, entity, "v_align");
        let (old_valign, old_halign) = self.old_alignment.get();

        if halign != old_halign || valign != old_valign {
            self.set_dirty(true);
        }

        let orientation: Orientation = component(ecm, entity, "orientation");
        let mut dirty = false;
        let mut desired_size: (f64, f64) = (0.0, 0.0);

        let nchildren = ecm.entity_store().children[&entity].len();
        let spacing: f64 = component_or_default(ecm, entity, "spacing");

        for index in 0..nchildren {
            let child = ecm.entity_store().children[&entity][index];

            if let Some(child_layout) = layouts.get(&child) {
                let child_desired_size =
                    child_layout.measure(render_context_2_d, child, ecm, layouts, theme);

                let mut child_margin = {
                    if child_desired_size.width() > 0.0 && child_desired_size.height() > 0.0 {
                        component(ecm, child, "margin")
                    } else {
                        Thickness::default()
                    }
                };

                if spacing != 0.0 && nchildren > 1 {
                    apply_spacing(&mut child_margin, spacing, orientation, index, nchildren);
                }

                accumulate_desired_size(
                    &mut desired_size,
                    child_desired_size,
                    child_margin,
                    orientation,
                );

                if child_desired_size.dirty() || self.desired_size.borrow().dirty() {
                    dirty = true;
                }
            }
        }

        self.set_dirty(dirty);

        let mut desired = self.desired_size.borrow_mut();
        desired.set_size(desired_size.0, desired_size.1);
        *desired
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

        let halign: Alignment = component(ecm, entity, "h_align");
        let valign: Alignment = component(ecm, entity, "v_align");
        let margin: Thickness = component(ecm, entity, "margin");
        let constraint: Constraint = component(ecm, entity, "constraint");
        let orientation: Orientation = component(ecm, entity, "orientation");

        let mut size_counter = 0.0;

        let mut size = constraint.perform((
            halign.align_measure(
                parent_size.0,
                self.desired_size.borrow().width(),
                margin.left(),
                margin.right(),
            ),
            valign.align_measure(
                parent_size.1,
                self.desired_size.borrow().height(),
                margin.top(),
                margin.bottom(),
            ),
        ));

        let available_size = size;
        let nchildren = ecm.entity_store().children[&entity].len();
        let spacing: f64 = component_or_default(ecm, entity, "spacing");

        for index in 0..nchildren {
            let child = ecm.entity_store().children[&entity][index];

            match orientation {
                Orientation::Horizontal => {
                    if let Some(halign) = component_try_mut::<Alignment>(ecm, child, "h_align") {
                        *halign = Alignment::Start;
                    }
                }
                _ => {
                    if let Some(halign) = component_try_mut::<Alignment>(ecm, child, "v_align") {
                        *halign = Alignment::Start;
                    }
                }
            }

            let mut child_desired_size = (0.0, 0.0);
            if let Some(child_layout) = layouts.get(&child) {
                child_desired_size =
                    child_layout.arrange(render_context_2_d, size, child, ecm, layouts, theme);
            }

            let mut child_margin = {
                if child_desired_size.0 > 0.0 && child_desired_size.1 > 0.0 {
                    component(ecm, child, "margin")
                } else {
                    Thickness::default()
                }
            };

            if spacing != 0.0 && nchildren > 1 {
                apply_spacing(&mut child_margin, spacing, orientation, index, nchildren);
            }

            let child_halign: Alignment = component(ecm, child, "h_align");
            let child_valign: Alignment = component(ecm, child, "v_align");

            if let Some(child_bounds) = component_try_mut::<Rectangle>(ecm, child, "bounds") {
                apply_arrangement(
                    child_bounds,
                    &mut size_counter,
                    child_margin,
                    (child_halign, child_valign),
                    orientation,
                    available_size,
                );
            }

            mark_as_dirty("bounds", child, ecm);
        }

        self.set_dirty(false);

        match orientation {
            Orientation::Horizontal => {
                size.0 = size_counter;
            }
            _ => {
                size.1 = size_counter;
            }
        }

        if let Some(bounds) = component_try_mut::<Rectangle>(ecm, entity, "bounds") {
            bounds.set_width(size.0);
            bounds.set_height(size.1);
        }

        mark_as_dirty("bounds", entity, ecm);

        size
    }
}

fn apply_arrangement(
    bounds: &mut Rectangle,
    size_counter: &mut f64,
    margin: Thickness,
    alignment: (Alignment, Alignment),
    orientation: Orientation,
    available_size: (f64, f64),
) {
    let (xpos, ypos, size);

    match orientation {
        Orientation::Horizontal => {
            xpos = *size_counter
                + alignment.0.align_position(
                    available_size.0,
                    bounds.width(),
                    margin.left(),
                    margin.right(),
                );

            ypos = alignment.1.align_position(
                available_size.1,
                bounds.height(),
                margin.top(),
                margin.bottom(),
            );

            size = bounds.width() + margin.left() + margin.right();
        }
        _ => {
            xpos = alignment.0.align_position(
                available_size.0,
                bounds.width(),
                margin.left(),
                margin.right(),
            );

            ypos = *size_counter
                + alignment.1.align_position(
                    available_size.1,
                    bounds.height(),
                    margin.top(),
                    margin.bottom(),
                );

            size = bounds.height() + margin.top() + margin.bottom();
        }
    };

    bounds.set_x(xpos);
    bounds.set_y(ypos);
    *size_counter += size;
}

/// Applies spacing to widgets in a stack, depending upon their position, and the orientation.
fn apply_spacing(
    margins: &mut Thickness,
    spacing: f64,
    orientation: Orientation,
    index: usize,
    nchildren: usize,
) {
    let start = if index == 0 { 0.0 } else { spacing / 2.0 };
    let end = if index == nchildren - 1 {
        0.0
    } else {
        spacing / 2.0
    };

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

fn accumulate_desired_size(
    desired_size: &mut (f64, f64),
    desired: DirtySize,
    margin: Thickness,
    orientation: Orientation,
) {
    let width = desired.width() + margin.left() + margin.right();
    let height = desired.height() + margin.top() + margin.bottom();

    match orientation {
        Orientation::Horizontal => {
            desired_size.0 += width;
            desired_size.1 = desired_size.1.max(height);
        }
        Orientation::Vertical => {
            desired_size.0 = desired_size.0.max(width);
            desired_size.1 += height;
        }
    }
}

#[cfg(test)]
mod tests {
    use orbtk_utils::{Orientation, Thickness};
    use std::iter;

    use super::apply_spacing;

    const NUM_WIDGETS: usize = 5;

    #[test]
    fn spacing_vertical() {
        let expected = iter::once(Thickness {
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: 2.0,
        })
        .chain(
            iter::repeat(Thickness {
                left: 0.0,
                right: 0.0,
                top: 2.0,
                bottom: 2.0,
            })
            .take(3),
        )
        .chain(iter::once(Thickness {
            left: 0.0,
            right: 0.0,
            top: 2.0,
            bottom: 0.0,
        }));

        spacing(Orientation::Vertical, 4.0, expected);
    }

    #[test]
    fn spacing_horizontal() {
        let expected = iter::once(Thickness {
            left: 0.0,
            right: 4.0,
            top: 0.0,
            bottom: 0.0,
        })
        .chain(
            iter::repeat(Thickness {
                left: 4.0,
                right: 4.0,
                top: 0.0,
                bottom: 0.0,
            })
            .take(3),
        )
        .chain(iter::once(Thickness {
            left: 4.0,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
        }));

        spacing(Orientation::Horizontal, 8.0, expected);
    }

    fn spacing(orientation: Orientation, spacing: f64, expected: impl Iterator<Item = Thickness>) {
        let inputs = (0..NUM_WIDGETS).map(|id| (id, Thickness::default()));

        for ((index, mut input), expected) in inputs.zip(expected) {
            apply_spacing(&mut input, spacing, orientation, index, NUM_WIDGETS);
            assert_eq!(input, expected);
        }
    }
}
