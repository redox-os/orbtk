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
        position: Pos,

        /// Sets or shares the theme property.
        theme: Theme
    }
);

impl Template for Window {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Window")
            .background(colors::BRIGHT_GRAY_COLOR)
            .size(100.0, 100.0)
            .selector("window")
            .title("Window")
            .theme(default_theme())
            .resizeable(false)
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(ClearRenderObject)
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(GridLayout::new())
    }
}
