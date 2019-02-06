use std::rc::Rc;

use crate::{
    layout::TextSelectionLayout,
    properties::{
        FocusedProperty, OffsetProperty, TextProperty, TextSelection,
        TextSelectionProperty,
    },
    render_object::RectangleRenderObject,
    widget::{
        add_selector_to_widget, remove_selector_from_widget, Context, State, Template, Widget,
    },
};

// Default state of the `Cursor` widget.
struct CursorState;

impl State for CursorState {
    fn update(&self, context: &mut Context<'_>) {
        let mut selection_length = 0;
        let mut widget = context.widget();

        if let Ok(selection) = widget.borrow_property::<TextSelection>() {
            selection_length = selection.length;
        }

        if selection_length > 0 {
            add_selector_to_widget("expanded", &mut widget);
        } else {
            remove_selector_from_widget("expanded", &mut widget)
        }
    }
}

/// The `Cursor` represents a text cursor used to mark text.
///
/// # Properties
///
/// * `text_selection` - Represents the current selection of the text used by the cursor.
/// * `text` - Used to set the text of the cursor.
///
/// # Others
///
/// * `RectangleRenderObject` - Used to draw the widget.
/// * `TextSelectionLayout` - Used to layout the widget.
pub struct Cursor;

impl Widget for Cursor {
    type Template = CursorTemplate;

    fn create() -> Self::Template {
        CursorTemplate::new()
            .width(1.0)
            .text("")
            .selector("cursor")
            .offset(0.0)
            .text_selection(TextSelection::default())
            .render_object(RectangleRenderObject)
            .layout(TextSelectionLayout::new())
            .state(Rc::new(CursorState))
            .debug_name("Cursor")
    }
}

template!(
    CursorTemplate,
    [
        TextProperty,
        TextSelectionProperty,
        OffsetProperty,
        FocusedProperty
    ]
);
