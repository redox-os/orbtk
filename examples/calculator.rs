use orbtk::prelude::*;
use orbtk::theme::DEFAULT_THEME_CSS;

use std::cell::Cell;

static DARK_EXT: &'static str = include_str!("res/calculator-dark.css");

#[cfg(feature = "light-theme")]
static LIGHT_EXT: &'static str = include_str!("res/calculator-light.css");

#[cfg(not(feature = "light-theme"))]
fn get_theme() -> ThemeValue {
    ThemeValue::create_from_css(DEFAULT_THEME_CSS)
        .extension_css(DARK_EXT)
        .build()
}

#[cfg(feature = "light-theme")]
fn get_theme() -> ThemeValue {
    ThemeValue::create()
        .extension_css(DARK_EXT)
        .extension_css(LIGHT_EXT)
        .build()
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Digit(char),
    Operator(char),
}

#[derive(Default)]
pub struct MainViewState {
    input: RefCell<String>,
    operator: Cell<Option<char>>,
    left_side: Cell<Option<f64>>,
    right_side: Cell<Option<f64>>,
    action: Cell<Option<Action>>,
}

impl MainViewState {
    fn action(&self, action: impl Into<Option<Action>>) {
        self.action.set(action.into());
    }

    fn calculate(&self, context: &mut Context) {
        let mut result = 0.0;
        if let Some(operator) = self.operator.get() {
            if let Some(left_side) = self.left_side.get() {
                if let Some(right_side) = self.right_side.get() {
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

        context.widget().get_mut::<Text>().0 = result.to_string();
        self.left_side.set(Some(result));
        self.right_side.set(None);
    }
}

impl State for MainViewState {
    fn update(&self, context: &mut Context) {
        if let Some(action) = self.action.get() {
            match action {
                Action::Digit(digit) => {
                    self.input.borrow_mut().push(digit);
                    context
                        .child_by_id("input")
                        .unwrap()
                        .get_mut::<Text>()
                        .0
                        .push(digit);
                }
                Action::Operator(operator) => match operator {
                    'C' => {
                        self.input.borrow_mut().clear();
                        self.left_side.set(None);
                        self.operator.set(None);
                        self.right_side.set(None);
                        context.widget().get_mut::<Text>().0.clear();
                        context
                            .child_by_id("input")
                            .unwrap()
                            .get_mut::<Text>()
                            .0
                            .clear()
                    }
                    '=' => {
                        self.right_side
                            .set(Some(self.input.borrow().parse().unwrap_or(0.0)));
                        self.calculate(context);
                        self.input.borrow_mut().clear();
                        self.left_side.set(None);
                        self.operator.set(None);
                        self.right_side.set(None);
                        context
                            .child_by_id("input")
                            .unwrap()
                            .get_mut::<Text>()
                            .0
                            .clear()
                    }
                    _ => {
                        if self.input.borrow().is_empty() {
                            return;
                        }
                        if self.left_side.get().is_none() {
                            self.left_side
                                .set(Some(self.input.borrow().parse().unwrap_or(0.0)));
                        } else {
                            self.right_side
                                .set(Some(self.input.borrow().parse().unwrap_or(0.0)));
                            self.calculate(context);
                        }

                        context
                            .child_by_id("input")
                            .unwrap()
                            .get_mut::<Text>()
                            .0
                            .push(operator);
                        self.input.borrow_mut().clear();
                        self.operator.set(Some(operator));
                    }
                },
            }

            self.action.set(None);
        }
    }
}

fn get_button_selector(primary: bool) -> Selector {
    let selector = Selector::from("button");

    if primary {
        selector.class("primary")
    } else {
        selector
    }
}

fn generate_digit_button(
    context: &mut BuildContext,
    state: &Rc<MainViewState>,
    sight: char,
    primary: bool,
    column: usize,
    column_span: usize,
    row: usize,
) -> Entity {
    let state = state.clone();

    Button::create()
        .min_size(48.0, 48.0)
        .text(sight.to_string())
        .selector(get_button_selector(primary))
        .on_click(move |_| -> bool {
            state.action(Action::Digit(sight));
            true
        })
        .attach(GridColumn(column))
        .attach(GridRow(row))
        .attach(ColumnSpan(column_span))
        .build(context)
}

fn generate_operation_button(
    context: &mut BuildContext,
    state: &Rc<MainViewState>,
    sight: char,
    primary: bool,
    column: usize,
    column_span: usize,
    row: usize,
) -> Entity {
    let state = state.clone();
    Button::create()
        .min_size(48.0, 48.0)
        .text(sight.to_string())
        .selector(get_button_selector(primary).class("square"))
        .on_click(move |_| -> bool {
            state.action(Action::Operator(sight));
            true
        })
        .attach(GridColumn(column))
        .attach(ColumnSpan(column_span))
        .attach(GridRow(row))
        .build(context)
}

widget!(MainView<MainViewState> {
    result: Text
});

impl Template for MainView {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        let state = self.clone_state();

        self.name("MainView").result("").child(
            Grid::create()
                .rows(Rows::create().row(72.0).row("*").build())
                .child(
                    Container::create()
                        .padding(8.0)
                        .selector(Selector::from("container").class("header"))
                        .attach(GridRow(0))
                        .child(
                            Grid::create()
                                .child(
                                    ScrollViewer::create()
                                        .child(
                                            TextBlock::create()
                                                .width(0.0)
                                                .height(14.0)
                                                .text("")
                                                .selector(Selector::from("text-block").id("input"))
                                                .vertical_alignment("Start")
                                                .build(context),
                                        )
                                        .build(context),
                                )
                                .child(
                                    TextBlock::create()
                                        .selector(Selector::from("text-block"))
                                        .text(id)
                                        .vertical_alignment("End")
                                        .horizontal_alignment("End")
                                        .build(context),
                                )
                                .build(context),
                        )
                        .build(context),
                )
                .child(
                    Container::create()
                        .selector(Selector::from("container").class("content"))
                        .padding(8.0)
                        .attach(GridRow(1))
                        .child(
                            Grid::create()
                                .columns(
                                    Columns::create()
                                        .column(48.0)
                                        .column(4.0)
                                        .column(48.0)
                                        .column(4.0)
                                        .column(48.0)
                                        .column(4.0)
                                        .column(48.0)
                                        .build(),
                                )
                                .rows(
                                    Rows::create()
                                        .row(48.0)
                                        .row(4.0)
                                        .row(48.0)
                                        .row(4.0)
                                        .row(48.0)
                                        .row(4.0)
                                        .row(48.0)
                                        .row(4.0)
                                        .row(48.0)
                                        .build(),
                                )
                                // row 0
                                .child(generate_operation_button(
                                    context, &state, 'C', false, 0, 5, 0,
                                ))
                                .child(generate_operation_button(
                                    context, &state, '/', true, 6, 3, 0,
                                ))
                                // row 2
                                .child(generate_digit_button(context, &state, '7', false, 0, 1, 2))
                                .child(generate_digit_button(context, &state, '8', false, 2, 1, 2))
                                .child(generate_digit_button(context, &state, '9', false, 4, 1, 2))
                                .child(generate_operation_button(
                                    context, &state, '*', true, 6, 1, 2,
                                ))
                                // row 4
                                .child(generate_digit_button(context, &state, '4', false, 0, 1, 4))
                                .child(generate_digit_button(context, &state, '5', false, 2, 1, 4))
                                .child(generate_digit_button(context, &state, '6', false, 4, 1, 4))
                                .child(generate_operation_button(
                                    context, &state, '-', true, 6, 1, 4,
                                ))
                                // row 6
                                .child(generate_digit_button(context, &state, '1', false, 0, 1, 6))
                                .child(generate_digit_button(context, &state, '2', false, 2, 1, 6))
                                .child(generate_digit_button(context, &state, '3', false, 4, 1, 6))
                                .child(generate_operation_button(
                                    context, &state, '+', true, 6, 1, 6,
                                ))
                                // row 8
                                .child(generate_digit_button(context, &state, '0', false, 0, 3, 8))
                                .child(generate_digit_button(context, &state, '.', false, 4, 1, 8))
                                .child(generate_operation_button(
                                    context, &state, '=', true, 6, 1, 8,
                                ))
                                .build(context),
                        )
                        .build(context),
                )
                .build(context),
        )
    }
}

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - Calculator example")
                .position((100.0, 100.0))
                .size(212.0, 336.0)
                .theme(get_theme())
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}
