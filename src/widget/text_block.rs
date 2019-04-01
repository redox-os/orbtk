use dces::prelude::Entity;

use crate::{
    layout::{FixedSizeLayout, Layout},
    properties::*,
    render_object::{RenderObject, TextRenderObject},
    styling::{colors, fonts},
    widget::Template,
};

widget!(
    /// The `TextBlock` widget is used to draw text. It is not interactive.
    /// 
    /// * CSS element: `text-block`
    TextBlock {
        /// Sets or shares the text property.
        text: Text,

        /// Sets or shares the foreground property.
        foreground: Foreground,

        /// Sets or share the font size property.
        font_size: FontSize,

        /// Sets or shares the font property.
        font: Font,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl Template for TextBlock {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("TextBlock")
            .selector("text-block")
            .text("")
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(fonts::FONT_SIZE_12)
            .font(fonts::font_into_box(fonts::ROBOTO_REGULAR_FONT))
    }

    fn render_object(&self) -> Option<Box<dyn RenderObject>> {
        Some(Box::new(TextRenderObject))
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(FixedSizeLayout::new())
    }
}
