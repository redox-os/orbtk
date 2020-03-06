use crate::prelude::*;

// --- KEYS --

pub static ELEMENT_SLIDER: &'static str = "slider";

static ID_THUMB: &'static str = "id_thumb";
static ID_TRACK: &'static str = "id_track";

// --- KEYS --

#[derive(Copy, Clone)]
enum SliderAction {
    Move { mouse_x: f64 },
}

/// The `SliderState` is used to manipulate the position of the thumb of the slider widget.
#[derive(Default, AsAny)]
pub struct SliderState {
    action: Option<SliderAction>,
    value: f64,
    minimum: f64,
    maximum: f64,
    thumb: Entity,
    track: Entity,
}

impl SliderState {
    // register an action
    fn action(&mut self, action: SliderAction) {
        self.action = Some(action);
    }

    // adjust minimum, maximum and value
    fn adjust(&mut self, ctx: &mut Context) -> bool {
        let mut has_changes = false;

        if *ctx.widget().get::<f64>("minimum") != self.minimum {
            let minimum = adjust_minimum(
                *ctx.widget().get::<f64>("minimum"),
                *ctx.widget().get::<f64>("maximum"),
            );
            ctx.widget().set("minimum", minimum);
            self.minimum = minimum;
            has_changes = true;
        }

        if *ctx.widget().get::<f64>("maximum") != self.maximum {
            let maximum = adjust_maximum(
                *ctx.widget().get::<f64>("minimum"),
                *ctx.widget().get::<f64>("maximum"),
            );
            ctx.widget().set("maximum", maximum);
            self.maximum = maximum;
            has_changes = true;
        }

        if *ctx.widget().get::<f64>("value") != self.value {
            let value = adjust_value(
                *ctx.widget().get::<f64>("value"),
                *ctx.widget().get::<f64>("minimum"),
                *ctx.widget().get::<f64>("maximum"),
            );
            ctx.widget().set("value", value);
            self.value = value;
            has_changes = true;
        }

        has_changes
    }

    // adjust the thump position
    fn adjust_thumb_x(&self, ctx: &mut Context) {
        let value = *ctx.widget().get::<f64>("value");
        let minimum = *ctx.widget().get::<f64>("minimum");
        let maximum = *ctx.widget().get::<f64>("maximum");

        let thumb_width = ctx
            .get_widget(self.thumb)
            .get::<Rectangle>("bounds")
            .width();

        let track_width = ctx
            .get_widget(self.track)
            .get::<Rectangle>("bounds")
            .width();

        ctx.get_widget(self.thumb)
            .get_mut::<Thickness>("margin")
            .set_left(calculate_thumb_x_from_value(
                value,
                minimum,
                maximum,
                track_width,
                thumb_width,
            ));
    }
}

impl State for SliderState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.thumb = ctx
            .entity_of_child(ID_THUMB)
            .expect("SliderState.init: Thumb child could not be found.");
        self.track = ctx
            .entity_of_child(ID_TRACK)
            .expect("SliderState.init: Track child could not be found.");
    }

    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                SliderAction::Move { mouse_x } => {
                    if *ctx.get_widget(self.thumb).get::<bool>("pressed") {
                        let thumb_width = ctx
                            .get_widget(self.thumb)
                            .get::<Rectangle>("bounds")
                            .width();
                        let track_width = ctx
                            .get_widget(self.track)
                            .get::<Rectangle>("bounds")
                            .width();
                        let slider_x = ctx.widget().get::<Point>("position").x;

                        let thumb_x =
                            calculate_thumb_x(mouse_x, thumb_width, slider_x, track_width);

                        ctx.get_widget(self.thumb)
                            .get_mut::<Thickness>("margin")
                            .set_left(thumb_x);

                        let minimum = *ctx.widget().get("minimum");
                        let maximum = *ctx.widget().get("maximum");

                        ctx.widget().set(
                            "value",
                            calculate_value(thumb_x, minimum, maximum, thumb_width, track_width),
                        );

                        ctx.push_event(ChangedEvent(ctx.entity));
                    }
                }
            }

            self.action = None;
            return;
        }

        if self.adjust(ctx) {
            self.adjust_thumb_x(ctx);
            ctx.push_event(ChangedEvent(ctx.entity));
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
        border_brush: Brush
    }
);

impl Template for Slider {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("Slider")
            .element(ELEMENT_SLIDER)
            .minimum(0.0)
            .maximum(100.0)
            .value(0.0)
            .height(24.0)
            .border_radius(2.0)
            .child(
                Grid::create()
                    .id(ID_TRACK)
                    .margin((8.0, 0.0, 8.0, 0.0))
                    .child(
                        Container::create()
                            .border_radius(id)
                            .background(id)
                            .vertical_alignment("center")
                            .height(2.0)
                            .build(ctx),
                    )
                    .child(
                        Button::create()
                            .element("thumb")
                            .id(ID_THUMB)
                            .vertical_alignment("center")
                            .horizontal_alignment("start")
                            .max_width(24.0)
                            .max_height(24.0)
                            .border_radius(12.0)
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .on_mouse_move(move |states, p| {
                states
                    .get_mut::<SliderState>(id)
                    .action(SliderAction::Move { mouse_x: p.x });
                true
            })
    }
}

// --- Helpers --

fn adjust_value(value: f64, minimum: f64, maximum: f64) -> f64 {
    if value < minimum {
        return minimum;
    }

    if value > maximum {
        return maximum;
    }

    value
}

fn adjust_minimum(minimum: f64, maximum: f64) -> f64 {
    if minimum > maximum {
        return maximum;
    }

    minimum
}

fn adjust_maximum(minimum: f64, maximum: f64) -> f64 {
    if maximum < minimum {
        return minimum;
    }

    maximum
}

fn calculate_thumb_x(mouse_x: f64, thumb_width: f64, slider_x: f64, track_width: f64) -> f64 {
    (mouse_x - slider_x - thumb_width)
        .max(0.0)
        .min(track_width - thumb_width)
}

fn calculate_value(
    thumb_x: f64,
    minimum: f64,
    maximum: f64,
    thumb_width: f64,
    track_width: f64,
) -> f64 {
    thumb_x / (track_width - thumb_width) * (maximum - minimum)
}

fn calculate_thumb_x_from_value(
    value: f64,
    minimum: f64,
    maximum: f64,
    track_width: f64,
    thumb_width: f64,
) -> f64 {
    (value / (maximum - minimum)) * (track_width - thumb_width)
}

// --- Helpers --

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_thumb_x() {
        assert_eq!(0.0, calculate_thumb_x(-1000.0, 32.0, 0.0, 100.0));
        assert_eq!(0.0, calculate_thumb_x(0.0, 32.0, 0.0, 100.0));
        assert_eq!(18.0, calculate_thumb_x(50.0, 32.0, 0.0, 100.0));
        assert_eq!(36.0, calculate_thumb_x(68.0, 32.0, 0.0, 100.0));
        assert_eq!(68.0, calculate_thumb_x(100.0, 32.0, 0.0, 100.0));
        assert_eq!(68.0, calculate_thumb_x(1000.0, 32.0, 0.0, 100.0));
    }

    #[test]
    fn test_calculate_value() {
        assert_eq!(0.0, calculate_value(0.0, 0.0, 100.0, 32.0, 100.0));
        assert_eq!(50.0, calculate_value(34.0, 0.0, 100.0, 32.0, 100.0));
        assert_eq!(100.0, calculate_value(68.0, 0.0, 100.0, 32.0, 100.0));
        assert_eq!(0.0, calculate_value(0.0, -50.0, 50.0, 32.0, 100.0));
        assert_eq!(50.0, calculate_value(34.0, -50.0, 50.0, 32.0, 100.0));
        assert_eq!(100.0, calculate_value(68.0, -50.0, 50.0, 32.0, 100.0));
    }

    #[test]
    fn test_adjust_value() {
        assert_eq!(0.0, adjust_value(-10.0, 0.0, 100.0));
        assert_eq!(10.0, adjust_value(10.0, 0.0, 100.0));
        assert_eq!(100.0, adjust_value(500.0, 0.0, 100.0));
    }

    #[test]
    fn test_adjust_minimum() {
        assert_eq!(0.0, adjust_minimum(0.0, 100.0));
        assert_eq!(5.0, adjust_minimum(5.0, 100.0));
        assert_eq!(100.0, adjust_minimum(500.0, 100.0));
    }

    #[test]
    fn test_adjust_maximum() {
        assert_eq!(100.0, adjust_maximum(0.0, 100.0));
        assert_eq!(100.0, adjust_maximum(100.0, 5.0));
        assert_eq!(100.0, adjust_maximum(0.0, 100.0));
    }

    #[test]
    fn test_calculate_thumb_x_from_value() {
        assert_eq!(
            0.0,
            calculate_thumb_x_from_value(0.0, 0.0, 100.0, 100.0, 32.0)
        );
        assert_eq!(
            34.0,
            calculate_thumb_x_from_value(50.0, 0.0, 100.0, 100.0, 32.0)
        );
        assert_eq!(
            68.0,
            calculate_thumb_x_from_value(100.0, 0.0, 100.0, 100.0, 32.0)
        );
    }
}
