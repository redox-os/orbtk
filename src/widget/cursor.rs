use layout_object::TextSelectionLayoutObject;
use render_object::RectangleRenderObject;
use theme::Selector;
use widget::{Template, Widget};
use properties::{TextSelection, Label};

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
            .with_debug_name("Cursor")
    }
}
