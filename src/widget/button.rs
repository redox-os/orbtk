use dces::prelude::Entity;

use crate::{
    event::ClickHandler,
    properties::*,
    styling::{colors, fonts},
    widget::{
       Container, TextBlock, Template
    },
};

widget!(
    /// This is a button
    Button: ClickHandler {
        /// Sets the background
        background: Background,

        /// Sets the foreground
        foreground: Foreground,

        /// Sets the text
        text: Text
    }
);

impl Template for Button {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        self.child(
            Container::create()
                .background(id)
                .child(
                    TextBlock::create()
                        .foreground(id)
                        .text(id)
                        .build(context),
                )
                .build(context),
        )
    }
}

// widget!(
//     /// The `Button` widget can be clicked by user. It's used to perform an action.
//     Button(
//         BackgroundProperty,
//         BorderRadiusProperty,
//         BorderThicknessProperty,
//         BorderBrushProperty,
//         TextProperty,
//         FontProperty,
//         FontSizeProperty,
//         FontIconProperty,
//         IconSizeProperty,
//         IconBrushProperty,
//         IconFontProperty,
//         ForegroundProperty,
//         PressedProperty,
//         PaddingProperty,
//         ClickHandler
//     )
// );

// impl Widget for Button {
//     fn create() -> Self {
//         // text properties
//         let text: Property = Text::default().into();
//         let foreground: Property = Foreground::from(colors::LINK_WATER_COLOR).into();
//         let font: Property = Font::from(fonts::font_into_box(fonts::ROBOTO_REGULAR_FONT)).into();
//         let font_size: Property = FontSize::from(fonts::FONT_SIZE_12).into();

//         // icon properties
//         let icon: Property = FontIcon::default().into();
//         let icon_brush: Property = IconBrush::from(colors::LINK_WATER_COLOR).into();
//         let icon_font: Property =
//             IconFont::from(fonts::font_into_box(fonts::MATERIAL_ICONS_REGULAR_FONT)).into();
//         let icon_size: Property = IconSize::from(fonts::ICON_FONT_SIZE_12).into();

//         // container properties
//         let background: Property = Background::from(colors::LYNCH_COLOR).into();
//         let border_radius: Property = BorderRadius::from(2.0).into();
//         let border_thickness: Property = BorderThickness::from(0.0).into();
//         let border_brush: Property = BorderBrush::from("transparent").into();
//         let padding: Property = Padding::from((8.0, 0.0, 8.0, 0.0)).into();
//         let opacity: Property = Opacity::from(1.0).into();

//         Button::new()
//             .height(32.0)
//             .min_width(80.0)
//             .pressed(false)
//             .selector("button")
//             .debug_name("Button")
//             .child(
//                 Container::create()
//                     .child(
//                         Stack::create()
//                             .orientation("Horizontal")
//                             .vertical_alignment("Center")
//                             .horizontal_alignment("Center")
//                             .child(
//                                 FontIconBlock::create()
//                                     .margin((0.0, 0.0, 2.0, 0.0))
//                                     .font_icon_prop(icon.share())
//                                     .icon_brush_prop(icon_brush.share())
//                                     .icon_size_prop(icon_size.share())
//                                     .icon_font_prop(icon_font.share()),
//                             )
//                             .child(
//                                 TextBlock::create()
//                                     .foreground_prop(foreground.share())
//                                     .text_prop(text.share())
//                                     .font_prop(font.share())
//                                     .font_size_prop(font_size.share()),
//                             ),
//                     )
//                     .padding_prop(padding.share())
//                     .background_prop(background.share())
//                     .border_radius_prop(border_radius.share())
//                     .border_thickness_prop(border_thickness.share())
//                     .border_brush_prop(border_brush.share()),
//             )
//             .text_prop(text)
//             .font_prop(font)
//             .font_size_prop(font_size)
//             .font_icon_prop(icon)
//             .icon_brush_prop(icon_brush)
//             .icon_size_prop(icon_size)
//             .icon_font_prop(icon_font)
//             .foreground_prop(foreground)
//             .background_prop(background)
//             .border_radius_prop(border_radius)
//             .border_thickness_prop(border_thickness)
//             .border_brush_prop(border_brush)
//             .padding_prop(opacity)
//     }
// }
