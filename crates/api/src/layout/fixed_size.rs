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

        let widget = WidgetContainer::new(entity, ecm, theme);

        let size = widget
            .try_get::<Image>("image")
            .map(|image| (image.width(), image.height()))
            .or_else(|| {
                widget.try_get::<Text>("text").and_then(|text| {
                    let font = widget.get::<Font>("font");
                    let font_size = widget.get::<FontSize>("font_size");
                    // render_context_2_d.set_font_size(font_size.0);
                    // render_context_2_d.set_font_family(&font.0[..]);

                    if text.0.is_empty() {
                        widget
                            .try_get::<WaterMark>("water_mark")
                            .filter(|water_mark| !water_mark.0.is_empty())
                            .map(|water_mark| {
                                let text_metrics = render_context_2_d.measure(
                                    water_mark.0.to_string().as_str(),
                                    font_size.0,
                                    &font.0[..],
                                );
                                (text_metrics.width, text_metrics.height)
                            })
                    } else {
                        let text_metrics = render_context_2_d.measure(
                            text.0.to_string().as_str(),
                            font_size.0,
                            &font.0[..],
                        );

                        let size = (text_metrics.width, text_metrics.height);

                        // if text.0.to_string().ends_with(" ") {
                        //     size.0 += render_context_2_d
                        //         .measure_text(&format!("{}a", text.0.to_string()))
                        //         .width
                        //         - render_context_2_d.measure_text("a").width;
                        // }
                        Some(size)
                    }
                })
            })
            .or_else(|| {
                widget
                    .try_clone::<FontIcon>("icon")
                    .filter(|font_icon| !font_icon.0.is_empty())
                    .map(|font_icon| {
                        let icon_size = widget.get::<IconSize>("icon_size").0;
                        // render_context_2_d.set_font_size(icon_size);
                        // render_context_2_d.set_font_family(&widget.get::<IconFont>().0[..]);
                        let text_metrics = render_context_2_d.measure(
                            &font_icon.0,
                            icon_size,
                            &widget.get::<IconFont>("icon_font").0[..],
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
