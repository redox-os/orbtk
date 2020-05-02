use crate::prelude::*;

static RANGE_MIN: f64 = 0.0;
static RANGE_MAX: f64 = 1.0;

#[derive(Default, AsAny)]
struct BarState {}

impl State for BarState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        ctx.child_from_index(0)
            .get_mut::<Constraint>("constraint")
            .set_width(0.1);
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        let val = ctx.widget().clone_or_default::<f64>("val");
        let new_width: f64;

        if val == RANGE_MIN {
            new_width = 0.1;
        } else {
            let max_width = ctx.widget().get::<Rectangle>("bounds").width();
            if val == RANGE_MAX {
                new_width = max_width * 0.99;
            } else if val > RANGE_MIN && val < RANGE_MAX {
                new_width = max_width * val;
            } else {
                new_width = max_width * 0.99;
            }
        }
        ctx.child_from_index(0)
            .get_mut::<Constraint>("constraint")
            .set_width(new_width);
    }
}

widget!(
    /// The `ProgressBar` widget is used to indicating a finite progress
    /// (e.g. copying a file, downloading a video from the internet).
    /// A progress is visually represented as a horizontal bar which grows when the progress advances.
    /// The ProgressBar expects values between 0.0 and 1.0, whereas 0.0 means 0%, and 1.0 means 100%.
    /// Any value outside of this range cosidered as 100%.
    /// 
    /// This example creates a ProgressBar with default values:
    /// ```rust
    /// ProgressBar::create().build(ctx)
    /// ```
    /// 
    /// The next example creates a ProgressBar initialized with 25% progress:
    /// ```rust
    /// ProgressBar::create().val(0.25).build(ctx)
    /// ```
    /// 
    /// The progress can be controlled by changing the value of the `val` property.
    /// (this code asssumes that you have a children with id "pgbar")
    /// ```rust
    /// ctx.child("pgbar").set::<f64>("val", amount);
    /// ```
    ProgressBar<BarState> {
        background: Brush,
        border_brush: Brush,
        border_radius: f64,
        border_width: Thickness,
        padding: Thickness,
        val: f64
    }
);

impl Template for ProgressBar {
    fn template(self, _: Entity, build_context: &mut BuildContext) -> Self {
        self.name("ProgressBar")
            .background("#000000")
            .border_brush("#BABABA")
            .border_radius(4.0)
            .border_width(1.0)
            .element("progress_bar")
            .height(34.0)
            .padding((2.0, 4.0, 2.0, 4.0))
            .child(
                Container::create()
                    .background("#EFD035")
                    .height(24.0)
                    .border_radius(1.0)
                    .width(0.0)
                    .build(build_context),
            )
            .val(0.0)
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(RectangleRenderObject)
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(PaddingLayout::new())
    }
}
