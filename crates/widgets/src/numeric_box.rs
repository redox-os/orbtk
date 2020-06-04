use super::behaviors::MouseBehavior;
use crate::prelude::*;
use crate::shell::{Key, KeyEvent};
use core::f64::MAX;
use rust_decimal::prelude::*;

pub static ID_INPUT: &'static str = "numeric_box_input";
pub static ELEMENT_INPUT: &'static str = "numeric_box_input";
pub static ELEMENT_BTN: &'static str = "numeric_box_button";
// one mouse up scroll is delta.y = 12.0
static ONE_SCROLL: f64 = 12.0;

enum InputAction {
    Inc,
    Dec,
    ChangeByKey(KeyEvent),
    ChangeByMouseScroll(Point),
    Focus,
}

#[derive(Default, AsAny)]
struct NumericBoxState {
    action: Option<InputAction>,
    pub input: Entity,
    min: Decimal,
    max: Decimal,
    step: Decimal,
    current_value: Decimal,
}

impl NumericBoxState {
    fn action(&mut self, action: InputAction) {
        self.action = Some(action);
    }

    fn change_val(&mut self, new_value: Decimal, ctx: &mut Context<'_>) {
        if new_value >= self.min && new_value <= self.max {
            self.current_value = new_value;
            ctx.get_widget(self.input)
                .set::<String16>("text", String16::from(self.current_value.to_string()));
        }
    }

    fn request_focus(&self, ctx: &mut Context<'_>) {
        if !ctx.widget().get::<bool>("focused") {
            ctx.widget().set::<bool>("focused", true);
            ctx.push_event_by_window(FocusEvent::RequestFocus(ctx.entity));
        }
    }
}

fn default_or(key: &str, default_value: f64, ctx: &mut Context<'_>) -> Decimal {
    let property = ctx.widget().clone_or_default(key);

    match Decimal::from_f64(property) {
        Some(val) => val,
        None => Decimal::from_f64(default_value).unwrap(),
    }
}

impl State for NumericBoxState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        self.input = ctx.entity_of_child(ID_INPUT).expect(
            "NumericBoxState
        .init(): the child input could not be found!",
        );
        self.min = default_or("min", 0.0, ctx);
        self.max = default_or("max", MAX, ctx);
        self.step = default_or("step", 1.0, ctx);
        self.current_value = default_or("val", 0.0, ctx);

        let init_value = String16::from(self.current_value.to_string());
        ctx.get_widget(self.input)
            .set::<String16>("text", init_value);
    }

    // TODO: let the user type the value, or select it for cut, copy, paste operations
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
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
                    _ => {}
                },
                InputAction::ChangeByMouseScroll(delta) => {
                    match Decimal::from_f64(delta.y / ONE_SCROLL) {
                        Some(scroll_count) => {
                            self.change_val(self.current_value + (self.step * scroll_count), ctx);
                        }
                        None => {}
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
    /// NumericBox::create().build(ctx)
    /// ```
    ///
    /// Create a NumericBox with custom values:
    /// ```rust
    /// NumericBox::create().min(10.0).max(100.0).val(50.0).step(5.0).build(ctx)
    /// ```
    NumericBox<NumericBoxState>: KeyDownHandler {
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

        /// Sets or shares the minimum allowed value property
        min: f64,

        /// Sets or shares the maximum allowed value property
        max: f64,

        /// Sets or shares the stepping value property
        step: f64,

        /// Sets or shares the current value property
        val: f64
    }
);

impl Template for NumericBox {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("NumericBox")
            .background("transparent")
            .foreground(colors::LINK_WATER_COLOR)
            .border_brush("#647b91")
            .border_width(1.0)
            .border_radius(3.0)
            .element("numeric_box")
            .focused(false)
            .height(32.0)
            .margin(4.0)
            .min(0.0)
            .max(MAX)
            .step(1.0)
            .val(0.0)
            .width(100.0)
            .child(
                MouseBehavior::create()
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
                Stack::create()
                    .orientation("horizontal")
                    .spacing(0.0)
                    .child(
                        TextBox::create()
                            .border_brush(id)
                            .border_radius(id)
                            .border_width(id)
                            .element(ELEMENT_INPUT)
                            .enabled(false)
                            .id(ID_INPUT)
                            .max_width(64.0)
                            .text("0")
                            .build(ctx),
                    )
                    .child(
                        Stack::create()
                            .orientation("vertical")
                            .spacing(0.0)
                            .child(
                                Button::create()
                                    .border_brush("transparent")
                                    .border_radius(0.0)
                                    .border_width(0.0)
                                    .class("single_content")
                                    .element(ELEMENT_BTN)
                                    .max_width(32.0)
                                    .margin(1.0)
                                    .padding(0.0)
                                    .text("+")
                                    .on_click(move |states, _| {
                                        states
                                            .get_mut::<NumericBoxState>(id)
                                            .action(InputAction::Inc);
                                        true
                                    })
                                    .build(ctx),
                            )
                            .child(
                                Button::create()
                                    .border_brush("transparent")
                                    .border_radius(0.0)
                                    .border_width(0.0)
                                    .class("single_content")
                                    .element(ELEMENT_BTN)
                                    .max_width(32.0)
                                    .margin(1.0)
                                    .padding(0.0)
                                    .text("-")
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
        Box::new(RectangleRenderObject)
    }
}
