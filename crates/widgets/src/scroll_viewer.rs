use crate::prelude::*;

/// The `ScrollViewerState` handles the `ScrollViewer` widget.
#[derive(Default, AsAny)]
pub struct ScrollViewerState {
    delta: Option<Point>,
}

impl ScrollViewerState {
    fn scroll(&mut self, delta: Point) {
        self.delta = Some(delta);
    }
}

impl State for ScrollViewerState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(delta) = self.delta {
            ctx.widget().set("delta", delta);
        }
    }

    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context) {
        if self.delta.is_some() {
            ctx.widget().set("delta", Point::new(0.0, 0.0));
            self.delta = None;
        }
    }
}

widget!(
    /// The `ScrollViewer` defines a layout that is used to stack its children on the z-axis.
    ScrollViewer<ScrollViewerState>: MouseHandler {
        /// Sets or shares the scroll offset property.
        scroll_offset: Point,

        /// Sets or shares the scroll mode property.
        scroll_viewer_mode: ScrollViewerMode,

        /// Sets or shares the (wheel, scroll) delta property.
        delta: Point
    }
);

impl Template for ScrollViewer {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("ScrollViewer")
            .scroll_offset(0.0)
            .delta(0.0)
            .clip(true)
            .scroll_viewer_mode(ScrollViewerMode::default())
            .on_scroll(move |states, p| {
                states.get_mut::<ScrollViewerState>(id).scroll(p);
                false
            })
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(ScrollLayout::new())
    }
}
