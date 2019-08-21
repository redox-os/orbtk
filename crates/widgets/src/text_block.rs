use crate::prelude::*;

widget!(
    /// The `TextBlock` widget is used to draw text. It is not interactive.
    ///
    /// **CSS element:** `text-block`
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
            .text("")
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(fonts::FONT_SIZE_12)
            .font("Roboto Regular")
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
       Box::new(TextRenderObject)
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(FixedSizeLayout::new())
    }
}
