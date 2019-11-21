use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
};

use orbtk::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Action {
    AddItem,
    RemoveItem,
    IncrementCounter,
    ClearText,
}

pub struct MainViewState {
    counter: Cell<i32>,
    list: RefCell<Vec<String>>,
    selection_list: RefCell<Vec<String>>,
    action: Cell<Option<Action>>,
}

impl Default for MainViewState {
    fn default() -> Self {
        MainViewState {
            counter: Cell::new(0),
            list: RefCell::new(vec![
                "Item 1".to_string(),
                "Item 2".to_string(),
                "Item 3".to_string(),
            ]),
            selection_list: RefCell::new(vec![
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
            ]),
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
    fn update(&self, _: &mut Registry, ctx: &mut Context<'_>) {
        if let Some(action) = self.action.get() {
            match action {
                Action::AddItem => {
                    let len = self.list.borrow().len();
                    if len < 5 {
                        self.list.borrow_mut().push(format!("Item {}", len + 1));
                        ctx.child("items").set("count", len + 1);

                        if len == 0 {
                            ctx.child("remove-item-button").set("enabled", true);
                        }

                        if len == 4 {
                            ctx.child("add-item-button").set("enabled", false);
                        }
                    }
                }
                Action::RemoveItem => {
                    let len = self.list.borrow().len();
                    self.list.borrow_mut().remove(len - 1);
                    ctx.child("items").set("count", len - 1);

                    if len == 1 {
                        ctx.child("remove-item-button").set("enabled", false);
                    }

                    if len < 6 {
                        ctx.child("add-item-button").set("enabled", true);
                    }
                }
                Action::IncrementCounter => {
                    self.counter.set(self.counter.get() + 1);
                    ctx.widget().set(
                        "result",
                        String16::from(format!("Button count: {}", self.counter.get())),
                    );
                }
                Action::ClearText => {
                    ctx.widget().set("text_one", String16::from(""));
                    ctx.widget().set("text_two", String16::from(""));
                }
            }

            self.action.set(None);
        }
    }

    fn update_post_layout(&self, _: &mut Registry, ctx: &mut Context<'_>) {
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

widget!(
    MainView<MainViewState> {
        selected_indices: SelectedIndices,
        text_one: String16,
        text_two: String16,
        result: String16
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let state = self.clone_state();
        let clear_text_state = self.clone_state();
        let add_item_state = self.clone_state();
        let remove_item_state = self.clone_state();
        let list_state = self.clone_state();
        let list_view_state = self.clone_state();
        let list_count = list_state.list.borrow().len();
        let selection_list_count = list_state.selection_list.borrow().len();

        self.name("MainView")
            .result("Button count: 0")
            .selected_indices(HashSet::new())
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
                                    .on_mouse_move(move |_| {
                                        println!("ABc");
                                        true
                                    })
                                    .on_click(move |_| {
                                        state.action(Action::IncrementCounter);
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
                                    .build(ctx),
                            )
                            .child(
                                TextBox::create()
                                    .water_mark("TextBox...")
                                    .text(("text_two", id))
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .attach(Grid::column(2))
                                    .attach(Grid::row(2))
                                    .build(ctx),
                            )
                            .child(
                                Button::create()
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .text("clear text")
                                    .on_click(move |_| {
                                        clear_text_state.action(Action::ClearText);
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
                                ItemsWidget::create()
                                    .selector(Selector::from("items-widget").id("items"))
                                    .padding((4.0, 4.0, 4.0, 2.0))
                                    .attach(Grid::column(0))
                                    .attach(Grid::column_span(3))
                                    .attach(Grid::row(1))
                                    .margin((0.0, 8.0, 0.0, 8.0))
                                    .items_builder(move |bc, index| {
                                        Button::create()
                                            .margin((0.0, 0.0, 0.0, 2.0))
                                            .text(list_state.list.borrow()[index].as_str())
                                            .build(bc)
                                    })
                                    .count(list_count)
                                    .build(ctx),
                            )
                            .child(
                                Button::create()
                                    .selector(Selector::from("button").id("remove-item-button"))
                                    .icon(material_font_icons::MINUS_FONT_ICON)
                                    .on_click(move |_| {
                                        remove_item_state.action(Action::RemoveItem);
                                        true
                                    })
                                    .min_width(0.0)
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(2))
                                    .build(ctx),
                            )
                            .child(
                                Button::create()
                                    .selector(Selector::from("button").id("add-item-button"))
                                    .icon(material_font_icons::ADD_FONT_ICON)
                                    .on_click(move |_| {
                                        add_item_state.action(Action::AddItem);
                                        true
                                    })
                                    .min_width(0.0)
                                    .attach(Grid::column(2))
                                    .attach(Grid::row(2))
                                    .build(ctx),
                            )
                            .child(
                                ListView::create()
                                    .attach(Grid::column(0))
                                    .attach(Grid::column_span(3))
                                    .attach(Grid::row(3))
                                    .selected_indices(id)
                                    .margin((0.0, 16.0, 0.0, 8.0))
                                    .items_builder(move |bc, index| {
                                        TextBlock::create()
                                            .margin((0.0, 0.0, 0.0, 2.0))
                                            .vertical_alignment("center")
                                            .text(
                                                list_view_state.selection_list.borrow()[index]
                                                    .as_str(),
                                            )
                                            .build(bc)
                                    })
                                    .count(selection_list_count)
                                    .build(ctx),
                            )
                            .child(
                                // todo: wrong text width????
                                TextBlock::create()
                                    .selector(Selector::from("text-block").id("selection"))
                                    .max_width(120.0)
                                    .attach(Grid::column(0))
                                    .attach(Grid::column_span(3))
                                    .attach(Grid::row(4))
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
