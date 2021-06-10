use crate::{api::prelude::*, proc_macros::*, themes::theme_orbtk::*};

widget!(
    /// The `FontIconBlock` widget is used to draw text. It is not interactive.
    ///
    /// **style:** `font-icon-block`
    FontIconBlock {
        /// Sets or shares the icon property.
        icon: String,

        /// Sets or shares the icon brush property.
        icon_brush: Brush,

        /// Sets or share the icon font size property.
        icon_size: f64,

        /// Sets or shares the icon font property.
        icon_font: String
    }
);

impl Template for FontIconBlock {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("FontIconBlock")
            .style("font-icon-block")
            .icon("")
            .icon_brush(colors::LINK_WATER_COLOR)
            .icon_size(orbtk_fonts::ICON_FONT_SIZE_12)
            .icon_font("MaterialIcons-Regular")
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        FontIconRenderObject.into()
    }

    fn layout(&self) -> Box<dyn Layout> {
        FixedSizeLayout::new().into()
    }
}
