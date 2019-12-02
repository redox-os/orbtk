use crate::prelude::*;

#[derive(Copy, Clone)]
enum SliderAction {
    Move { x: f64, y: f64 },
}

/// The `SliderState` is used to manipulate the position of the thumb of the slider widget.
#[derive(Default, AsAny)]
pub struct SliderState {
    action: Option<SliderAction>,
}

impl SliderState {
    fn action(&mut self, action: SliderAction) {
        self.action = Some(action);
    }
}

impl State for SliderState {
    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                SliderAction::Move { x, y } => {
                    if *ctx.child("thumb").get::<bool>("pressed") {
                        let width = ctx.widget().get::<Rectangle>("bounds").width();
                        let x = x - ctx.widget().get::<Point>("position").x;
                        let thumb_width = ctx.child("thumb").get::<Rectangle>("bounds").width() / 2.0;
                        
                        let pixel_range = width - thumb_width;
                        let x = (x - thumb_width).max(0.0).min(pixel_range);

                        let p_x = x / pixel_range;

                        let min: f64 = *ctx.widget().get("minimum");
                        let max: f64 = *ctx.widget().get("maximum");

                        let range = max + 1.0 - min;
                        let val = range * p_x;

                        ctx.widget().set("value", val);


         
                        ctx.child("thumb").get_mut::<Thickness>("margin").set_left(x);

                        ctx.push_event(ChangedEvent(ctx.entity));
                    }
                }
            }

            self.action = None;
        }
    }
}

widget!(
    /// The `Slider` allows to use a value in a range of values.
    ///
    /// **CSS element:** `Slider`
    Slider<SliderState>: MouseHandler, ChangedHandler {
        /// Sets or shares the minimum of the range.
        minimum: f64,

        /// Sets or shared the maximum of the range.
        maximum: f64,

        /// Sets or shares the current value of the range.
        value: f64,

        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl Template for Slider {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("Slider")
            .selector("slider")
            .minimum(0.0)
            .maximum(100.0)
            .value(0.0)
            .height(32.0)
            .border_radius(4.0)
            .child(
                Grid::create()
                    .child(
                        Container::create()
                            // todo fix border radius from css
                            .margin((8.0, 0.0, 8.0, 0.0))
                            .border_radius(id)
                            .background(id)
                            .vertical_alignment("center")
                            .height(8.0)
                            .build(ctx),
                    )
                    .child(
                        // todo: selector default crashes
                        Button::create()
                            .background("green")
                            .selector(Selector::from("thumb").id("thumb"))
                            .vertical_alignment("center")
                            .horizontal_alignment("start")
                            .max_width(28.0)
                            .max_height(28.0)
                            .border_radius(16.0)
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .on_mouse_move(move |states, p| {
                states
                    .get_mut::<SliderState>(id)
                    .action(SliderAction::Move { x: p.x, y: p.y });
                true
            })
    }
}
