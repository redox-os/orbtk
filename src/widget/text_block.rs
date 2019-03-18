use dces::prelude::Entity;

use crate::{
    widget::Template,
    properties::*,
    styling::{colors, fonts},
    render_object::{RenderObject, TextRenderObject},
    layout::{Layout, FixedSizeLayout}
};

widget!(
    /// The `TextBlock` widget is used to draw text. It is not interactive.
    TextBlock {
        /// Sets the text property.
        text: Text,

        /// Sets the foreground property.
        foreground: Foreground,

        /// Sets the font size property.
        font_size: FontSize,

        /// Sets the font property.
        font: Font 
    }
);

impl Template for TextBlock {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.text("").foreground(colors::LINK_WATER_COLOR).font_size(fonts::FONT_SIZE_12).font(fonts::font_into_box(fonts::ROBOTO_REGULAR_FONT))
    }

    fn render_object(&self) -> Option<Box<dyn RenderObject>> {
        Some(Box::new(TextRenderObject))
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(FixedSizeLayout::new())
    }
}

// use crate::{
//     layout::FixedSizeLayout,
//     properties::{FontProperty, FontSizeProperty, ForegroundProperty, TextProperty, Foreground, Text},
//     render_object::TextRenderObject,
//     styling::{colors, fonts},
//     widget::{Template, Widget, Template, BuildContext, Template },
// };

// widget!(
//     /// The `TextBlock` widget is used to draw text. It is not interactive.
//     TextBlock(
//         ForegroundProperty,
//         TextProperty,
//         FontSizeProperty,
//         FontProperty
//     )
// );

// impl Widget for TextBlock {
//     fn create() -> Self {
//         TextBlock::new()
//             .layout(FixedSizeLayout::new())
//             .render_object(TextRenderObject)
//             .foreground(colors::LINK_WATER_COLOR)
//             .font_size(fonts::FONT_SIZE_12)
//             .font(fonts::font_into_box(fonts::ROBOTO_REGULAR_FONT))
//             .text("TextBlock")
//             .debug_name("TextBlock")
//     }
// }

