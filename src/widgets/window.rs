use crate::prelude::*;

widget!(
    /// The `Window` widget provides access to the properties of a application window.
    /// It also contains global properties like keyboard modifier and focused widget.
    /// 
    /// **CSS element:** `window`
    Window {
        /// Sets or shares the background property.
        background: Background,

        /// Sets or shares the title property.
        title: Title,

        /// Sets or shares the css selector property. 
        selector: Selector,

        /// Sets or shares the resizeable property. 
        resizeable: Resizeable,

        /// Sets or shares the position property. 
        position: Pos
    }
);

impl Template for Window {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Window")
            .background(colors::BRIGHT_GRAY_COLOR)
            .size(100.0, 100.0)
            .selector("window")
            .title("Window")
            .resizeable(false)
    }

    fn render_object(&self) -> Option<Box<dyn RenderObject>> {
        Some(Box::new(RectangleRenderObject))
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(GridLayout::new())
    }
}