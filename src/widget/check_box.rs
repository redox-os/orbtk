use crate::{
    material_font_icons,
    properties::{
        FontIcon, FontIconProperty, OrientationProperty, PressedProperty, SelectedProperty, Text,
        TextProperty,
    },
    theme::Selector,
    widget::{Container, FontIconBlock, SharedProperty, Stack, Template, TextBlock, Widget},
};
/// The `Checkbox` widget can be switch its selected state. It contains a selection box and a text.
///
/// # Properties
///
/// * `text` - String used to display the text of the check box.
/// * `font_icon` - String used to display the font icon of the check box.
/// * `selector` - CSS selector with  element name `checkbox`, used to request the theme of the widget.
/// * `selected` - Bool value represents the selected state of the widget.
pub struct CheckBox;

impl Widget for CheckBox {
    type Template = CheckBoxTemplate;

    fn create() -> Self::Template {
        let text = SharedProperty::new(Text::default());
        let icon = SharedProperty::new(FontIcon::from(material_font_icons::CHECK_FONT_ICON));
        let selector = SharedProperty::new(Selector::from("checkbox"));

        CheckBoxTemplate::new()
            .height(24.0)
            .selected(false)
            .debug_name("CheckBox")
            .child(
                Stack::create()
                    .orientation("Horizontal")
                    .child(
                        Container::create()
                            .size(24.0, 24.0)
                            .shared_selector(selector.clone())
                            .child(
                                FontIconBlock::create()
                                    .vertical_alignment("Center")
                                    .horizontal_alignment("Center")
                                    .shared_font_icon(icon.clone())
                                    .shared_selector(selector.clone()),
                            ),
                    )
                    .child(
                        TextBlock::create()
                            .vertical_alignment("Center")
                            .margin((8.0, 0.0, 0.0, 0.0))
                            .shared_text(text.clone())
                            .shared_selector(selector.clone()),
                    ),
            )
            .shared_font_icon(icon)
            .shared_text(text)
            .shared_selector(selector)
    }
}

template!(
    CheckBoxTemplate,
    [
        TextProperty,
        FontIconProperty,
        PressedProperty,
        SelectedProperty
    ]
);
