use crate::{
    event::ClickHandler,
    properties::*,
    styling::{colors, fonts},
    widget::{Container, FontIconBlock, Property, Stack, Template, TextBlock, Widget},
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
        let text = Text::prop("");
        let foreground = Foreground::prop(colors::LINK_WATER_COLOR);
        let font =
            Property::new(Font::from(fonts::font_into_box(fonts::ROBOTO_REGULAR_FONT)));
        let font_size = Property::new(FontSize::from(fonts::FONT_SIZE_12));

        // icon properties
        let icon = Property::new(FontIcon::default());
        let icon_brush = Property::new(IconBrush::from(colors::LINK_WATER_COLOR));
        let icon_font = Property::new(IconFont::from(fonts::font_into_box(
            fonts::MATERIAL_ICONS_REGULAR_FONT,
        )));
        let icon_size = Property::new(IconSize::from(fonts::ICON_FONT_SIZE_12));

        // container properties
        let background = Property::new(Background::from(colors::LYNCH_COLOR));
        let border_radius = Property::new(BorderRadius::from(2.0));
        let border_thickness = Property::new(BorderThickness::from(0.0));
        let border_brush = Property::new(BorderBrush::from("transparent"));
        let padding = Property::new(Padding::from((8.0, 0.0, 8.0, 0.0)));
        let opacity = Property::new(Opacity::from(1.0));

        ButtonTemplate::new()
            .height(32.0)
            .min_width(80.0)
            .pressed(false)
            .selector("button")
            .debug_name("Button")
            .child(
                Container::create()
                    .child(
                        Stack::create()
                            .orientation("Horizontal")
                            .vertical_alignment("Center")
                            .horizontal_alignment("Center")
                            .child(
                                FontIconBlock::create()
                                    .margin((0.0, 0.0, 2.0, 0.0))
                                    .shared_font_icon(icon.share())
                                    .shared_icon_brush(icon_brush.share())
                                    .shared_icon_size(icon_size.share())
                                    .shared_icon_font(icon_font.share()),
                            )
                            .child(
                                TextBlock::create()
                                    .shared_foreground(foreground.share())
                                    .shared_text(text.share())
                                    .shared_font(font.share())
                                    .shared_font_size(font_size.share()),
                            ),
                    )
                    .shared_padding(padding.share())
                    .shared_background(background.share())
                    .shared_border_radius(border_radius.share())
                    .shared_border_thickness(border_thickness.share())
                    .shared_border_brush(border_brush.share()),
            )
            .shared_text(text)
            .shared_font(font)
            .shared_font_size(font_size)
            .shared_font_icon(icon)
            .shared_icon_brush(icon_brush)
            .shared_icon_size(icon_size)
            .shared_icon_font(icon_font)
            .shared_foreground(foreground)
            .shared_background(background)
            .shared_border_radius(border_radius)
            .shared_border_thickness(border_thickness)
            .shared_border_brush(border_brush)
            .shared_padding(padding)
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
        IconSizeProperty,
        IconBrushProperty,
        IconFontProperty,
        ForegroundProperty,
        PressedProperty,
        PaddingProperty,
        ClickHandler
    ]
);
