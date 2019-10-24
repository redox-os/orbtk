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
    fn update(&self, context: &mut Context<'_>) {
        if let Some(action) = self.action.get() {
            match action {
                Action::AddItem => {
                    let len = self.list.borrow().len();
                    if len < 5 {
                        self.list.borrow_mut().push(format!("Item {}", len + 1));
                        context.child_by_id("items").unwrap().set("count", Count(len + 1));

                        if len == 0 {
                            context
                                .child_by_id("remove-item-button")
                                .unwrap()
                                .set("enabled", Enabled(true));
                        }

                        if len == 4 {
                            context
                                .child_by_id("add-item-button")
                                .unwrap()
                                .set("enabled", Enabled(false));
                        }
                    }
                }
                Action::RemoveItem => {
                    let len = self.list.borrow().len();
                    self.list.borrow_mut().remove(len - 1);
                    context.child_by_id("items").unwrap().set("count", Count(len - 1));

                    if len == 1 {
                        context
                            .child_by_id("remove-item-button")
                            .unwrap()
                            .set("enabled", Enabled(false));
                    }

                    if len < 6 {
                        context
                            .child_by_id("add-item-button")
                            .unwrap()
                            .set("enabled", Enabled(true));
                    }
                }
                Action::IncrementCounter => {
                    self.counter.set(self.counter.get() + 1);
                    if let Some(button_count_text) = context.widget().try_get_mut::<Text>("text") {
                        button_count_text.0 =
                            String16::from(format!("Button count: {}", self.counter.get()));
                    }
                }
            }

            self.action.set(None);
        }
    }

    fn update_post_layout(&self, context: &mut Context<'_>) {
        let mut selection_string = "Selected:".to_string();

        for index in &context.widget().get::<SelectedIndices>("selected_indices").0 {
            selection_string = format!("{} {}", selection_string, index);
        }

        context
            .child_by_id("selection")
            .unwrap()
            .set("text", Text(String16::from(selection_string)));
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
        selected_indices: SelectedIndices,
        text: Text
    }
);

impl Template for MainView {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        let state = self.clone_state();
        let add_item_state = self.clone_state();
        let remove_item_state = self.clone_state();
        let list_state = self.clone_state();
        let list_view_state = self.clone_state();
        let list_count = list_state.list.borrow().len();
        let selection_list_count = list_state.selection_list.borrow().len();

        self.name("MainView")
            .text("Button count: 0")
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
                            .attach("grid_column", GridColumn(0))
                            // Column 0
                            .child(create_header(context, "Buttons"))
                            .child(
                                Button::create()
                                    .text("Button")
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .icon(material_font_icons::CHECK_FONT_ICON)
                                    .attach("grid_column", GridColumn(0))
                                    .attach("grid_row", GridRow(1))
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
                                    .attach("grid_column", GridColumn(0))
                                    .attach("grid_row", GridRow(2))
                                    .build(context),
                            )
                            .child(
                                ToggleButton::create()
                                    .text("ToggleButton")
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .attach("grid_column", GridColumn(0))
                                    .attach("grid_row", GridRow(3))
                                    .build(context),
                            )
                            .child(
                                CheckBox::create()
                                    .text("CheckBox")
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .attach("grid_column", GridColumn(0))
                                    .attach("grid_row", GridRow(4))
                                    .build(context),
                            )
                            .child(
                                Switch::create()
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .attach("grid_column", GridColumn(0))
                                    .attach("grid_row", GridRow(5))
                                    .build(context),
                            )
                            .build(context),
                    )
                    .child(
                        Stack::create()
                            .attach("grid_column", GridColumn(2))
                            .child(create_header(context, "Text"))
                            .child(
                                TextBlock::create()
                                    .selector(SelectorValue::new().class("body"))
                                    .text(id)
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .attach("grid_column", GridColumn(2))
                                    .attach("grid_row", GridRow(1))
                                    .build(context),
                            )
                            .child(
                                TextBox::create()
                                    .placeholder("TextBox...")
                                    .text("")
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .attach("grid_column", GridColumn(2))
                                    .attach("grid_row", GridRow(2))
                                    .build(context),
                            )
                            .child(
                                TextBox::create()
                                    .placeholder("TextBox...")
                                    .text("")
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .attach("grid_column", GridColumn(2))
                                    .attach("grid_row", GridRow(2))
                                    .build(context),
                            )
                            .build(context),
                    )
                    .child(
                        Grid::create()
                            .rows(
                                Rows::create()
                                    .row("Auto")
                                    .row(192.0)
                                    .row("Auto")
                                    .row(192.0)
                                    .row("Auto")
                                    .build(),
                            )
                            .columns(
                                Columns::create()
                                    .column("*")
                                    .column(4.0)
                                    .column("*")
                                    .build(),
                            )
                            .attach("grid_column", GridColumn(4))
                            .child(
                                TextBlock::create()
                                    .text("Items")
                                    .selector(SelectorValue::new().with("text-block").class("h1"))
                                    .attach("grid_column", GridColumn(0))
                                    .attach("column_span", ColumnSpan(3))
                                    .attach("grid_row", GridRow(0))
                                    .build(context),
                            )
                            .child(
                                ItemsWidget::create()
                                    .selector(Selector::from("items-widget").id("items"))
                                    .padding((4.0, 4.0, 4.0, 2.0))
                                    .attach("grid_column", GridColumn(0))
                                    .attach("column_span", ColumnSpan(3))
                                    .attach("grid_row", GridRow(1))
                                    .margin((0.0, 8.0, 0.0, 8.0))
                                    .items_builder(move |bc, index| {
                                        Button::create()
                                            .margin((0.0, 0.0, 0.0, 2.0))
                                            .text(list_state.list.borrow()[index].as_str())
                                            .build(bc)
                                    })
                                    .count(list_count)
                                    .build(context),
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
                                    .attach("grid_column", GridColumn(0))
                                    .attach("grid_row", GridRow(2))
                                    .build(context),
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
                                    .attach("grid_column", GridColumn(2))
                                    .attach("grid_row", GridRow(2))
                                    .build(context),
                            )
                            .child(
                                ListView::create()
                                    .attach("grid_column", GridColumn(0))
                                    .attach("column_span", ColumnSpan(3))
                                    .attach("grid_row", GridRow(3))
                                    .selected_indices(id)
                                    .margin((0.0, 16.0, 0.0, 8.0))
                                    .items_builder(move |bc, index| {
                                        TextBlock::create()
                                            .margin((0.0, 0.0, 0.0, 2.0))
                                            .vertical_alignment("Center")
                                            .text(
                                                list_view_state.selection_list.borrow()[index]
                                                    .as_str(),
                                            )
                                            .build(bc)
                                    })
                                    .count(selection_list_count)
                                    .build(context),
                            )
                            .child(
                                // todo: wrong text width????
                                TextBlock::create()
                                    .selector(Selector::from("text-block").id("selection"))
                                    .max_width(120.0)
                                    .attach("grid_column", GridColumn(0))
                                    .attach("column_span", ColumnSpan(3))
                                    .attach("column_span", ColumnSpan(2))
                                    .attach("grid_row", GridRow(4))
                                    .text("Selected:")
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
