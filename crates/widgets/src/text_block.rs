use crate::{api::prelude::*, proc_macros::*, theme::prelude::*};

widget!(
    /// The `TextBlock` widget is used to draw text. It is not interactive.
    ///
    /// **style:** `text-block`
    TextBlock {
        /// Sets or shares the text property.
        text: String16,

        /// Sets or shares the water_mark text property.
        water_mark: String16,

        /// Sets or shares the foreground property.
        foreground: Brush,

        /// Sets or shares the font size property.
        font_size: f64,

        /// Sets or shares the font property.
        font: String,

        /// Defines an extra offset that can be used to the text on x axis.
        offset: f64
    }
);

impl Template for TextBlock {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("TextBlock")
            .text("")
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(fonts::FONT_SIZE_12)
            .font("Roboto-Regular")
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        TextRenderObject.into()
    }

    fn layout(&self) -> Box<dyn Layout> {
        FixedSizeLayout::new().into()
    }
}
