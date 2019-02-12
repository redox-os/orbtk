use crate::{
    event::ClickHandler,
    properties::*,
    styling::{colors, fonts},
    widget::{Container, FontIconBlock, SharedProperty, Stack, Template, TextBlock, Widget},
};

/// The `Button` widget can be clicked by user. It's used to perform an action.
///
/// # Properties
///
/// * `text` - String used to display the text of the button.
/// * `font_icon` - String used to display the font icon of the button.
/// * `selector` - CSS selector with  element name `button`, used to request the theme of the widget.
/// * `pressed` - Bool value represents the pressed state of the button.
pub struct Button;

impl Widget for Button {
    type Template = ButtonTemplate;

    fn create() -> Self::Template {
        // text properties
        let text = SharedProperty::new(Text::default());
        let icon = SharedProperty::new(FontIcon::default());
        let foreground = SharedProperty::new(Foreground::from(colors::LINK_WATER_COLOR));
        let font =
            SharedProperty::new(Font::from(fonts::font_into_box(fonts::ROBOTO_REGULAR_FONT)));
        let font_size = SharedProperty::new(FontSize::from(fonts::FONT_SIZE_12));

        // container properties
        let background = SharedProperty::new(Background::from(colors::LYNCH_COLOR));
        let border_radius = SharedProperty::new(BorderRadius::from(2.0));
        let border_thickness = SharedProperty::new(BorderThickness::from(0.0));
        let border_brush = SharedProperty::new(BorderBrush::from("transparent"));
        let opacity = SharedProperty::new(Opacity::from(1.0));

        ButtonTemplate::new()
            .height(32.0)
            .min_width(80.0)
            .pressed(false)
            .selector("button")
            .debug_name("Button")
            .child(
                Container::create()
                    .padding((8.0, 0.0, 8.0, 0.0))
                    .shared_background(background.clone())
                    .shared_border_radius(border_radius.clone())
                    .shared_border_thickness(border_thickness.clone())
                    .shared_border_brush(border_brush.clone())
                    .child(
                        Stack::create()
                            .orientation("Horizontal")
                            .vertical_alignment("Center")
                            .horizontal_alignment("Center")
                            .child(
                                FontIconBlock::create()
                                    .margin((0.0, 0.0, 2.0, 0.0))
                                    .shared_font_icon(icon.clone()),
                            )
                            .child(
                                TextBlock::create()
                                    .shared_foreground(foreground.clone())
                                    .shared_text(text.clone())
                                    .shared_font(font.clone())
                                    .shared_font_size(font_size.clone()),
                            ),
                    ),
            )
            .shared_text(text)
            .shared_font(font)
            .shared_font_size(font_size)
            .shared_font_icon(icon)
            .shared_foreground(foreground)
            .shared_background(background)
            .shared_border_radius(border_radius)
            .shared_border_thickness(border_thickness)
            .shared_border_brush(border_brush)
    }
}

template!(
    ButtonTemplate,
    [
        BackgroundProperty,
        BorderRadiusProperty,
        BorderThicknessProperty,
        BorderBrushProperty,
        TextProperty,
        FontProperty,
        FontSizeProperty,
        FontIconProperty,
        ForegroundProperty,
        PressedProperty,
        ClickHandler
    ]
);
