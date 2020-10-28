use crate::{api::prelude::*, prelude::*, proc_macros::*};

// --- KEYS --
pub static STYLE_SLIDER: &str = "slider";
static ID_THUMB: &str = "id_thumb";
static ID_TRACK: &str = "id_track";
static ID_ACCENT_TRACK: &str = "id_accent_track";
// --- KEYS --

#[derive(Copy, Clone)]
enum SliderAction {
    Move { mouse_x: f64 },
}

/// The `SliderState` is used to manipulate the position of the thumb of the slider widget.
#[derive(Default, AsAny)]
pub struct SliderState {
    action: Option<SliderAction>,
    val: f64,
    min: f64,
    max: f64,
    thumb: Entity,
    track: Entity,
    accent_track: Entity,
}

impl SliderState {
    // register an action
    fn action(&mut self, action: SliderAction) {
        self.action = Some(action);
    }

    // adjust min, max and val
    fn adjust(&mut self, ctx: &mut Context) -> bool {
        let mut has_changes = false;
        let error = f64::EPSILON;

        if (*ctx.widget().get::<f64>("min") - self.min).abs() > error {
            let min = adjust_min(
                *ctx.widget().get::<f64>("min"),
                *ctx.widget().get::<f64>("max"),
            );
            ctx.widget().set("min", min);
            self.min = min;
            has_changes = true;
        }

        if (*ctx.widget().get::<f64>("max") - self.max).abs() > error {
            let max = adjust_max(
                *ctx.widget().get::<f64>("min"),
                *ctx.widget().get::<f64>("max"),
            );
            ctx.widget().set("max", max);
            self.max = max;
            has_changes = true;
        }

        if (*ctx.widget().get::<f64>("val") - self.val).abs() > error {
            let val = adjust_val(
                *ctx.widget().get::<f64>("val"),
                *ctx.widget().get::<f64>("min"),
                *ctx.widget().get::<f64>("max"),
            );
            ctx.widget().set("val", val);
            self.val = val;
            has_changes = true;
        }

        has_changes
    }

    // adjust the thump position
    fn adjust_thumb_x(&self, ctx: &mut Context) {
        let val = *ctx.widget().get::<f64>("val");
        let min = *ctx.widget().get::<f64>("min");
        let max = *ctx.widget().get::<f64>("max");

        let thumb_width = ctx
            .get_widget(self.thumb)
            .get::<Rectangle>("bounds")
            .width();

        let track_width = ctx
            .get_widget(self.track)
            .get::<Rectangle>("bounds")
            .width();

        let thumb_x = calculate_thumb_x_from_val(val, min, max, track_width, thumb_width);

        ctx.get_widget(self.accent_track)
            .get_mut::<Constraint>("constraint")
            .set_width(thumb_x + 2.);

        ctx.get_widget(self.thumb)
            .get_mut::<Thickness>("margin")
            .set_left(thumb_x);
    }
}

impl State for SliderState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.thumb = ctx.child(ID_THUMB).entity();
        self.track = ctx.child(ID_TRACK).entity();
        self.accent_track = ctx.child(ID_ACCENT_TRACK).entity();

        // todo call update
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
                        let slider_x = ctx.widget().get::<Point>("position").x();

                        let thumb_x =
                            calculate_thumb_x(mouse_x, thumb_width, slider_x, track_width);

                        ctx.get_widget(self.accent_track)
                            .get_mut::<Constraint>("constraint")
                            .set_width(thumb_x + 2.);

                        ctx.get_widget(self.thumb)
                            .get_mut::<Thickness>("margin")
                            .set_left(thumb_x);

                        let min = *ctx.widget().get("min");
                        let max = *ctx.widget().get("max");

                        ctx.widget().set(
                            "val",
                            calculate_val(thumb_x, min, max, thumb_width, track_width),
                        );
                    } else {
                        ctx.widget().clear_dirty();
                    }
                }
            }

            self.action = None;
            return;
        }

        if self.adjust(ctx) {
            self.adjust_thumb_x(ctx);
        }
    }
}

widget!(
    /// The `Slider` allows to use a val in a range of values.
    ///
    /// **style:** `slider`
    ///
    /// # Example
    ///
    /// ```rust
    /// Slider::new().min(0).max(100).val(50).build(ctx)
    /// ```
    Slider<SliderState>: MouseHandler {
        /// Sets or shares the min val of the range.
        min: f64,

        /// Sets or shares the max val of the range.
        max: f64,

        /// Sets or shares the current val of the range.
        val: f64,

        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Indicates if the widget is hovered by the mouse cursor.
        hover: bool,

        /// Defines the margin around the inner border.
        container_margin: Thickness,

         /// Defines the margin around the accent border.
        accent_margin: Thickness,

        /// Defines the accent_brush
        accent_brush: Brush
    }
);

impl Template for Slider {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("Slider")
            .style(STYLE_SLIDER)
            .min(0.0)
            .max(100.0)
            .val(0.0)
            .height(24.0)
            .border_radius(2.0)
            .container_margin((0, 11, 0, 11))
            .accent_margin((0, 11, 0, 11))
            .child(
                Grid::new()
                    .id(ID_TRACK)
                    .child(
                        Grid::new()
                            .margin((8, 0))
                            .child(
                                // background border
                                Container::new()
                                    .margin(("container_margin", id))
                                    .opacity(id)
                                    .border_radius(id)
                                    .background(id)
                                    .border_brush(id)
                                    .border_width(id)
                                    .build(ctx),
                            )
                            .child(
                                // accent border
                                Container::new()
                                    .id(ID_ACCENT_TRACK)
                                    .margin(("accent_margin", id))
                                    .h_align("start")
                                    .width(0)
                                    .opacity(id)
                                    .border_radius(id)
                                    .background(("accent_brush", id))
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(
                        Button::new()
                            .style("thumb")
                            .opacity(id)
                            .id(ID_THUMB)
                            .v_align("center")
                            .h_align("start")
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
                    .action(SliderAction::Move { mouse_x: p.x() });
                false
            })
    }
}

// --- Helpers --

fn adjust_val(val: f64, min: f64, max: f64) -> f64 {
    if val < min {
        return min;
    }

    if val > max {
        return max;
    }

    val
}

fn adjust_min(min: f64, max: f64) -> f64 {
    if min > max {
        return max;
    }

    min
}

fn adjust_max(min: f64, max: f64) -> f64 {
    if max < min {
        return min;
    }

    max
}

fn calculate_thumb_x(mouse_x: f64, thumb_width: f64, slider_x: f64, track_width: f64) -> f64 {
    (mouse_x - slider_x - thumb_width)
        .max(0.0)
        .min(track_width - thumb_width)
}

fn calculate_val(thumb_x: f64, min: f64, max: f64, thumb_width: f64, track_width: f64) -> f64 {
    thumb_x / (track_width - thumb_width) * (max - min)
}

fn calculate_thumb_x_from_val(
    val: f64,
    min: f64,
    max: f64,
    track_width: f64,
    thumb_width: f64,
) -> f64 {
    (val / (max - min)) * (track_width - thumb_width)
}

// --- Helpers --

#[cfg(test)]
mod tests {
    use super::*;

    const ERROR: f64 = f64::EPSILON;

    #[test]
    fn test_calculate_thumb_x() {
        assert!((0.0 - calculate_thumb_x(-1000.0, 32.0, 0.0, 100.0)).abs() < ERROR);
        assert!((0.0 - calculate_thumb_x(0.0, 32.0, 0.0, 100.0)).abs() < ERROR);
        assert!((18.0 - calculate_thumb_x(50.0, 32.0, 0.0, 100.0)).abs() < ERROR);
        assert!((36.0 - calculate_thumb_x(68.0, 32.0, 0.0, 100.0)).abs() < ERROR);
        assert!((68.0 - calculate_thumb_x(100.0, 32.0, 0.0, 100.0)).abs() < ERROR);
        assert!((68.0 - calculate_thumb_x(1000.0, 32.0, 0.0, 100.0)).abs() < ERROR);
    }

    #[test]
    fn test_calculate_val() {
        assert!((0.0 - calculate_val(0.0, 0.0, 100.0, 32.0, 100.0)).abs() < ERROR);
        assert!((50.0 - calculate_val(34.0, 0.0, 100.0, 32.0, 100.0)).abs() < ERROR);
        assert!((100.0 - calculate_val(68.0, 0.0, 100.0, 32.0, 100.0)).abs() < ERROR);
        assert!((0.0 - calculate_val(0.0, -50.0, 50.0, 32.0, 100.0)).abs() < ERROR);
        assert!((50.0 - calculate_val(34.0, -50.0, 50.0, 32.0, 100.0)).abs() < ERROR);
        assert!((100.0 - calculate_val(68.0, -50.0, 50.0, 32.0, 100.0)).abs() < ERROR);
    }

    #[test]
    fn test_adjust_val() {
        assert!((0.0 - adjust_val(-10.0, 0.0, 100.0)).abs() < ERROR);
        assert!((10.0 - adjust_val(10.0, 0.0, 100.0)).abs() < ERROR);
        assert!((100.0 - adjust_val(500.0, 0.0, 100.0)).abs() < ERROR);
    }

    #[test]
    fn test_adjust_min() {
        assert!((0.0 - adjust_min(0.0, 100.0)).abs() < ERROR);
        assert!((5.0 - adjust_min(5.0, 100.0)).abs() < ERROR);
        assert!((100.0 - adjust_min(500.0, 100.0)).abs() < ERROR);
    }

    #[test]
    fn test_adjust_max() {
        assert!((100.0 - adjust_max(0.0, 100.0)).abs() < ERROR);
        assert!((100.0 - adjust_max(100.0, 5.0)).abs() < ERROR);
        assert!((100.0 - adjust_max(0.0, 100.0)).abs() < ERROR);
    }

    #[test]
    fn test_calculate_thumb_x_from_val() {
        assert!((0.0 - calculate_thumb_x_from_val(0.0, 0.0, 100.0, 100.0, 32.0)).abs() < ERROR);
        assert!((34.0 - calculate_thumb_x_from_val(50.0, 0.0, 100.0, 100.0, 32.0)).abs() < ERROR);
        assert!((68.0 - calculate_thumb_x_from_val(100.0, 0.0, 100.0, 100.0, 32.0)).abs() < ERROR);
    }
}
