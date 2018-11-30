use layout_object::FontIconSizeLayoutObject;
use render_object::FontIconRenderObject;
use properties::FontIcon;
use theme::Selector;
use widget::{Template, Widget};

/// The `FontIconBlock` widget is used to draw an font icon. It is not interactive.
/// 
/// # Properties
/// 
/// * `FontIcon` - String used to display the icon of the font icon block.
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
            .with_property(FontIcon::default)
            .with_property(Selector::new().with("fonticon"))
            .with_layout_object(FontIconSizeLayoutObject)
            .with_render_object(FontIconRenderObject)
            .with_debug_name("FontIconBlock")
    }
}
