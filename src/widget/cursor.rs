// use std::rc::Rc;

// use crate::{
//     layout::TextSelectionLayout,
//     properties::{
//         BackgroundProperty, FocusedProperty, OffsetProperty, TextProperty, TextSelection,
//         TextSelectionProperty, FontProperty, FontSizeProperty
//     },
//     render_object::RectangleRenderObject,
//     widget::{
//         add_selector_to_widget, remove_selector_from_widget, Context, State, Template, Widget,
//     },
//     styling::fonts
// };

use dces::prelude::Entity;

use crate::{
    layout::{TextSelectionLayout, Layout},
    properties::*,
    render_object::{RenderObject, RectangleRenderObject},
    styling::fonts,
    widget::{Template, State, Context, add_selector_to_widget, remove_selector_from_widget},
};

// Default state of the `Cursor` widget.
#[derive(Default)]
struct CursorState;

impl State for CursorState {
    fn update(&self, context: &mut Context<'_>) {
        let mut widget = context.widget();

        let selection_length = widget.property::<TextSelection>().0.length;

        if selection_length > 0 {
            add_selector_to_widget("expanded", &mut widget);
        } else {
            remove_selector_from_widget("expanded", &mut widget)
        }
    }
}

widget!(
    /// The `Cursor` widget represents a text cursor used to mark text.
    /// 
    /// * CSS element: `cursor`
    Cursor<CursorState> {
        /// Sets or shares the text property.
        text: Text,

        /// Sets or shares the text selection property.
        selection: TextSelection,

        /// Sets or shares the background property.
        background: Background,

        /// Sets or share the font size property.
        font_size: FontSize,

        /// Sets or shares the font property.
        font: Font,

        /// Sets or shares the text offset property.
        text_offset: Offset,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl Template for Cursor {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Cursor")
            .width(1.0)
            .selector("cursor")
            .text("")
            .text_offset(0.0)
            .background("transparent")
            .font_size(fonts::FONT_SIZE_12)
            .font(fonts::font_into_box(fonts::ROBOTO_REGULAR_FONT))
    }

    fn render_object(&self) -> Option<Box<dyn RenderObject>> {
        Some(Box::new(RectangleRenderObject))
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(TextSelectionLayout::default())
    }
}
