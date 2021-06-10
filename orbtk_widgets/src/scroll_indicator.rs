use crate::{api::prelude::*, prelude::*, proc_macros::*};

// --- KEYS --
static ID_SCROLL_BAR_HORIZONTAL: &str = "scroll_bar_horizontal";
static ID_SCROLL_BAR_VERTICAL: &str = "scroll_bar_vertical";
// --- KEYS --

/// The `ScrollIndicatorState` handles the `ScrollIndicator` widget.
#[derive(Default, AsAny)]
pub struct ScrollIndicatorState {
    horizontal_scroll_bar: Entity,
    vertical_scroll_bar: Entity,
}

impl State for ScrollIndicatorState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.horizontal_scroll_bar = ctx
            .entity_of_child(ID_SCROLL_BAR_HORIZONTAL)
            .expect("ScrollIndicatorState.init: scroll_bar_horizontal child could not be found.");
        self.vertical_scroll_bar = ctx
            .entity_of_child(ID_SCROLL_BAR_HORIZONTAL)
            .expect("ScrollIndicatorState.init: scroll_bar_vertical child could not be found.");
    }

    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context) {
        let mode = *ctx.widget().get::<ScrollViewerMode>("mode");

        if mode.vertical != ScrollMode::Auto && mode.horizontal != ScrollMode::Auto {
            return;
        }

        let size = ctx.widget().get::<Rectangle>("bounds").size();
        let content_size = ctx.widget().get::<Rectangle>("content_bounds").size();
        let view_port_size = ctx.widget().get::<Rectangle>("view_port_bounds").size();
        let padding = *ctx.widget().get::<Thickness>("padding");
        let scroll_padding = *ctx.widget().get::<Thickness>("scroll_padding");

        // adjust vertical scroll bar
        if mode.vertical != ScrollMode::Disabled && content_size.height() > view_port_size.height()
        {
            let mut scroll_bar = ctx.get_widget(self.vertical_scroll_bar);

            if *scroll_bar.get::<Visibility>("visibility") != Visibility::Visible {
                scroll_bar.set("visibility", Visibility::Visible);
            }

            scroll_bar
                .get_mut::<Rectangle>("bounds")
                .set_height(scroll_bar_size(
                    size.height(),
                    content_size.height(),
                    view_port_size.height(),
                    padding.top() + padding.bottom(),
                ));

            scroll_bar.get_mut::<Rectangle>("bounds").set_y(-offset(
                size.height(),
                content_size.height(),
                scroll_padding.top(),
            ));
        }

        // adjust horizontal scroll bar
        if mode.horizontal != ScrollMode::Disabled && content_size.width() > view_port_size.width()
        {
            let mut scroll_bar = ctx.get_widget(self.horizontal_scroll_bar);

            if *scroll_bar.get::<Visibility>("visibility") != Visibility::Visible {
                scroll_bar.set("visibility", Visibility::Visible);
            }

            scroll_bar
                .get_mut::<Rectangle>("bounds")
                .set_width(scroll_bar_size(
                    size.width(),
                    content_size.width(),
                    view_port_size.width(),
                    padding.left() + padding.right(),
                ));

            scroll_bar.get_mut::<Rectangle>("bounds").set_x(-offset(
                size.width(),
                content_size.width(),
                scroll_padding.left(),
            ));
        }
    }
}

widget!(
    /// The `ScrollIndicator` widget contains two scroll bars.
    ScrollIndicator<ScrollIndicatorState> {
        /// Shares the mode of the `ScrollViewer`.
        mode: ScrollViewerMode,

        /// Shares the padding of the `ScrollViewer`.
        scroll_padding: Thickness,

        /// Shares the bounds of the content.
        content_bounds: Rectangle,

        /// Shares the bounds of the `ScrollViewer`.
        view_port_bounds: Rectangle,

        /// Sets or shares the padding property.
        padding: Thickness
    }
);

impl Template for ScrollIndicator {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("ScrollIndicator")
            .v_align("stretch")
            .h_align("stretch")
            .padding(0.0)
            .child(
                Grid::new()
                    .child(
                        ScrollBar::new()
                            .id(ID_SCROLL_BAR_HORIZONTAL)
                            .visibility("collapsed")
                            .min_height(8.0)
                            .margin((0.0, 0.0, 0.0, 6.0))
                            .h_align("end")
                            .opacity(id)
                            .build(ctx),
                    )
                    .child(
                        ScrollBar::new()
                            .id(ID_SCROLL_BAR_VERTICAL)
                            .visibility("collapsed")
                            .min_width(8.0)
                            .margin((0.0, 0.0, 6.0, 0.0))
                            .height(4.0)
                            .v_align("end")
                            .opacity(id)
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(PaddingLayout::new())
    }
}

// --- Helpers --

fn scroll_bar_size(size: f64, content_size: f64, view_port_size: f64, padding: f64) -> f64 {
    (size * view_port_size / content_size) - padding
}

fn offset(size: f64, content_size: f64, offset: f64) -> f64 {
    size * offset / content_size
}

// --- Helpers --

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_bar_size() {
        let size = 50.;
        let content_size = 200.;
        let view_port_size = 80.0;
        let padding = 8.;

        assert!(
            (scroll_bar_size(size, content_size, view_port_size, padding) - 12.).abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn test_offset() {
        let size = 50.;
        let content_size = 200.;
        let offset_in = 8.;

        assert!((offset(size, content_size, offset_in) - 2.).abs() < f64::EPSILON);
    }
}
