use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    iter,
};

use dces::prelude::*;
use memchr::memchr_iter;
use smallvec::SmallVec;

use crate::{
    proc_macros::IntoLayout,
    render::Image,
    render::RenderContext2D,
    theming::*,
    tree::Tree,
    utils::prelude::*,
    widget_base::{mark_as_dirty, WidgetContainer},
};

use super::{component, component_try_mut, Layout};

/// Fixed size layout is defined by fixed bounds like the size of an image or the size of a text.
#[derive(Default, IntoLayout)]
pub struct FixedSizeLayout {
    desired_size: RefCell<DirtySize>,
    old_alignment: Cell<(Alignment, Alignment)>,
}

impl FixedSizeLayout {
    pub fn new() -> Self {
        FixedSizeLayout::default()
    }
}

impl Layout for FixedSizeLayout {
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

        let widget = WidgetContainer::new(entity, ecm, theme, None);

        let horizontal_alignment: Alignment = *widget.get("h_align");
        let vertical_alignment: Alignment = *widget.get("v_align");

        if horizontal_alignment != self.old_alignment.get().1
            || vertical_alignment != self.old_alignment.get().0
        {
            self.desired_size.borrow_mut().set_dirty(true);
        }

        let size = widget
            .try_get::<Image>("image")
            .map(|image| (Size::new(image.width(), image.height())))
            .or_else(|| {
                text(&widget).and_then(|text| {
                    let font = widget.get::<String>("font");
                    let font_size = widget.get::<f64>("font_size");

                    if text.is_empty() {
                        widget
                            .try_get::<String>("water_mark")
                            .filter(|water_mark| !water_mark.is_empty())
                            .map(|water_mark| {
                                measure_text(
                                    render_context_2_d,
                                    &water_mark,
                                    font.as_str(),
                                    *font_size,
                                )
                            })
                    } else {
                        Some(measure_text(
                            render_context_2_d,
                            &text,
                            font.as_str(),
                            *font_size,
                        ))
                    }
                })
            })
            .or_else(|| {
                widget
                    .try_clone::<String>("icon")
                    .filter(|font_icon| !font_icon.is_empty())
                    .map(|font_icon| {
                        let icon_size = widget.get::<f64>("icon_size");
                        measure_text(
                            render_context_2_d,
                            &font_icon,
                            widget.get::<String>("icon_font").as_str(),
                            *icon_size,
                        )
                    })
            });

        if let Some(size) = size {
            if let Some(constraint) = component_try_mut::<Constraint>(ecm, entity, "constraint") {
                constraint.set_width(size.width());
                constraint.set_height(size.height());
            }
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

        for index in 0..ecm.entity_store().children[&entity].len() {
            let child = ecm.entity_store().children[&entity][index];
            if let Some(child_layout) = layouts.get(&child) {
                let dirty = child_layout
                    .measure(render_context_2_d, child, ecm, layouts, theme)
                    .dirty()
                    || self.desired_size.borrow().dirty();

                self.desired_size.borrow_mut().set_dirty(dirty);
            }
        }

        *self.desired_size.borrow()
    }

    fn arrange(
        &self,
        render_context_2_d: &mut RenderContext2D,
        _parent_size: (f64, f64),
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree>,
        layouts: &BTreeMap<Entity, Box<dyn Layout>>,
        theme: &Theme,
    ) -> (f64, f64) {
        if component::<Visibility>(ecm, entity, "visibility") == Visibility::Collapsed {
            self.desired_size.borrow_mut().set_size(0.0, 0.0);
            return (0.0, 0.0);
        }

        if let Some(bounds) = component_try_mut::<Rectangle>(ecm, entity, "bounds") {
            bounds.set_width(self.desired_size.borrow().width());
            bounds.set_height(self.desired_size.borrow().height());
        }

        mark_as_dirty("bounds", entity, ecm);

        for index in 0..ecm.entity_store().children[&entity].len() {
            let child = ecm.entity_store().children[&entity][index];
            if let Some(child_layout) = layouts.get(&child) {
                child_layout.arrange(
                    render_context_2_d,
                    (
                        self.desired_size.borrow().width(),
                        self.desired_size.borrow().height(),
                    ),
                    child,
                    ecm,
                    layouts,
                    theme,
                );
            }
        }

        self.desired_size.borrow_mut().set_dirty(false);
        self.desired_size.borrow().size()
    }
}

fn text(widget: &WidgetContainer) -> Option<String> {
    if let Some(localizable) = widget.try_get::<bool>("localizable") {
        if *localizable {
            if let Some(localized_text) = widget.try_get::<String>("localized_text") {
                return Some(localized_text.clone());
            }
        }
    }

    if let Some(text) = widget.try_get::<String>("text") {
        return Some(text.clone());
    }

    None
}

fn measure_text(
    render_context_2_d: &mut RenderContext2D,
    text: &str,
    font_family: &str,
    font_size: f64,
) -> Size {
    let mut text_metrics = render_context_2_d.measure(text, font_size, font_family);
    let mut lines = SmallVec::<[&str; 2]>::new();
    let mut last_hit = 0;
    for hit in memchr_iter(b'\n', text.as_bytes()).chain(iter::once(text.len())) {
        let line = &text[last_hit..hit];
        lines.push(line);
        last_hit = hit;
    }
    text_metrics.width = 0.0;
    for line in lines.iter() {
        let line_text_metrics = render_context_2_d.measure(line, font_size, font_family);
        text_metrics.width = text_metrics.width.max(line_text_metrics.width);
    }
    Size::new(
        text_metrics.width,
        text_metrics.height * (((lines.len() - 1) as f64) * 1.15 + 1.0),
    )
}
