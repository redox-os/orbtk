use crate::{
    enums::ParentType,
    properties::{FontIcon, Text, Selected},
    styling::vector_graphics::material_font_icons,
    theme::Selector,
    widget::{
        Center, Container, FontIconBlock, Row, SharedProperty, Spacer, Template, TextBlock, Widget,
    },
};

/// The `Checkbox` widget can be switch its selected state. It contains a selection box and a text.
///
/// # Shared Properties
///
/// * `Text` - String used to display the text of the check box.
/// * `FontIcon` - String used to display the font icon of the check box.
/// * `Selector` - CSS selector with  element name `checkbox`, used to request the theme of the widget.
///
/// # Properties
///
/// * `Selected` - Bool value represents the selected state of the widget.
///
/// # Others
///
/// * `ParentType`- Single.
pub struct CheckBox;

impl Widget for CheckBox {
    fn create() -> Template {
        let text = SharedProperty::new(Text::default());
        let icon = SharedProperty::new(FontIcon::from(material_font_icons::CHECK_FONT_ICON));
        let selector = SharedProperty::new(Selector::from("checkbox"));

        Template::new()
           .parent_type(ParentType::Single)
            .child(
                Row::create()
                    .child(
                        Container::create()
                            .child(
                                Center::create().child(
                                    FontIconBlock::create()
                                        .shared_property(icon.clone())
                                        .shared_property(selector.clone()),
                                ),
                            )
                            .shared_property(selector.clone()),
                    )
                    .child(Spacer::create())
                    .child(
                        Center::create().child(
                            TextBlock::create()
                                .shared_property(text.clone())
                                .shared_property(selector.clone()),
                        ),
                    ),
            )
            .shared_property(icon)
            .shared_property(text)
            .shared_property(selector)
            .property(Selected(false))
            .debug_name("CheckBox")
    }
}
