use dces::prelude::Entity;

use crate::{
    layout::{TextSelectionLayout, Layout},
    properties::*,
    render_object::{RenderObject, RectangleRenderObject},
    styling::fonts,
    widgets::{Template, State, Context, add_selector_to_widget, remove_selector_from_widget},
};

// Default state of the `Cursor` widget.
#[derive(Default)]
pub struct CursorState;

impl State for CursorState {
    fn update(&self, context: &mut Context<'_>) {
        let mut widget = context.widget();

        let selection_length = widget.get::<TextSelection>().0.length;

        if selection_length > 0 {
            add_selector_to_widget("expanded", &mut widget);
        } else {
            remove_selector_from_widget("expanded", &mut widget)
        }

        if widget.get::<Focused>().0 {
            add_selector_to_widget("focus", &mut widget);
        } else {
            remove_selector_from_widget("focus", &mut widget)
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
        offset: Offset,

        /// Sets or shares the focused property.
        focused: Focused,

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
            .offset(0.0)
            .background("transparent")
            .font_size(fonts::FONT_SIZE_12)
            .font(fonts::font_into_box(fonts::ROBOTO_REGULAR_FONT))
            .focused(false)
    }

    fn render_object(&self) -> Option<Box<dyn RenderObject>> {
        Some(Box::new(RectangleRenderObject))
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(TextSelectionLayout::default())
    }
}
