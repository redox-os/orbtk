use std::cell::Cell;

use crate::prelude::*;

/// The `ScrollViewerState` handles the `ScrollViewer` widget.
#[derive(Default)]
pub struct ScrollViewerState {
    delta: Cell<Option<Point>>,
}

impl ScrollViewerState {
    fn scroll(&self, delta: Point) {
        self.delta.set(Some(delta));
    }
}

impl State for ScrollViewerState {
    fn update(&self, context: &mut Context<'_>) {
        if let Some(delta) = self.delta.get() {
            context.widget().set(Delta(delta));
        }
    }

    fn update_post_layout(&self, context: &mut Context<'_>) {
        if let Some(_) = self.delta.get() {
            context.widget().set(Delta(Point::new(0.0, 0.0)));
            self.delta.set(None);
        }
    }
}

widget!(
    /// The `ScrollViewer` defines a layout that is used to stack its children on the z-axis.
    ///
    /// **CSS element:** `scroll-viewer`
    ScrollViewer<ScrollViewerState>: MouseHandler {
        /// Sets or shares the scroll offset property.
        scroll_offset: ScrollOffset,

        /// Sets or shares the scroll mode property.
        scroll_mode: ScrollViewerMode,

        /// Sets or shares the css selector property.
        selector: Selector,

        /// Sets or shares the (wheel, scroll) delta property. 
        delta: Delta
    }
);

impl Template for ScrollViewer {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        let state = self.clone_state();

        self.name("ScrollViewer")
            .selector("scroll-viewer")
            .scroll_offset(0.0)
            .delta(0.0)
            .scroll_mode(ScrollViewerModeValue::default())
            .on_scroll(move |p| {
                state.scroll(p);
                false
            })
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(ScrollLayout::new())
    }
}
