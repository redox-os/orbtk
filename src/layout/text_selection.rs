use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager};

use crate::prelude::*;
use crate::backend::FontMeasure;

use super::Layout;

/// The text selection layout is used to measure and arrange a text selection cursor.
#[derive(Default)]
pub struct TextSelectionLayout {
    desired_size: RefCell<DirtySize>,
    old_text_selection: Cell<TextSelectionValue>,
}

impl TextSelectionLayout {
    pub fn new() -> Self {
        TextSelectionLayout::default()
    }
}

impl Into<Box<dyn Layout>> for TextSelectionLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}

impl Layout for TextSelectionLayout {
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

        let constraint = Constraint::get(entity, ecm);

        if let Ok(selection) = ecm.borrow_component::<TextSelection>(entity) {
            if selection.0 != self.old_text_selection.get() {
                self.desired_size.borrow_mut().set_dirty(true);
            }

            self.old_text_selection.set(selection.0);
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
        parent_size: (f64, f64),
        entity: Entity,
        ecm: &mut EntityComponentManager,
        tree: &Tree,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &Theme,
    ) -> (f64, f64) {
        if !self.desired_size.borrow().dirty() {
            return self.desired_size.borrow().size();
        }

        let mut pos = 0.0;
        let mut size = self.desired_size.borrow().size();

        let vertical_alignment = VerticalAlignment::get(entity, ecm);
        let margin = Margin::get(entity, ecm);
        {
            let mut widget = WidgetContainer::new(entity, ecm);

            size.1 = vertical_alignment.align_measure(parent_size.1, size.1, margin.top(), margin.bottom());

            if let Some(text) = widget.try_get::<Text>() {
                let font = widget.get::<Font>();
                let font_size = widget.get::<FontSize>();

                if let Some(selection) = widget.try_get::<TextSelection>() {
                    if let Some(text_part) = text.0.get(0..selection.0.start_index) {
                        pos = crate::FONT_MEASURE
                            .measure(text_part, &(font.0).0, font_size.0 as u32)
                            .0 as f64;

                        if text_part.ends_with(" ") {
                            pos +=
                                (crate::FONT_MEASURE.measure("a", &(font.0).0, font_size.0 as u32).0 / 2) as f64;
                        }
                    }
                }
            }

            pos += widget.try_get::<Offset>().map_or(0.0, |off| (off.0).0);

            if let Some(margin) = widget.try_get_mut::<Margin>() {
                margin.set_left(pos);
            }

            if let Some(bounds) = widget.try_get_mut::<Bounds>() {
                bounds.set_width(size.0);
                bounds.set_height(size.1);
            }
        }

        for child in &tree.children[&entity] {
            if let Some(child_layout) = layouts.borrow().get(child) {
                child_layout.arrange(size, *child, ecm, tree, layouts, theme);
            }
        }

        self.desired_size.borrow_mut().set_dirty(false);
        size
    }
}
