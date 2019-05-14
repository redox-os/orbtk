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
        resizeable: Resizeable
    }
);

impl Template for Window {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Window")
            .selector("window")
            .title("Window")
            .resizeable(false)
    }
}