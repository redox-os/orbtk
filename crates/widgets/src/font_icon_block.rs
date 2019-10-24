use crate::prelude::*;

widget!(
    /// The `FontIconBlock` widget is used to draw text. It is not interactive.
    ///
    /// **CSS element:** `font-icon-block`
    FontIconBlock {
        /// Sets or shares the icon property.
        icon: FontIcon,

        /// Sets or shares the icon brush property.
        icon_brush: IconBrush,

        /// Sets or share the icon font size property.
        icon_size: FontSize,

        /// Sets or shares the icon font property.
        icon_font: Font,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl Template for FontIconBlock {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("FontIconBlock")
            .selector("font-icon-block")
            .icon("")
            .icon_brush(colors::LINK_WATER_COLOR)
            .icon_size(fonts::ICON_FONT_SIZE_12)
            .icon_font("Material Icons")
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(FontIconRenderObject)
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(FixedSizeLayout::new())
    }
}
