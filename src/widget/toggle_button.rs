use crate::{
    properties::{
        FontIcon, FontIconProperty, OrientationProperty, PaddingProperty, PressedProperty,
        SelectedProperty, Text, TextProperty,
    },
    theme::Selector,
    widget::{Container, FontIconBlock, Property, Stack, Template, TextBlock, Widget},
};

widget!(
    /// The `ToggleButton` widget can be clicked by user. It's used to perform an action.
    ToggleButton
    (
        TextProperty,
        FontIconProperty,
        PressedProperty,
        SelectedProperty
    )
);

impl Widget for ToggleButton {
    fn create() -> Self {
        let text = Property::new(Text::default());
        let icon = Property::new(FontIcon::default());
        let selector = Property::new(Selector::from("togglebutton"));

        ToggleButton::new()
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
                                    .font_icon_prop(icon.share())
                                    .shared_selector(selector.share()),
                            )
                            .child(
                                TextBlock::create()
                                    .text_prop(text.share())
                                    .shared_selector(selector.share()),
                            ),
                    ),
            )
            .text_prop(text)
            .font_icon_prop(icon)
            .shared_selector(selector)
    }
}
