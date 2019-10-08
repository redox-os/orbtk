use crate::prelude::*;

widget!(
    /// The `ScrollBar` widget represents a position inside of a scroll viewer.
    /// 
    /// **CSS element:** `cursor`
    ScrollBar {
        /// Sets or shares the background property.
        background: Background,

        /// Sets or shares the border radius property.
        border_radius: BorderRadius,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl Template for ScrollBar {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("ScrollBar")
            .width(4.0)
            .border_radius(4.0)
            .background("#647b91")
            .selector("scroll-bar")
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(RectangleRenderObject)
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(TextSelectionLayout::default())
    }
}