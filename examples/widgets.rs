use std::cell::Cell;

use orbtk::prelude::*;

#[derive(Default)]
pub struct MainViewState {
    counter: Cell<i32>,
}

impl MainViewState {
    fn increment(&self) {
        self.counter.set(self.counter.get() + 1)
    }
}

impl State for MainViewState {
    fn update(&self, context: &mut Context<'_>) {
        if let Some(button_count_text) = context.widget().try_get_mut::<Text>() {
            button_count_text.0 = String16::from(format!("Button count: {}", self.counter.get()));
        }
    }
}

fn create_header(context: &mut BuildContext, text: &str, grid: usize, column: usize) -> Entity {
    TextBlock::create()
        .text(text)
        .selector(SelectorValue::new().with("text-block").class("h1"))
        .attach(GridColumn(grid))
        .attach(GridRow(column))
        .build(context)
}

widget!(
    MainView<MainViewState> {
        count_text: Text
    }
);

impl Template for MainView {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        let state = self.clone_state();

        self.name("MainView").count_text("Button count: 0").child(
            Grid::create()
                .margin(8.0)
                .columns(
                    Columns::create()
                        .column("Auto")
                        .column(32.0)
                        .column("Auto")
                        .column("*")
                        .build(),
                )
                .rows(
                    Rows::create()
                        .row("Auto")
                        .row("Auto")
                        .row("Auto")
                        .row("Auto")
                        .row("Auto")
                        .row("Auto")
                        .build(),
                )
                // Column 0
                .child(create_header(context, "Button", 0, 0))
                .child(
                    Button::create()
                        .text("Button")
                        .margin((0.0, 8.0, 0.0, 0.0))
                        .icon(material_font_icons::CHECK_FONT_ICON)
                        .attach(GridColumn(0))
                        .attach(GridRow(1))
                        .on_click(move |_| {
                            state.increment();
                            true
                        })
                        .build(context),
                )
                .child(
                    Button::create()
                        .text("Primary")
                        .selector(SelectorValue::new().with("button").class("primary"))
                        .margin((0.0, 8.0, 0.0, 0.0))
                        .icon(material_font_icons::CHECK_FONT_ICON)
                        .attach(GridColumn(0))
                        .attach(GridRow(2))
                        .build(context),
                )
                .child(
                    ToggleButton::create()
                        .text("ToggleButton")
                        .margin((0.0, 8.0, 0.0, 0.0))
                        .attach(GridColumn(0))
                        .attach(GridRow(3))
                        .build(context),
                )
                .child(
                    CheckBox::create()
                        .text("CheckBox")
                        .margin((0.0, 8.0, 0.0, 0.0))
                        .attach(GridColumn(0))
                        .attach(GridRow(4))
                        .build(context),
                )
                .child(
                    Switch::create()
                        .margin((0.0, 8.0, 0.0, 0.0))
                        .attach(GridColumn(0))
                        .attach(GridRow(5))
                        .build(context),
                )
                // Column 2
                .child(create_header(context, "Text", 2, 0))
                .child(
                    TextBlock::create()
                        .selector(SelectorValue::new().class("body"))
                        .text(id)
                        .margin((0.0, 8.0, 0.0, 0.0))
                        .attach(GridColumn(2))
                        .attach(GridRow(1))
                        .build(context),
                )
                .child(
                    TextBox::create()
                        .placeholder("TextBox...")
                        .text("")
                        .margin((0.0, 8.0, 0.0, 0.0))
                        .attach(GridColumn(2))
                        .attach(GridRow(2))
                        .build(context),
                )
                .build(context),
        )
    }
}

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - widgets example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .resizeable(true)
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}
