use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

use dces::prelude::Entity;

use crate::{prelude::*, render::RenderContext2D, tree::Tree, utils::prelude::*};

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

        let constraint = Constraint::get("constraint", entity, ecm.component_store());

        if let Ok(selection) = ecm
            .component_store()
            .borrow_component::<TextSelection>("text_selection", entity)
        {
            if selection.0 != self.old_text_selection.get() {
                self.desired_size.borrow_mut().set_dirty(true);
            }

            self.old_text_selection.set(selection.0);
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
        parent_size: (f64, f64),
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree, StringComponentStore>,
        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &ThemeValue,
    ) -> (f64, f64) {
        if !self.desired_size.borrow().dirty() {
            return self.desired_size.borrow().size();
        }

        let mut pos = 0.0;
        let mut size = self.desired_size.borrow().size();

        let vertical_alignment = VerticalAlignment::get("vertical_alignment", entity, ecm.component_store());
        let margin = Margin::get("margin", entity, ecm.component_store());

        {
            let mut widget = WidgetContainer::new(entity, ecm, &theme);

            size.1 = vertical_alignment.align_measure(
                parent_size.1,
                size.1,
                margin.top(),
                margin.bottom(),
            );

            if let Some(text) = widget.try_get::<String16>("text") {
                let font = widget.get::<Font>("font");
                let font_size = widget.get::<FontSize>("font_size");
                // render_context_2_d.set_font_size(font_size.0);
                // render_context_2_d.set_font_family(&font.0[..]);

                if let Some(selection) = widget.try_get::<TextSelection>("text_selection") {
                    if let Some(text_part) = text.get_string(0, selection.0.start_index) {
                        pos = render_context_2_d
                            .measure(text_part.as_str(), font_size.0, &font.0[..])
                            .width;
                    }
                }
            }

            pos += widget
                .try_get::<ScrollOffset>("scroll_offset")
                .map_or(0.0, |off| (off.0).x);

            if let Some(margin) = widget.try_get_mut::<Margin>("margin") {
                margin.set_left(pos);
            }

            if let Some(bounds) = widget.try_get_mut::<Bounds>("bounds") {
                bounds.set_width(size.0);
                bounds.set_height(size.1);
            }
        }

        if ecm.entity_store().children[&entity].len() > 0 {
            let mut index = 0;

            loop {
                let child = ecm.entity_store().children[&entity][index];

                if let Some(child_layout) = layouts.borrow().get(&child) {
                    child_layout.arrange(render_context_2_d, size, child, ecm, layouts, theme);
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
