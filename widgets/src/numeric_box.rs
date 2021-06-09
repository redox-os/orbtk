use core::f64::MAX;
use rust_decimal::prelude::*;

use super::behaviors::MouseBehavior;

use crate::{
    api::prelude::*, prelude::*, proc_macros::*, shell::prelude::*, theme_default::prelude::*,
};

// --- KEYS --

pub static ID_INPUT: &str = "numeric_box_input";
pub static STYLE_INPUT: &str = "numeric_box_input";
pub static STYLE_BTN: &str = "numeric_box_button";

// --- KEYS --

pub enum InputAction {
    Inc,
    Dec,
    ChangeByKey(KeyEvent),
    ChangeByMouseScroll(Point),
    Focus,
}

#[derive(Default, AsAny)]
pub struct NumericBoxState {
    pub action: Option<InputAction>,
    pub input: Entity,
    pub min: Decimal,
    pub max: Decimal,
    pub step: Decimal,
    pub current_value: Decimal,
    event_adapter: EventAdapter,
    window: Entity,
}

impl NumericBoxState {
    fn action(&mut self, action: InputAction) {
        self.action = Some(action);
    }

    fn change_val(&mut self, new_value: Decimal, ctx: &mut Context) {
        if self.current_value == self.min && new_value < self.min
            || self.current_value == self.max && new_value > self.max
        {
            return;
        }

        self.current_value = self.max(self.min(new_value));
        if let Some(val) = self.current_value.to_f64() {
            ctx.widget().set("val", val);
        }

        ctx.get_widget(self.input)
            .set::<String>("text", self.current_value.to_string());
    }

    fn min(&self, d: Decimal) -> Decimal {
        if d <= self.min {
            return self.min;
        }

        d
    }

    fn max(&self, d: Decimal) -> Decimal {
        if d >= self.max {
            return self.max;
        }

        d
    }

    fn request_focus(&self, ctx: &mut Context) {
        if !ctx.widget().get::<bool>("focused") {
            ctx.widget().set::<bool>("focused", true);
            self.event_adapter
                .push_event_direct(self.window, FocusEvent::RequestFocus(ctx.entity()));
        }
    }
}

fn default_or(key: &str, default_value: f64, ctx: &mut Context) -> Decimal {
    let property = ctx.widget().clone_or_default(key);

    match Decimal::from_f64(property) {
        Some(val) => val,
        None => Decimal::from_f64(default_value).unwrap(),
    }
}

impl State for NumericBoxState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.input = ctx.entity_of_child(ID_INPUT).expect(
            "NumericBoxState
        .init(): the child input could not be found!",
        );
        self.event_adapter = ctx.event_adapter();
        self.min = default_or("min", 0.0, ctx);
        self.max = default_or("max", MAX, ctx);
        self.step = default_or("step", 1.0, ctx);
        self.current_value = default_or("val", 0.0, ctx);

        let init_value = self.current_value.to_string();
        ctx.get_widget(self.input).set::<String>("text", init_value);
    }

    // TODO: let the user type the value, or select it for cut, copy, paste operations
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = &self.action {
            match action {
                InputAction::Inc => {
                    self.change_val(self.current_value + self.step, ctx);
                }
                InputAction::Dec => {
                    self.change_val(self.current_value - self.step, ctx);
                }
                InputAction::ChangeByKey(key_event) => match key_event.key {
                    Key::Up | Key::NumpadAdd => {
                        self.change_val(self.current_value + self.step, ctx);
                    }
                    Key::Down | Key::NumpadSubtract => {
                        self.change_val(self.current_value - self.step, ctx);
                    }
                    Key::Enter => {
                        if *ctx.widget().get::<bool>("lose_focus_on_activation") {
                            self.event_adapter.push_event_direct(
                                self.window,
                                FocusEvent::RequestFocus(ctx.entity()),
                            );
                        }

                        self.event_adapter
                            .push_event_direct(ctx.entity(), ActivateEvent(ctx.entity()));
                    }
                    _ => {}
                },
                InputAction::ChangeByMouseScroll(delta) => {
                    if delta.y() < 0.0 {
                        self.change_val(self.current_value - self.step, ctx);
                    } else {
                        self.change_val(self.current_value + self.step, ctx);
                    }
                }
                InputAction::Focus => {
                    self.request_focus(ctx);
                }
            }
            self.action = None;
        }
    }
}

widget!(
    /// `NumericBox` is used to let the user increase or decrease
    /// the value of the input by a given, fixed value called `step` until it reaches the upper or
    /// lower bounds.
    /// The widget can be controlled by clicking on the two control buttons, or the keybaord's
    /// Up and Down, Numpad+ and Numpad- keys, or the mouse scroll.
    /// Note: after the widget is initialized, changing the min, max or step properties has no effect.
    ///
    /// # Examples:
    /// Create a NumericBox with default values:
    /// ```rust
    /// NumericBox::new().build(ctx)
    /// ```
    ///
    /// Create a NumericBox with custom values:
    /// ```rust
    /// NumericBox::new().min(10.0).max(100.0).val(50.0).step(5.0).build(ctx)
    /// ```
    NumericBox<NumericBoxState>: ActivateHandler, KeyDownHandler {
        /// Sets or shares the background color property
        background: Brush,

        /// Sets or shares the border color property
        border_brush: Brush,

        /// Sets or shares the border width property
        border_width: Thickness,

        /// Sets or shares the border radius property
        border_radius: f64,

        /// Sets or shares the focused property
        focused: bool,

        /// Sets or shares the foreground color property
        foreground: Brush,

        /// Sets or shares the font size property.
        font_size: f64,

        /// Sets or shares the font property.
        font: String,

        /// Sets or shares the value that describes if the NumericBox should lose focus on activation (when enter pressed).
        lose_focus_on_activation: bool,

        /// Sets or shares the minimum allowed value property
        min: f64,

        /// Sets or shares the maximum allowed value property
        max: f64,

        /// Sets or shares the stepping value property
        step: f64,

        /// Sets or shares the current value property
        val: f64,

        /// Indicates if the widget is hovered by the mouse cursor.
        hover: bool,

        /// Represents the up icon of the up button.
        icon_up: String,

        /// Represents the up icon of the down button
        icon_down: String,

        padding: Thickness
    }
);

impl Template for NumericBox {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("NumericBox")
            .style("numeric_box")
            .background("transparent")
            .foreground(colors::LINK_WATER_COLOR)
            .border_brush("#647b91")
            .border_width(1.0)
            .border_radius(3.0)
            .focused(false)
            .height(32.0)
            .lose_focus_on_activation(true)
            .min(0.0)
            .max(200.0)
            .step(1.0)
            .val(0.0)
            .icon_up(material_icons_font::MD_KEYBOARD_ARROW_UP)
            .icon_down(material_icons_font::MD_KEYBOARD_ARROW_DOWN)
            .min_width(128.0)
            .child(
                MouseBehavior::new()
                    .on_mouse_down(move |states, _| {
                        states
                            .get_mut::<NumericBoxState>(id)
                            .action(InputAction::Focus);
                        true
                    })
                    .on_scroll(move |states, delta| {
                        states
                            .get_mut::<NumericBoxState>(id)
                            .action(InputAction::ChangeByMouseScroll(delta));
                        true
                    })
                    .build(ctx),
            )
            .child(
                Grid::new()
                    .columns("*, 32")
                    .rows("16, 16")
                    .child(
                        TextBox::new()
                            .margin(("padding", id))
                            .id(ID_INPUT)
                            .style("")
                            .attach(Grid::column(0))
                            .attach(Grid::row_span(2))
                            .attach(Grid::row(0))
                            .foreground(id)
                            .border_brush("transparent")
                            .border_width(0)
                            .font_size(id)
                            .background("transparent")
                            .h_align("stretch")
                            .enabled(false)
                            .v_align("center")
                            .text("0")
                            .lose_focus_on_activation(id)
                            .build(ctx),
                    )
                    .child(
                        Button::new()
                            .style("button_small")
                            .attach(Grid::column(1))
                            .attach(Grid::row(0))
                            .min_width(14)
                            .height(15)
                            .icon(("icon_up", id))
                            .margin(1)
                            .on_click(move |states, _| {
                                states
                                    .get_mut::<NumericBoxState>(id)
                                    .action(InputAction::Inc);
                                true
                            })
                            .build(ctx),
                    )
                    .child(
                        Button::new()
                            .style("button_small")
                            .attach(Grid::column(1))
                            .attach(Grid::row(1))
                            .min_width(14)
                            .height(15)
                            .padding(0.0)
                            .margin(1)
                            .icon(("icon_down", id))
                            .on_click(move |states, _| {
                                states
                                    .get_mut::<NumericBoxState>(id)
                                    .action(InputAction::Dec);
                                true
                            })
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .on_key_down(move |states, event| -> bool {
                states
                    .get_mut::<NumericBoxState>(id)
                    .action(InputAction::ChangeByKey(event));
                false
            })
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        RectangleRenderObject.into()
    }
}
