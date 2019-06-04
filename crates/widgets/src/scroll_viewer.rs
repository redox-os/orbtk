use crate::prelude::*;

widget!(
    /// The `ScrollViewer` defines a layout that is used to stack its children on the z-axis.
    /// 
    /// **CSS element:** `scroll-viewer`
    ScrollViewer {
        /// Sets or shares the orientation property.
        offset: Offset,

        /// Sets or shares the scroll mode property.
        scroll_mode: ScrollViewerMode,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl Template for ScrollViewer {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("ScrollViewer").selector("scroll-viewer").offset(0.0).scroll_mode(ScrollViewerModeValue::default())
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(ScrollLayout::new())
    }
}
