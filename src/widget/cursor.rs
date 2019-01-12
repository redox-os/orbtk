use std::rc::Rc;

use crate::{
    core::Rectangle,
    layout::TextSelectionLayout,
    properties::{Label, TextSelection},
    // render_object::RectangleRenderObject,
    theme::Selector,
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
/// * `TextSelection` - Represents the current selection of the text used by the cursor.
///
/// # Others
///
/// * `RectangleRenderObject` - Used to draw the widget.
/// * `TextSelectionLayout` - Used to layout the widget.
pub struct Cursor;

impl Widget for Cursor {
    fn create() -> Template {
        Template::default()
            .with_property(Label::default())
            .with_property(Selector::from("cursor"))
            .with_property(TextSelection::default())
            .with_property(Rectangle::default())
            .with_layout(TextSelectionLayout)
            .with_state(Rc::new(CursorState))
            .with_debug_name("Cursor")
    }
}
