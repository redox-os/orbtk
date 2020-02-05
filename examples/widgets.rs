use std::collections::HashSet;

use orbtk::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Action {
    AddItem,
    ClearText,
    EntryActivated(Entity),
    EntryChanged(Entity),
    ValueChanged(Entity),
    IncrementCounter,
    RemoveItem,
}

#[derive(AsAny)]
pub struct MainViewState {
    action: Option<Action>,
}

impl Default for MainViewState {
    fn default() -> Self {
        MainViewState { action: None }
    }
}

impl MainViewState {
    fn action(&mut self, action: impl Into<Option<Action>>) {
        self.action = action.into();
    }
}

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if let Some(action) = self.action {
            match action {
                Action::AddItem => {
                    let len = ctx.widget().get::<List>("list").len();
                    if len < 5 {
                        ctx.widget()
                            .get_mut::<List>("list")
                            .push(format!("Item {}", len + 1));
                        ctx.child("items").set("count", len + 1);
                        ctx.child("remove-item-button").set("enabled", true);

                        if len == 4 {
                            ctx.child("add-item-button").set("enabled", false);
                        }
                    }
                }
                Action::RemoveItem => {
                    let len = ctx.widget().get::<List>("list").len();
                    if len > 0 {
                        ctx.widget().get_mut::<List>("list").remove(len - 1);
                        ctx.child("items").set("count", len - 1);
                        ctx.child("add-item-button").set("enabled", true);

                        if len == 1 {
                            ctx.child("remove-item-button").set("enabled", false);
                        }
                    }
                }
                Action::IncrementCounter => {
                    *ctx.widget().get_mut::<usize>("counter") += 1;

                    let counter = *ctx.widget().get::<usize>("counter");

                    ctx.widget().set(
                        "result",
                        String16::from(format!("Button count: {}", counter)),
                    );
                }
                Action::ClearText => {
                    ctx.widget().set("text_one", String16::from(""));
                    ctx.widget().set("text_two", String16::from(""));
                }
                Action::EntryActivated(entity) => {
                    let mut widget = ctx.get_widget(entity);
                    let text = widget.get_mut::<String16>("text");
                    println!("submitting {}", text);
                    text.clear();
                }
                Action::EntryChanged(entity) => {
                    let widget = ctx.get_widget(entity);
                    let text = widget.get::<String16>("text");
                    println!("entry changed: {}", text);
                }
                Action::ValueChanged(entity) => {
                    let value =
                        ((*ctx.get_widget(entity).get::<f64>("value")).floor() as i32).to_string();
                    ctx.child("value_text").set("text", String16::from(value));
                }
            }

            self.action = None;
        }
    }

    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        let mut selection_string = "Selected:".to_string();

        for index in &ctx.widget().get::<SelectedIndices>("selected_indices").0 {
            selection_string = format!("{} {}", selection_string, index);
        }

        ctx.child("selection")
            .set("text", String16::from(selection_string));
    }
}

fn create_header(ctx: &mut BuildContext, text: &str) -> Entity {
    TextBlock::create()
        .text(text)
        .selector(Selector::new().with("text-block").class("h1"))
        .build(ctx)
}

type List = Vec<String>;

widget!(
    MainView<MainViewState> {
        selected_indices: SelectedIndices,
        counter: usize,
        list_count: usize,
        list: List,
        selection_list: List,
        selection_list_count: usize,
        text_one: String16,
        text_two: String16,
        result: String16
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            .result("Button count: 0")
            .counter(0)
            .selected_indices(HashSet::new())
            .list(vec![
                "Item 1".to_string(),
                "Item 2".to_string(),
                "Item 3".to_string(),
            ])
            .list_count(3)
            .selection_list(vec![
                "Item 1".to_string(),
                "Item 2".to_string(),
                "Item 3".to_string(),
                "Item 4".to_string(),
                "Item 5".to_string(),
                "Item 6".to_string(),
                "Item 7".to_string(),
                "Item 8".to_string(),
                "Item 9".to_string(),
                "Item 10".to_string(),
            ])
            .selection_list_count(10)
            .child(
                Grid::create()
                    .margin(8.0)
                    .columns(
                        Columns::create()
                            .column(132.0)
                            .column(16.0)
                            .column(132.0)
                            .column(16.0)
                            .column(132.0)
                            .build(),
                    )
                    .child(
                        Stack::create()
                            .attach(Grid::column(0))
                            // Column 0
                            .child(create_header(ctx, "Buttons"))
                            .child(
                                Button::create()
                                    .text("Button")
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .icon(material_font_icons::CHECK_FONT_ICON)
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(1))
                                    .on_click(move |states, _| {
                                        state(id, states).action(Action::IncrementCounter);
                                        true
                                    })
                                    .build(ctx),
                            )
                            .child(
                                Button::create()
                                    .text("Primary")
                                    .selector(Selector::new().with("button").class("primary"))
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .icon(material_font_icons::CHECK_FONT_ICON)
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(2))
                                    .build(ctx),
                            )
                            .child(
                                ToggleButton::create()
                                    .text("ToggleButton")
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(3))
                                    .build(ctx),
                            )
                            .child(
                                CheckBox::create()
                                    .text("CheckBox")
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(4))
                                    .build(ctx),
                            )
                            .child(
                                Switch::create()
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(5))
                                    .build(ctx),
                            )
                            .child(
                                TextBlock::create()
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .selector(Selector::from("h1").id("value_text"))
                                    .text("0")
                                    .horizontal_alignment("center")
                                    .build(ctx),
                            )
                            .child(
                                Slider::create()
                                    .on_changed(move |states, entity| {
                                        state(id, states).action(Action::ValueChanged(entity));
                                    })
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(
                        Stack::create()
                            .attach(Grid::column(2))
                            .child(create_header(ctx, "Text"))
                            .child(
                                TextBlock::create()
                                    .selector(Selector::new().class("body"))
                                    .text(("result", id))
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .attach(Grid::column(2))
                                    .attach(Grid::row(1))
                                    .build(ctx),
                            )
                            .child(
                                TextBox::create()
                                    .water_mark("TextBox...")
                                    .text(("text_one", id))
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .attach(Grid::column(2))
                                    .attach(Grid::row(2))
                                    .on_activate(move |states, entity| {
                                        state(id, states).action(Action::EntryActivated(entity));
                                    })
                                    .on_changed(move |states, entity| {
                                        state(id, states).action(Action::EntryChanged(entity));
                                    })
                                    .build(ctx),
                            )
                            .child(
                                TextBox::create()
                                    .water_mark("TextBox...")
                                    .text(("text_two", id))
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .attach(Grid::column(2))
                                    .attach(Grid::row(2))
                                    .on_activate(move |states, entity| {
                                        state(id, states).action(Action::EntryActivated(entity));
                                    })
                                    .on_changed(move |states, entity| {
                                        state(id, states).action(Action::EntryChanged(entity));
                                    })
                                    .build(ctx),
                            )
                            .child(
                                Button::create()
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .text("clear text")
                                    .on_click(move |states, _| {
                                        state(id, states).action(Action::ClearText);
                                        true
                                    })
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(
                        Grid::create()
                            .rows(
                                Rows::create()
                                    .row("auto")
                                    .row(32.0)
                                    .row(16.0)
                                    .row(192.0)
                                    .row("auto")
                                    .row(192.0)
                                    .row("auto")
                                    .build(),
                            )
                            .columns(
                                Columns::create()
                                    .column("*")
                                    .column(4.0)
                                    .column("*")
                                    .build(),
                            )
                            .attach(Grid::column(4))
                            .child(
                                TextBlock::create()
                                    .text("Items")
                                    .selector(Selector::new().with("text-block").class("h1"))
                                    .attach(Grid::column(0))
                                    .attach(Grid::column_span(3))
                                    .attach(Grid::row(0))
                                    .build(ctx),
                            )
                            .child(
                                ComboBox::from_items_builder(move |bc, index| {
                                    let text = bc
                                        .get_widget(id)
                                        .get::<Vec<String>>("selection_list")[index]
                                        .clone();
                                    TextBlock::create()
                                        .margin((0.0, 0.0, 0.0, 2.0))
                                        .vertical_alignment("center")
                                        .text(text)
                                        .build(bc)
                                })
                                .attach(Grid::column(0))
                                .attach(Grid::column_span(3))
                                .attach(Grid::row(1))
                                .margin((0.0, 8.0, 0.0, 0.0))
                                .selected_indices(id)
                                .count(("selection_list_count", id))
                                .build(ctx),
                            )
                            .child(
                                ItemsWidget::create()
                                    .selector(Selector::from("items-widget").id("items"))
                                    .padding((4.0, 4.0, 4.0, 2.0))
                                    .attach(Grid::column(0))
                                    .attach(Grid::column_span(3))
                                    .attach(Grid::row(3))
                                    .margin((0.0, 0.0, 0.0, 8.0))
                                    .items_builder(move |bc, index| {
                                        let text = bc.get_widget(id).get::<Vec<String>>("list")
                                            [index]
                                            .clone();

                                        Button::create()
                                            .margin((0.0, 0.0, 0.0, 2.0))
                                            .text(text)
                                            .build(bc)
                                    })
                                    .count(("list_count", id))
                                    .build(ctx),
                            )
                            .child(
                                Button::create()
                                    .selector(Selector::from("button").id("remove-item-button"))
                                    .icon(material_font_icons::MINUS_FONT_ICON)
                                    .on_click(move |states, _| {
                                        state(id, states).action(Action::RemoveItem);
                                        true
                                    })
                                    .min_width(0.0)
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(4))
                                    .build(ctx),
                            )
                            .child(
                                Button::create()
                                    .selector(Selector::from("button").id("add-item-button"))
                                    .icon(material_font_icons::ADD_FONT_ICON)
                                    .on_click(move |states, _| {
                                        state(id, states).action(Action::AddItem);
                                        true
                                    })
                                    .min_width(0.0)
                                    .attach(Grid::column(2))
                                    .attach(Grid::row(4))
                                    .build(ctx),
                            )
                            .child(
                                ListView::create()
                                    .attach(Grid::column(0))
                                    .attach(Grid::column_span(3))
                                    .attach(Grid::row(5))
                                    .selected_indices(id)
                                    .margin((0.0, 16.0, 0.0, 8.0))
                                    .items_builder(move |bc, index| {
                                        let text = bc
                                            .get_widget(id)
                                            .get::<Vec<String>>("selection_list")[index]
                                            .clone();
                                        TextBlock::create()
                                            .margin((0.0, 0.0, 0.0, 2.0))
                                            .vertical_alignment("center")
                                            .text(text)
                                            .build(bc)
                                    })
                                    .count(("selection_list_count", id))
                                    .build(ctx),
                            )
                            .child(
                                // todo: wrong text width????
                                TextBlock::create()
                                    .selector(Selector::from("text-block").id("selection"))
                                    .max_width(120.0)
                                    .attach(Grid::column(0))
                                    .attach(Grid::column_span(3))
                                    .attach(Grid::row(6))
                                    .text("Selected:")
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx),
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

// helper to request MainViewState
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut MainViewState {
    states.get_mut(id)
}
