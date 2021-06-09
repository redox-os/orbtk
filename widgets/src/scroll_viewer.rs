use crate::{api::prelude::*, proc_macros::*};

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
            self.delta = None;
            let mode = *ctx.widget().get::<ScrollViewerMode>("mode");

            if mode.vertical != ScrollMode::Auto && mode.horizontal != ScrollMode::Auto {
                return;
            }

            let size = ctx.widget().get::<Rectangle>("bounds").size();
            let speed = *ctx.widget().get::<f64>("speed");
            let mut padding = *ctx.widget().get::<Thickness>("padding");

            if let Some(child) = &mut ctx.try_child_from_index(0) {
                let child_size = child.get::<Rectangle>("bounds").size();

                if mode.vertical == ScrollMode::Auto && child_size.height() > size.height() {
                    padding.set_top(offset(
                        size.height(),
                        child_size.height(),
                        padding.top(),
                        delta.y() * speed,
                    ));
                }

                if mode.horizontal == ScrollMode::Auto && child_size.width() > size.width() {
                    padding.set_left(offset(
                        size.width(),
                        child_size.width(),
                        padding.left(),
                        delta.x() * speed,
                    ));
                }
            } else {
                return;
            }

            ctx.widget().set("padding", padding);
        }
    }
}

widget!(
    /// The `ScrollViewer` is used to scroll its child vertical and or horizontal.
    /// Only the first child of the scroll viewer can be scrolled.
    ScrollViewer<ScrollViewerState>: MouseHandler {
        /// Sets or shares the scroll mode property.
        mode: ScrollViewerMode,

        /// Sets or shares the scroll speed. Use it to adjust the speed of scrolling.
        speed: f64,

        /// Sets or shares padding, that is used to scroll the first child.
        padding: Thickness
    }
);

impl Template for ScrollViewer {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("ScrollViewer")
            .padding(0)
            .speed(8)
            .clip(true)
            .mode(ScrollViewerMode::default())
            .on_scroll(move |states, p| {
                states.get_mut::<ScrollViewerState>(id).scroll(p);
                false
            })
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(PaddingLayout::new())
    }
}

// --- Helpers --

fn offset(size: f64, child_size: f64, current_offset: f64, delta: f64) -> f64 {
    (current_offset + delta).min(0.).max(size - child_size)
}

// --- Helpers --

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset() {
        let width = 100.;
        let child_width = 200.0;

        assert!((offset(width, child_width, 0., -10.) + 10.).abs() < f64::EPSILON);
        assert!((offset(width, child_width, 0., -200.) + 100.).abs() < f64::EPSILON);
        assert!((offset(width, child_width, 0., 200.) + 0.).abs() < f64::EPSILON);
    }
}
