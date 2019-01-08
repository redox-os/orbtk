use crate::layout_object::FontIconSizeLayoutObject;
use crate::render_object::FontIconRenderObject;
use crate::theme::Selector;
use crate::widget::{Template, Widget};

/// The `FontIconBlock` widget is used to draw an font icon. It is not interactive.
///
/// # Properties
///
/// * `Selector` - CSS selector with  element name `fonticon`, used to request the theme of the font icon block.
///
/// # Others
///
/// * `ParentType`- None.
/// * `FontIconSizeLayoutObject` - Used to layout the widget.
/// * `FontIconRenderObject` - Used to draw the text of the widget.
pub struct FontIconBlock;

impl Widget for FontIconBlock {
    fn create() -> Template {
        Template::default()
            .with_property(Selector::from("fonticon"))
            .with_layout_object(FontIconSizeLayoutObject)
            .with_render_object(FontIconRenderObject)
            .with_debug_name("FontIconBlock")
    }
}
