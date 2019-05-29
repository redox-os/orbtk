use orbtk::prelude::*;
use orbtk::theme::DEFAULT_THEME_CSS;

use std::cell::Cell;

static DARK_EXT: &'static str = include_str!("res/calculator-dark.css");

#[cfg(feature = "light-theme")]
static LIGHT_EXT: &'static str = include_str!("res/calculator-light.css");

#[cfg(not(feature = "light-theme"))]
fn get_theme() -> ThemeValue {
    ThemeValue::create_from_css(DEFAULT_THEME_CSS).extension_css(DARK_EXT).build()
}

#[cfg(feature = "light-theme")]
fn get_theme() -> ThemeValue {
    ThemeValue::create()
        .extension_css(DARK_EXT)
        .extension_css(LIGHT_EXT)
        .build()
}

#[derive(Default)]
pub struct MainViewState {
    result: RefCell<String>,
    input: RefCell<String>,
    eval: Cell<bool>,
    clear: Cell<bool>,
    updated: Cell<bool>,
}

impl MainViewState {
    fn clear(&self) {
        self.result.borrow_mut().clear();
        self.input.borrow_mut().clear();
        self.clear.set(true);
        self.updated.set(true);
    }

    fn eval(&self) {
        self.eval.set(true);
        self.updated.set(true);
    }
    fn input(&self, sight: &str) {
        *self.input.borrow_mut() = sight.to_string();
        self.updated.set(true);
    }
}

impl State for MainViewState {
    fn update(&self, context: &mut Context) {
        let mut result = None;

        if let Some(child) = &mut context.child_by_id("input") {
            if let Some(text) = child.try_get_mut::<Text>() {
                if self.clear.get() {
                    text.0.clear();
                    self.clear.set(false);
                } else if self.updated.get() {
                    text.0.push_str(&*self.input.borrow());
                }

                if self.eval.get() {
                    // let res = match calc::eval(&text.0) {
                    //     Ok(s) => s.to_string(),
                    //     Err(e) => e.into(),
                    // };

                    // result = Some(res);
                    self.eval.set(false);
                }

                self.input.borrow_mut().clear();
            }
        }

        if let Some(result) = result {
            *self.result.borrow_mut() = result;
        }

        if self.updated.get() || self.clear.get() {
            if let Some(text) = context.widget().try_get_mut::<Text>() {
                text.0 = self.result.borrow().clone();
            }
        }

        self.updated.set(false);
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

fn generate_button(
    context: &mut BuildContext,
    state: &Rc<MainViewState>,
    sight: &str,
    primary: bool,
    column: usize,
    row: usize,
) -> Entity {
    let sight = String::from(sight);
    let state = state.clone();

    Button::create()
        .min_size(48.0, 48.0)
        .max_size(48.0, 48.0)
        .size(48.0, 48.0)
        .text(sight.clone())
        .selector(get_button_selector(primary))
        .on_click(move |_| -> bool {
            state.input(&String::from(sight.clone()));
            true
        })
        .attach(GridColumn(column))
        .attach(GridRow(row))
        .build(context)
}

fn generate_operation_button(sight: &str, primary: bool, column: usize, row: usize) -> Button {
    Button::create()
        .min_size(48.0, 48.0)
        .max_size(48.0, 48.0)
        .size(48.0, 48.0)
        .text(sight.to_string())
        .selector(get_button_selector(primary).class("square"))
        .attach(GridColumn(column))
        .attach(GridRow(row))
}

widget!(MainView<MainViewState> {
    text: Text
});

impl Template for MainView {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        let state = self.clone_state();
        let clear_state = state.clone();

        self.name("MainView").text("").child(
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
                                    TextBlock::create()
                                        .width(0.0)
                                        .height(14.0)
                                        .text("")
                                        .selector(Selector::from("text-block").id("input"))
                                        .vertical_alignment("Start")
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
                                .child(generate_button(context, &state, "(", false, 0, 0))
                                .child(generate_button(context, &state, ")", false, 2, 0))
                                .child(generate_button(context, &state, "^", false, 4, 0))
                                .child(generate_button(context, &state, "/", true, 6, 0))
                                // row 2
                                .child(generate_button(context, &state, "7", false, 0, 2))
                                .child(generate_button(context, &state, "8", false, 2, 2))
                                .child(generate_button(context, &state, "9", false, 4, 2))
                                .child(generate_button(context, &state, "*", true, 6, 2))
                                // row 4
                                .child(generate_button(context, &state, "4", false, 0, 4))
                                .child(generate_button(context, &state, "5", false, 2, 4))
                                .child(generate_button(context, &state, "6", false, 4, 4))
                                .child(generate_button(context, &state, "-", true, 6, 4))
                                // row 6
                                .child(generate_button(context, &state, "1", false, 0, 6))
                                .child(generate_button(context, &state, "2", false, 2, 6))
                                .child(generate_button(context, &state, "3", false, 4, 6))
                                .child(generate_button(context, &state, "+", true, 6, 6))
                                // row 8
                                .child(generate_button(context, &state, "0", false, 0, 8))
                                .child(generate_button(context, &state, ".", false, 2, 8))
                                .child(
                                    generate_operation_button("C", false, 4, 8)
                                        .on_click(move |_| {
                                            clear_state.clear();
                                            true
                                        })
                                        .build(context),
                                )
                                .child(
                                    generate_operation_button("=", true, 6, 8)
                                        .on_click(move |_| {
                                            state.eval();
                                            true
                                        })
                                        .build(context),
                                )
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
                .title("Calculator")
                .position((100.0, 100.0))
                .size(220.0, 344.0)
                .theme(get_theme())
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}
