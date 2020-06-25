use orbtk::prelude::*;
use orbtk::theme::DEFAULT_THEME_CSS;

static DARK_EXT: &'static str = include_str!("../res/calculator-dark.css");

#[cfg(feature = "light-theme")]
static LIGHT_EXT: &'static str = include_str!("../res/calculator-light.css");

#[cfg(not(feature = "light-theme"))]
fn get_theme() -> Theme {
    Theme::create_from_css(DEFAULT_THEME_CSS)
        .extension_css(DARK_EXT)
        .build()
}

#[cfg(feature = "light-theme")]
fn get_theme() -> Theme {
    Theme::new()
        .extension_css(DARK_EXT)
        .extension_css(LIGHT_EXT)
        .build()
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Digit(char),
    Operator(char),
}

#[derive(Default, AsAny)]
pub struct MainViewState {
    input: String,
    operator: Option<char>,
    left_side: Option<f64>,
    right_side: Option<f64>,
    action: Option<Action>,
}

impl MainViewState {
    fn action(&mut self, action: impl Into<Option<Action>>) {
        self.action = action.into();
    }

    fn calculate(&mut self, ctx: &mut Context) {
        let mut result = 0.0;
        if let Some(operator) = self.operator {
            if let Some(left_side) = self.left_side {
                if let Some(right_side) = self.right_side {
                    match operator {
                        '+' => {
                            result = left_side + right_side;
                        }
                        '-' => {
                            result = left_side - right_side;
                        }
                        '*' => {
                            result = left_side * right_side;
                        }
                        '/' => {
                            result = left_side / right_side;
                        }
                        _ => {}
                    }
                }
            }
        }

        if result % 1.0 == 0.0 {
            main_view(ctx.widget()).set_text(format!("{}", result));
        } else {
            main_view(ctx.widget()).set_text(format!("{:.8}", result));
        }

        self.left_side = Some(result);
        self.right_side = None;
    }
}

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                Action::Digit(digit) => {
                    self.input.push(digit);
                    text_block(ctx.child("input")).text_mut().push(digit);
                }
                Action::Operator(operator) => match operator {
                    'C' => {
                        self.input.clear();
                        self.left_side = None;
                        self.operator = None;
                        self.right_side = None;
                        main_view(ctx.widget()).text_mut().clear();
                        text_block(ctx.child("input")).text_mut().clear()
                    }
                    '=' => {
                        self.right_side = Some(self.input.parse().unwrap_or(0.0));
                        self.calculate(ctx);
                        self.input.clear();
                        self.left_side = None;
                        self.operator = None;
                        self.right_side = None;
                        text_block(ctx.child("input")).text_mut().clear()
                    }
                    _ => {
                        if self.input.is_empty() {
                            return;
                        }
                        if self.left_side.is_none() {
                            self.left_side = Some(self.input.parse().unwrap_or(0.0));
                        } else {
                            self.right_side = Some(self.input.parse().unwrap_or(0.0));
                            self.calculate(ctx);
                        }

                        text_block(ctx.child("input")).text_mut().push(operator);
                        self.input.clear();
                        self.operator = Some(operator);
                    }
                },
            }

            self.action = None;
        }
    }
}

fn generate_digit_button(
    ctx: &mut BuildContext,
    id: Entity,
    sight: char,
    primary: bool,
    column: usize,
    column_span: usize,
    row: usize,
) -> Entity {
    let mut button = Button::new()
        .style("button_single_content")
        .min_size(48.0, 48.0)
        .text(sight.to_string())
        .on_click(move |states, _| -> bool {
            state(id, states).action(Action::Digit(sight));
            true
        })
        .attach(Grid::column(column))
        .attach(Grid::row(row))
        .attach(Grid::column_span(column_span));

    // if primary {
    //     button = button.style("button_primary");
    // }

    button.build(ctx)
}

fn generate_operation_button(
    ctx: &mut BuildContext,
    id: Entity,
    sight: char,
    primary: bool,
    column: usize,
    column_span: usize,
    row: usize,
) -> Entity {
    let mut button = Button::new()
        .style("button_single_content")
        .min_size(48.0, 48.0)
        .text(sight.to_string())
        .style("square")
        .on_click(move |states, _| -> bool {
            state(id, states).action(Action::Operator(sight));
            true
        })
        .attach(Grid::column(column))
        .attach(Grid::column_span(column_span))
        .attach(Grid::row(row));

    // if primary {
    //     button = button.style("button_primary");
    // }

    button.build(ctx)
}

widget!(MainView<MainViewState> {
    text: String16
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            .width(212.0)
            .height(336.0)
            .text("")
            .child(
                Grid::new()
                    .rows(Rows::new().add(72.0).add("*"))
                    .child(
                        Container::new()
                            .padding(8.0)
                            .style("container")
                            .style("header")
                            .attach(Grid::row(0))
                            .child(
                                Grid::new()
                                    .child(
                                        ScrollViewer::new()
                                            .scroll_viewer_mode(("custom", "disabled"))
                                            .child(
                                                TextBlock::new()
                                                    .width(0.0)
                                                    .height(14.0)
                                                    .text("")
                                                    .style("text-block")
                                                    .id("input")
                                                    .v_align("start")
                                                    .build(ctx),
                                            )
                                            .build(ctx),
                                    )
                                    .child(
                                        TextBlock::new()
                                            .style("text-block")
                                            .text(id)
                                            .v_align("end")
                                            .h_align("end")
                                            .build(ctx),
                                    )
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(
                        Container::new()
                            .style("container")
                            .style("content")
                            .padding(8.0)
                            .attach(Grid::row(1))
                            .child(
                                Grid::new()
                                    .columns(
                                        Columns::new()
                                            .add(48.0)
                                            .add(4.0)
                                            .add(48.0)
                                            .add(4.0)
                                            .add(48.0)
                                            .add(4.0)
                                            .add(48.0),
                                    )
                                    .rows(
                                        Rows::new()
                                            .add(48.0)
                                            .add(4.0)
                                            .add(48.0)
                                            .add(4.0)
                                            .add(48.0)
                                            .add(4.0)
                                            .add(48.0)
                                            .add(4.0)
                                            .add(48.0),
                                    )
                                    // row 0
                                    .child(generate_operation_button(ctx, id, 'C', false, 0, 5, 0))
                                    .child(generate_operation_button(ctx, id, '/', true, 6, 3, 0))
                                    // row 2
                                    .child(generate_digit_button(ctx, id, '7', false, 0, 1, 2))
                                    .child(generate_digit_button(ctx, id, '8', false, 2, 1, 2))
                                    .child(generate_digit_button(ctx, id, '9', false, 4, 1, 2))
                                    .child(generate_operation_button(ctx, id, '*', true, 6, 1, 2))
                                    // row 4
                                    .child(generate_digit_button(ctx, id, '4', false, 0, 1, 4))
                                    .child(generate_digit_button(ctx, id, '5', false, 2, 1, 4))
                                    .child(generate_digit_button(ctx, id, '6', false, 4, 1, 4))
                                    .child(generate_operation_button(ctx, id, '-', true, 6, 1, 4))
                                    // row 6
                                    .child(generate_digit_button(ctx, id, '1', false, 0, 1, 6))
                                    .child(generate_digit_button(ctx, id, '2', false, 2, 1, 6))
                                    .child(generate_digit_button(ctx, id, '3', false, 4, 1, 6))
                                    .child(generate_operation_button(ctx, id, '+', true, 6, 1, 6))
                                    // row 8
                                    .child(generate_digit_button(ctx, id, '0', false, 0, 3, 8))
                                    .child(generate_digit_button(ctx, id, '.', false, 4, 1, 8))
                                    .child(generate_operation_button(ctx, id, '=', true, 6, 1, 8))
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}

fn main() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk - Calculator example")
                .position((100.0, 100.0))
                .size(212.0, 336.0)
                .theme(get_theme())
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}

// helper to request MainViewState
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut MainViewState {
    states.get_mut(id)
}
