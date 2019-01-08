
use crate::layout_object::PaddingLayoutObject;
use crate::render_object::{RectangleRenderObject};
use crate::theme::Selector;
use crate::widget::{Template, Widget};
use crate::enums::ParentType;

/// The `Container` represents a layout that surrounds its child with a padding. Draws a box arround the child.
/// 
/// # Properties
/// 
/// * `Selector` - CSS selector with element name `container`, used to request the theme of the widget.
/// 
/// # Others
/// 
/// * `ParentType`- Single.
/// * `PaddingLayoutObject` - Used to layout the widget.
/// * `RectangleRenderObject` - Used to draw the widget.
pub struct Container;

impl Widget for Container {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_property(Selector::from("container"))
            .with_render_object(RectangleRenderObject)
            .with_layout_object(PaddingLayoutObject) 
            .with_debug_name("Container")
    }
}
