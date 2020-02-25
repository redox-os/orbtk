use crate::prelude::*;

widget!(
    /// The `ScrollBar` widget represents a position inside of a scroll container.
    ///
    /// **CSS element:** `scroll-bar`
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
            .width(4.0)
            .border_radius(2.0)
            .background("#647b91")
            .element("scroll-bar")
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(RectangleRenderObject)
    }
}
