use crate::{api::prelude::*, proc_macros::*};

// --- KEYS --

pub static STYLE_SCROLL_BAR: &str = "scroll_bar";

// --- KEYS --

widget!(
    /// The `ScrollBar` widget represents a position inside of a scroll container.
    ///
    /// **style:** `scroll_bar`
    ScrollBar {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64
    }
);

impl Template for ScrollBar {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("ScrollBar")
            .style(STYLE_SCROLL_BAR)
            .width(4.0)
            .border_radius(2.0)
            .background("#647b91")
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        RectangleRenderObject.into()
    }
}
