use crate::{
    properties::{
        FontIcon, FontIconProperty, OrientationProperty, PaddingProperty, PressedProperty,
        SelectedProperty, Text, TextProperty,
    },
    theme::Selector,
    widget::{Container, FontIconBlock, Property, Stack, Template, TextBlock, Widget},
};

/// The `ToggleButton` widget can be clicked by user. It's used to perform an action.
///
/// # Properties
///
/// * `text` - String used to display the text of the button.
/// * `font_icon` - String used to display the font icon of the button.
/// * `selector` - CSS selector with  element name `button`, used to request the theme of the widget.
/// * `pressed` - Bool value represents the pressed state of the button.
pub struct ToggleButton;

impl Widget for ToggleButton {
    type Template = ToggleButtonTemplate;

    fn create() -> Self::Template {
        let text = Property::new(Text::default());
        let icon = Property::new(FontIcon::default());
        let selector = Property::new(Selector::from("togglebutton"));

        ToggleButtonTemplate::new()
            .height(32.0)
            .min_width(80.0)
            .selected(false)
            .pressed(false)
            .debug_name("ToggleButton")
            .child(
                Container::create()
                    .padding((8.0, 0.0, 8.0, 0.0))
                    .shared_selector(selector.share())
                    .child(
                        Stack::create()
                            .orientation("Horizontal")
                            .vertical_alignment("Center")
                            .horizontal_alignment("Center")
                            .child(
                                FontIconBlock::create()
                                    .margin((0.0, 0.0, 2.0, 0.0))
                                    .shared_font_icon(icon.share())
                                    .shared_selector(selector.share()),
                            )
                            .child(
                                TextBlock::create()
                                    .shared_text(text.share())
                                    .shared_selector(selector.share()),
                            ),
                    ),
            )
            .shared_text(text)
            .shared_font_icon(icon)
            .shared_selector(selector)
    }
}

template!(
    ToggleButtonTemplate,
    [
        TextProperty,
        FontIconProperty,
        PressedProperty,
        SelectedProperty
    ]
);
