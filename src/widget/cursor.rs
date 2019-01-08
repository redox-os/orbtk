use std::rc::Rc;

use crate::layout_object::TextSelectionLayoutObject;
use crate::properties::{Label, TextSelection};
use crate::render_object::RectangleRenderObject;
use crate::theme::Selector;
use crate::widget::{
    add_selector_to_widget, remove_selector_from_widget, Context, State, Template, Widget,
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
