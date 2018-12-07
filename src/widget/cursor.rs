use std::rc::Rc;

use layout_object::TextSelectionLayoutObject;
use properties::{Label, TextSelection};
use render_object::RectangleRenderObject;
use theme::Selector;
use widget::{
    add_selector_to_widget, remove_selector_from_widget, Context, State, Template, Widget,
};

// Default state of the `Cursor` widget.
struct CursorState;

impl State for CursorState {
    fn update(&self, context: &mut Context) {
        let mut selection_length = 0;
        if let Ok(selection) = context.widget.borrow_property::<TextSelection>() {
           selection_length = selection.length;
        }

        if selection_length > 0 {
            add_selector_to_widget("expanded", context.widget);
        } else {
            remove_selector_from_widget("expanded", context.widget)
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
/// * `TextSelectionLayoutObject` - Used to layout the widget.
pub struct Cursor;

impl Widget for Cursor {
    fn create() -> Template {
        Template::default()
            .with_property(Label::default())
            .with_property(Selector::from("cursor"))
            .with_property(TextSelection::default())
            .with_render_object(RectangleRenderObject)
            .with_layout_object(TextSelectionLayoutObject)
            .with_state(Rc::new(CursorState))
            .with_debug_name("Cursor")
    }
}
