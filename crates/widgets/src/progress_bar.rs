use crate::{api::prelude::*, prelude::*, proc_macros::*};

static RANGE_MIN: f64 = 0.0;
static RANGE_MAX: f64 = 1.0;

// --- KEYS --

pub static STYLE_PROGRESS_BAR: &'static str = "progress_bar";
pub static STYLE_PROGRESS_BAR_INDICATOR: &'static str = "progress_bar_indicator";
static ID_INDICATOR: &'static str = "PGBAR_INDICATOR";

// --- KEYS --

#[derive(Default, AsAny)]
struct BarState {
    indicator: Entity,
}

impl State for BarState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.indicator = ctx
            .entity_of_child(ID_INDICATOR)
            .expect("BarState.init(): Child could not be found!");
        self.update_post_layout(registry, ctx);
    }

    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context) {
        let val = ctx.widget().clone_or_default::<f64>("val");
        let max_width = ctx.widget().get::<Rectangle>("bounds").width()
            - ctx.widget().get::<Thickness>("padding").left()
            - ctx.widget().get::<Thickness>("padding").right();
        let new_width = calculate_width(val, max_width);

        ctx.get_widget(self.indicator)
            .get_mut::<Constraint>("constraint")
            .set_width(new_width);
    }
}

fn calculate_width(current_progress: f64, max_width: f64) -> f64 {
    if current_progress == RANGE_MIN {
        return 0.01;
    } else {
        if current_progress == RANGE_MAX {
            return max_width * 0.99;
        } else if current_progress > RANGE_MIN && current_progress < RANGE_MAX {
            return max_width * current_progress;
        } else {
            return max_width * 0.99;
        }
    }
}

widget!(
    /// The `ProgressBar` widget is used to indicating a finite progress
    /// (e.g. copying a file, downloading a video from the internet).
    /// A progress is visually represented as a horizontal bar which grows when the progress advances.
    /// The ProgressBar expects values between 0.0 and 1.0, whereas 0.0 means 0%, and 1.0 means 100%.
    /// Any value outside of this range considered as 100%.
    ///
    /// This example creates a ProgressBar with default values:
    /// ```rust
    /// ProgressBar::new().build(ctx)
    /// ```
    ///
    /// The next example creates a ProgressBar initialized with 25% progress:
    /// ```rust
    /// ProgressBar::new().val(0.25).build(ctx)
    /// ```
    ///
    /// The progress can be controlled by changing the value of the `val` property.
    /// (this code assumes that you have a children with id "pgbar")
    /// ```rust
    /// ctx.child("pgbar").set::<f64>("val", amount);
    /// ```
    ProgressBar<BarState> {
        /// Sets or shares the background color property
        background: Brush,
        /// Sets or shares the border color property
        border_brush: Brush,
        /// Sets or shares the border radius property
        border_radius: f64,
        /// Sets or shares the border width property
        border_width: Thickness,
        /// Sets or shares the padding property
        padding: Thickness,
        /// Sets or shares the current progress property
        val: f64
    }
);

impl Template for ProgressBar {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        self.name("ProgressBar")
            .style(STYLE_PROGRESS_BAR)
            .on_changed_filter(vec!["val"])
            .val(0.0)
            .background("#000000")
            .border_brush("#BABABA")
            .border_radius(4)
            .border_width(1)
            .height(34)
            .min_width(100.0)
            .padding((2, 4, 2, 4))
            .child(
                Container::new()
                    .id(ID_INDICATOR)
                    .style(STYLE_PROGRESS_BAR_INDICATOR)
                    .background("#EFD035")
                    .border_radius(1.0)
                    .width(0.0)
                    .height(24.0)
                    .v_align("center")
                    .h_align("start")
                    .build(ctx),
            )
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(RectangleRenderObject)
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(PaddingLayout::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_width() {
        assert_eq!(0.01, calculate_width(0.0, 100.0));
        assert_eq!(50.0, calculate_width(0.5, 100.0));
        assert_eq!(99.0, calculate_width(1.0, 100.0));
        assert_eq!(99.0, calculate_width(1.23, 100.0));
        assert_eq!(99.0, calculate_width(-1.23, 100.0));
    }
}
