use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::Entity;

use crate::{prelude::*, render::RenderContext2D, tree::Tree, utils::prelude::*};

use super::Layout;

/// Fixed size layout is defined by fixed bounds like the size of an image or the size of a text.
#[derive(Default)]
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
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &ThemeValue,
    ) -> DirtySize {
        if *ecm.component_store().borrow_component::<Visibility>("visibility", entity).unwrap() == Visibility::Collapsed {
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

        let widget = WidgetContainer::new(entity, ecm, theme);

        let size = widget
            .try_get::<Image>("image")
            .map(|image| (image.width(), image.height()))
            .or_else(|| {
                widget.try_get::<String16>("text").and_then(|text| {
                    let font = widget.get::<String>("font");
                    let font_size = widget.get::<f64>("font_size");
                    // render_context_2_d.set_font_size(font_size);
                    // render_context_2_d.set_font_family(font.as_str());

                    if text.is_empty() {
                        widget
                            .try_get::<String16>("water_mark")
                            .filter(|water_mark| !water_mark.is_empty())
                            .map(|water_mark| {
                                let text_metrics = render_context_2_d.measure(
                                    water_mark.to_string().as_str(),
                                    *font_size,
                                    font.as_str(),
                                );
                                (text_metrics.width, text_metrics.height)
                            })
                    } else {
                        let text_metrics = render_context_2_d.measure(
                            text.to_string().as_str(),
                            *font_size,
                            font.as_str(),
                        );

                        Some((text_metrics.width, text_metrics.height))
                    }
                })
            })
            .or_else(|| {
                widget
                    .try_clone::<String>("icon")
                    .filter(|font_icon| !font_icon.is_empty())
                    .map(|font_icon| {
                        let icon_size = widget.get::<f64>("icon_size");
                        let text_metrics = render_context_2_d.measure(
                            &font_icon,
                            *icon_size,
                            widget.get::<String>("icon_font").as_str(),
                        );
                        (text_metrics.width, text_metrics.height)
                    })
            });

        if let Some(size) = size {
            if let Ok(constraint) = ecm
                .component_store_mut()
                .borrow_mut_component::<Constraint>("constraint", entity)
            {
                constraint.set_width(size.0 as f64);
                constraint.set_height(size.1 as f64);
            }
        }

        // -- todo will be removed after orbgl merge --

        let constraint = Constraint::get("constraint", entity, ecm.component_store());

        if constraint.width() > 0.0 {
            self.desired_size.borrow_mut().set_width(constraint.width());
        }

        if constraint.height() > 0.0 {
            self.desired_size
                .borrow_mut()
                .set_height(constraint.height());
        }

        if ecm.entity_store().children[&entity].len() > 0 {
            let mut index = 0;

            loop {
                let child = ecm.entity_store().children[&entity][index];
                if let Some(child_layout) = layouts.borrow().get(&child) {
                    let dirty = child_layout
                        .measure(render_context_2_d, child, ecm, layouts, theme)
                        .dirty()
                        || self.desired_size.borrow().dirty();

                    self.desired_size.borrow_mut().set_dirty(dirty);
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
        _parent_size: (f64, f64),
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &ThemeValue,
    ) -> (f64, f64) {
        if !self.desired_size.borrow().dirty() {
            return self.desired_size.borrow().size();
        }

        if let Ok(bounds) = ecm
            .component_store_mut()
            .borrow_mut_component::<Bounds>("bounds", entity)
        {
            bounds.set_width(self.desired_size.borrow().width());
            bounds.set_height(self.desired_size.borrow().height());
        }

        if ecm.entity_store().children[&entity].len() > 0 {
            let mut index = 0;

            loop {
                let child = ecm.entity_store().children[&entity][index];
                if let Some(child_layout) = layouts.borrow().get(&child) {
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

                if index + 1 < ecm.entity_store().children[&entity].len() {
                    index += 1;
                } else {
                    break;
                }
            }
        }

        self.desired_size.borrow_mut().set_dirty(false);
        self.desired_size.borrow().size()
    }
}

impl Into<Box<dyn Layout>> for FixedSizeLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}
