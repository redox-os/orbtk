use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager};

use crate::{
    application::Tree,
    backend::{FontMeasure, FONT_MEASURE},
    properties::{
        Bounds, Constraint, Font, FontIcon, FontSize, HorizontalAlignment, IconFont, IconSize,
        Image, Text, VerticalAlignment, Visibility, VisibilityValue, WaterMark, ImageExtension, ConstraintExtension,
    },
    enums::Alignment,
    structs::{DirtySize, Size},
    theme::Theme,
    widget::WidgetContainer,
};

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
        entity: Entity,
        ecm: &mut EntityComponentManager,
        tree: &Tree,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &Theme,
    ) -> DirtySize {
        if Visibility::get(entity, ecm) == VisibilityValue::Collapsed {
            self.desired_size.borrow_mut().set_size(0.0, 0.0);
            return self.desired_size.borrow().clone();
        }

        let horizontal_alignment = HorizontalAlignment::get(entity, ecm);
        let vertical_alignment = VerticalAlignment::get(entity, ecm);

        if horizontal_alignment != self.old_alignment.get().1
            || vertical_alignment != self.old_alignment.get().0
        {
            self.desired_size.borrow_mut().set_dirty(true);
        }

        let widget = WidgetContainer::new(entity, ecm);

        // -- todo will be removed after orbgl merge --

        let size = {
            if widget.has::<Image>() {
                if let Ok(image) = widget.borrow::<Image>() {
                    Some((image.width(), image.height()))
                } else {
                    None
                }
            } else if widget.has::<Text>() {
                let text = widget.get::<Text>();
                let font = widget.get::<Font>();
                let font_size = widget.get::<FontSize>();

                if text.0.is_empty() {
                    if let Ok(water_mark) = ecm.borrow_component::<WaterMark>(entity) {
                        if water_mark.0.is_empty() {
                            None
                        } else {
                            Some(FONT_MEASURE.measure(&water_mark.0, &(font.0).0, font_size.0 as u32))
                        }
                    } else {
                        None
                    }
                } else {
                    let mut size = FONT_MEASURE.measure(&text.0, &(font.0).0, font_size.0 as u32);

                    if text.0.ends_with(" ") {
                        size.0 += FONT_MEASURE.measure("a", &(font.0).0, font_size.0 as u32).0 / 2;
                    }
                    Some(size)
                }
            } else if widget.has::<FontIcon>() {
                let font_icon = widget.get::<FontIcon>();
                if font_icon.0.is_empty() {
                    None
                } else {
                    Some(FONT_MEASURE.measure(
                        &font_icon.0,
                        &(widget.get::<IconFont>().0).0,
                        widget.get::<IconSize>().0 as u32,
                    ))
                }
            } else {
                None
            }
        };

        if let Some(size) = size {
            if let Ok(constraint) = ecm.borrow_mut_component::<Constraint>(entity) {
                constraint.set_width(size.0 as f64);
                constraint.set_height(size.1 as f64);
            }
        }

        // -- todo will be removed after orbgl merge --

        let constraint = Constraint::get(entity, ecm);

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
                    .measure(*child, ecm, tree, layouts, theme)
                    .dirty()
                    || self.desired_size.borrow().dirty();

                self.desired_size.borrow_mut().set_dirty(dirty);
            }
        }

        self.desired_size.borrow().clone()
    }

    fn arrange(
        &self,
        _parent_size: (f64, f64),
        entity: Entity,
        ecm: &mut EntityComponentManager,
        tree: &Tree,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &Theme,
    ) -> (f64, f64) {
        if !self.desired_size.borrow().dirty() {
            return self.desired_size.borrow().size();
        }

        if let Ok(bounds) = ecm.borrow_mut_component::<Bounds>(entity) {
            bounds.set_width(self.desired_size.borrow().width());
            bounds.set_height(self.desired_size.borrow().height());
        }

        for child in &tree.children[&entity] {
            if let Some(child_layout) = layouts.borrow().get(child) {
                child_layout.arrange(
                    (
                        self.desired_size.borrow().width(),
                        self.desired_size.borrow().height(),
                    ),
                    *child,
                    ecm,
                    tree,
                    layouts,
                    theme,
                );
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
