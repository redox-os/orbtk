use crate::{
    enums::ParentType,
    properties::{FontIcon, Text, Pressed},
    theme::Selector,
    widget::{Center, Container, FontIconBlock, Row, SharedProperty, Template, TextBlock, Widget},
};

/// The `Button` widget can be clicked by user. It's used to peform an action.
///
/// # Shared Properties
///
/// * `Text` - String used to display the text of the button.
/// * `FontIcon` - String used to display the font icon of the button.
/// * `Selector` - CSS selector with  element name `button`, used to request the theme of the widget.
///
/// # Properties
///
/// * `Pressed` - Bool value represents the pressed state of the button.
///
/// # Others
///
/// * `ParentType`- Single.
pub struct Button;

impl Widget for Button {
    fn create() -> Template {
        let text = SharedProperty::new(Text::default());
        let icon = SharedProperty::new(FontIcon::default());
        let selector = SharedProperty::new(Selector::from("button"));

        Template::new()
           .parent_type(ParentType::Single)
            .child(
                Container::create()
                    .shared_property(selector.clone())
                    .child(
                        Center::create().child(
                            Row::create()
                                .child(
                                    FontIconBlock::create()
                                        .shared_property(icon.clone())
                                        .shared_property(selector.clone()),
                                )
                                .child(
                                    TextBlock::create()
                                        .shared_property(text.clone())
                                        .shared_property(selector.clone()),
                                ),
                        ),
                    ),
            )
            .shared_property(text)
            .shared_property(icon)
            .shared_property(selector)
            .property(Pressed(false))
            .debug_name("Button")
    }
}
