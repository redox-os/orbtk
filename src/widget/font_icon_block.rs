// use crate::{
//     layout::FixedSizeLayout,
//     properties::{FontIconProperty, IconBrushProperty, IconFontProperty, IconSizeProperty},
//     render_object::FontIconRenderObject,
//     styling::{colors, fonts},
//     widget::{Template, Widget},
// };

// widget!(
//     /// The `FontIconBlock` widget is used to draw an font icon. It is not interactive.
//     FontIconBlock
//     (
//         FontIconProperty,
//         IconSizeProperty,
//         IconBrushProperty,
//         IconFontProperty
//     )
// );

// impl Widget for FontIconBlock {
//     fn create() -> Self {
//         FontIconBlock::new()
//             .layout(FixedSizeLayout::new())
//             .render_object(FontIconRenderObject)
//             .debug_name("FontIconBlock")
//             .font_icon("")
//             .icon_brush(colors::LINK_WATER_COLOR)
//             .icon_size(fonts::ICON_FONT_SIZE_12)
//             .icon_font(fonts::font_into_box(fonts::MATERIAL_ICONS_REGULAR_FONT))
//             .selector("fonticon")
//     }
// }
