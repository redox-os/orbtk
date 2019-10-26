use crate::prelude::*;

widget!(
    /// The `ScrollBar` widget represents a position inside of a scroll container.
    ///
    /// **CSS element:** `scroll-bar`
    ScrollBar {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl Template for ScrollBar {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("ScrollBar")
            .width(4.0)
            .border_radius(2.0)
            .background("#647b91")
            .selector("scroll-bar")
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(RectangleRenderObject)
    }
}
