use crate::layout_object::FixedSizeLayoutObject;
use crate::widget::{Template, Widget};
use crate::theme::Selector;

/// The `Space` is used to create empty space inside of a `Row` or `Column`.
/// 
/// # Shared Properties
///
/// * `Selector` - CSS selector with  element name `spacer`, used to request the theme of the widget.
/// 
/// # Others
/// 
/// * `ParentType`- None.
pub struct Spacer;

impl Widget for Spacer {
    fn create() -> Template {
        Template::default()
            .with_property(Selector::from("spacer"))
            .with_layout_object(FixedSizeLayoutObject::default())
            .with_debug_name("Spacer")
    }
}
