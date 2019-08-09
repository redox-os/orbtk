use std::cell::Cell;

use orbtk::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Action {
    AddItem,
    RemoveItem,
    IncrementCounter,
}

pub struct MainViewState {
    counter: Cell<i32>,
    list: Vec<String>,
    action: Cell<Option<Action>>,
}

impl Default for MainViewState {
    fn default() -> Self {
        MainViewState {
            counter: Cell::new(0),
            list: vec![
                "Item 1".to_string(),
                "Item 2".to_string(),
                "Item 3".to_string(),
            ],
            action: Cell::new(None),
        }
    }
}

impl MainViewState {
    fn action(&self, action: impl Into<Option<Action>>) {
        self.action.set(action.into());
    }
}

impl State for MainViewState {
    fn update(&self, context: &mut Context<'_>) {
        if let Some(action) = self.action.get() {
            match action {
                Action::AddItem => (),
                Action::RemoveItem => (),
                Action::IncrementCounter => {
                    self.counter.set(self.counter.get() + 1);
                    if let Some(button_count_text) = context.widget().try_get_mut::<Text>() {
                        button_count_text.0 =
                            String16::from(format!("Button count: {}", self.counter.get()));
                    }
                }
            }

            self.action.set(None);
        }
    }
}

fn create_header(context: &mut BuildContext, text: &str) -> Entity {
    TextBlock::create()
        .text(text)
        .selector(SelectorValue::new().with("text-block").class("h1"))
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
        let list_state = self.clone_state();
        let list_count = list_state.list.len();

        self.name("MainView").count_text("Button count: 0").child(
            Grid::create()
                .margin(8.0)
                .columns(
                    Columns::create()
                        .column("Auto")
                        .column(16.0)
                        .column("Auto")
                        .column(16.0)
                        .column("Auto")
                        .build(),
                )
                .child(
                    Stack::create()
                        .attach(GridColumn(0))
                        // Column 0
                        .child(create_header(context, "Buttons"))
                        .child(
                            Button::create()
                                .text("Button")
                                .margin((0.0, 8.0, 0.0, 0.0))
                                .icon(material_font_icons::CHECK_FONT_ICON)
                                .attach(GridColumn(0))
                                .attach(GridRow(1))
                                .on_click(move |_| {
                                    state.action(Action::IncrementCounter);
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
                        .build(context),
                )
                .child(
                    Stack::create()
                        .attach(GridColumn(2))
                        .child(create_header(context, "Text"))
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
                .child(
                    Grid::create()
                        .rows(Rows::create().row("Auto").row("Auto").row("Auto").build())
                        .columns(
                            Columns::create()
                                .column("*")
                                .column(4.0)
                                .column("*")
                                .build(),
                        )
                        .attach(GridColumn(4))
                        .child(
                            TextBlock::create()
                                .text("Items")
                                .selector(SelectorValue::new().with("text-block").class("h1"))
                                .attach(GridColumn(0))
                                .attach(ColumnSpan(3))
                                .attach(GridRow(0))
                                .build(context),
                        )
                        .child(
                            ItemsWidget::create()
                                .padding((4.0, 4.0, 4.0, 2.0))
                                .attach(GridColumn(0))
                                .attach(ColumnSpan(3))
                                .attach(GridRow(1))
                                .vertical_alignment("Start")
                                .margin((0.0, 8.0, 0.0, 8.0))
                                .items_builder(move |bc, index| {
                                    Button::create()
                                        .margin((0.0, 0.0, 0.0, 2.0))
                                        .text(list_state.list[index].as_str())
                                        .build(bc)
                                })
                                .items_count(list_count)
                                .build(context),
                        )
                        .child(
                            Button::create()
                                .icon(material_font_icons::MINUS_FONT_ICON)
                                .min_width(0.0)
                                .attach(GridColumn(0))
                                .attach(GridRow(2))
                                .build(context),
                        )
                        .child(
                            Button::create()
                                .icon(material_font_icons::ADD_FONT_ICON)
                                .min_width(0.0)
                                .attach(GridColumn(2))
                                .attach(GridRow(2))
                                .build(context),
                        )
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
                .size(468.0, 730.0)
                .resizeable(true)
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}
