use crate::{
    event::ClickHandler,
    properties::*,
    styling::{colors, fonts},
    widget::{Container, FontIconBlock, Property, Stack, Template, TextBlock, Widget },
};

widget!(
    /// The `Button` widget can be clicked by user. It's used to perform an action.
    Button
    (
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
    )
);

impl Widget for Button {
    fn create() -> Self {
        // text properties
        let text: Property = Text::default().into();
        let foreground: Property = Foreground::from(colors::LINK_WATER_COLOR).into();
        let font: Property = Font::from(fonts::font_into_box(fonts::ROBOTO_REGULAR_FONT)).into();
        let font_size: Property = FontSize::from(fonts::FONT_SIZE_12).into();

        // icon properties
        let icon: Property = FontIcon::default().into();
        let icon_brush: Property = IconBrush::from(colors::LINK_WATER_COLOR).into();
        let icon_font: Property =
            IconFont::from(fonts::font_into_box(fonts::MATERIAL_ICONS_REGULAR_FONT)).into();
        let icon_size: Property = IconSize::from(fonts::ICON_FONT_SIZE_12).into();

        // container properties
        let background: Property = Background::from(colors::LYNCH_COLOR).into();
        let border_radius: Property = BorderRadius::from(2.0).into();
        let border_thickness: Property = BorderThickness::from(0.0).into();
        let border_brush: Property = BorderBrush::from("transparent").into();
        let padding: Property = Padding::from((8.0, 0.0, 8.0, 0.0)).into();
        let _opacity: Property = Opacity::from(1.0).into();

        Button::new()
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
                    .background_prop(background.share())
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
            .background_prop(background)
            .shared_border_radius(border_radius)
            .shared_border_thickness(border_thickness)
            .shared_border_brush(border_brush)
            .shared_padding(padding)
    }
}
