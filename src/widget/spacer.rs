use crate::{
    layout::FixedSizeLayout,
    theme::Selector,
    widget::{Template, Widget},
};

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
            .with_layout(FixedSizeLayout::default())
            .with_debug_name("Spacer")
    }
}
