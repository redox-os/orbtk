use std::collections::HashSet;

use orbtk::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Action {
    AddItem,
    ClearText,
    EntryActivated(Entity),
    IncrementCounter,
    RemoveItem,
    ToggleTheme(Entity),
    SelectionChanged,
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
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                Action::AddItem => {
                    let len = MainView::list_ref(&ctx.widget()).len();

                    if len < 5 {
                        MainView::list_mut(&mut ctx.widget()).push(format!("Item {}", len + 1));
                        ctx.child("items").clone_or_default::<usize>("Item");
                        ItemsWidget::count_set(&mut ctx.child("items"), len + 1);
                        Button::enabled_set(&mut ctx.child("remove-item-button"), true);

                        if len == 4 {
                            Button::enabled_set(&mut ctx.child("add-item-button"), false);
                        }
                    }
                }
                Action::RemoveItem => {
                    let len = MainView::list_ref(&ctx.widget()).len();
                    if len > 0 {
                        MainView::list_mut(&mut ctx.widget()).remove(len - 1);
                        ItemsWidget::count_set(&mut ctx.child("items"), len - 1);
                        Button::enabled_set(&mut ctx.child("add-item-button"), true);

                        if len == 1 {
                            Button::enabled_set(&mut ctx.child("remove-item-button"), false);
                        }
                    }
                }
                Action::IncrementCounter => {
                    ctx.send_window_request(orbtk::shell::WindowRequest::Redraw);
                    *MainView::counter_mut(&mut ctx.widget()) += 1;

                    let counter = *MainView::counter_ref(&ctx.widget());

                    MainView::result_set(&mut ctx.widget(), format!("Button count: {}", counter));
                }
                Action::ClearText => {
                    MainView::text_one_set(&mut ctx.widget(), String::default());
                    MainView::text_two_set(&mut ctx.widget(), String::default());
                }
                Action::EntryActivated(entity) => {
                    let text = TextBox::text_clone(&ctx.get_widget(entity));
                    println!("submitting {}", text);
                    TextBox::text_mut(&mut ctx.get_widget(entity)).clear();
                }
                Action::ToggleTheme(entity) => {
                    let light = *ctx.get_widget(entity).get::<bool>("selected");

                    let theme = if light { light_theme() } else { dark_theme() };
                    ctx.switch_theme(theme);
                }
                Action::SelectionChanged => {
                    let mut selection_string = "Selected:".to_string();

                    for index in &MainView::selected_indices_ref(&ctx.widget()).0 {
                        selection_string = format!("{} {}", selection_string, index);
                    }

                    TextBlock::text_set(&mut ctx.child("selection"), selection_string);
                }
            }

            self.action = None;
        }
    }
}

fn create_header(ctx: &mut BuildContext, text: &str) -> Entity {
    TextBlock::new().text(text).style("header").build(ctx)
}

type List = Vec<String>;

widget!(
    MainView<MainViewState> {
        selected_indices: SelectedIndices,
        counter: usize,
        list_count: usize,
        combo_box_list_count: usize,
        list: List,
        selection_list: List,
        combo_box_list: List,
        selection_list_count: usize,
        text_one: String,
        text_two: String,
        result: String
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let slider = Slider::new()
            .min(0.0)
            .max(1.0)
            // .on_changed(move |states, entity| {
            //     state(id, states).action(Action::ValueChanged(entity));
            // })
            .build(ctx);

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
                "Select Item 1".to_string(),
                "Select Item 2".to_string(),
                "Select Item 3".to_string(),
                "Select Item 4".to_string(),
                "Select Item 5".to_string(),
                "Select Item 6".to_string(),
                "Select Item 7".to_string(),
                "Select Item 8".to_string(),
                "Select Item 9".to_string(),
                "Select Item 10".to_string(),
            ])
            .combo_box_list(vec![
                "CB 1".to_string(),
                "CB 2".to_string(),
                "CB 3".to_string(),
                "CB 4".to_string(),
                "CB 5".to_string(),
                "CB 6".to_string(),
                "CB 7".to_string(),
                "CB 8".to_string(),
                "CB 9".to_string(),
                "CB 10".to_string(),
            ])
            .selection_list_count(10)
            .combo_box_list_count(10)
            .child(
                Grid::new()
                    .margin(8)
                    .columns(
                        Columns::create()
                            .push(132)
                            .push(16)
                            .push(132)
                            .push(16)
                            .push(132),
                    )
                    .rows(Rows::create().push("*").push(32))
                    .child(
                        Stack::new()
                            .attach(Grid::column(0))
                            .attach(Grid::row(0))
                            // Column 0
                            .child(create_header(ctx, "Buttons"))
                            .child(
                                Button::new()
                                    .text("Button")
                                    .margin((0, 8, 0, 0))
                                    .icon(material_icons_font::MD_CHECK)
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(1))
                                    .on_click(move |states, _| {
                                        state(id, states).action(Action::IncrementCounter);
                                        true
                                    })
                                    .build(ctx),
                            )
                            .child(
                                Button::new()
                                    .text("Primary")
                                    .style("button_primary")
                                    .margin((0, 8, 0, 0))
                                    .icon(material_icons_font::MD_360)
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(2))
                                    .build(ctx),
                            )
                            .child(
                                ToggleButton::new()
                                    .style("button_single_content")
                                    .text("ToggleButton")
                                    .margin((0, 8, 2, 0))
                                    .icon(material_icons_font::MD_ALARM_ON)
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(3))
                                    .build(ctx),
                            )
                            .child(
                                CheckBox::new()
                                    .text("CheckBox")
                                    .margin((0, 8, 0, 0))
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(4))
                                    .build(ctx),
                            )
                            .child(
                                Switch::new()
                                    .margin((0, 8, 0, 0))
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(5))
                                    .build(ctx),
                            )
                            .child(slider)
                            .child(
                                ProgressBar::new()
                                    .val(slider)
                                    .margin((0, 8, 0, 0))
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(
                        Stack::new()
                            .attach(Grid::column(2))
                            .attach(Grid::row(0))
                            .child(create_header(ctx, "Text"))
                            .child(
                                TextBlock::new()
                                    .style("body")
                                    .text(("result", id))
                                    .margin((0, 8, 0, 0))
                                    .attach(Grid::column(2))
                                    .attach(Grid::row(1))
                                    .build(ctx),
                            )
                            .child(
                                TextBox::new()
                                    .water_mark("TextBox...")
                                    .text(("text_one", id))
                                    .margin((0, 8, 0, 0))
                                    .attach(Grid::column(2))
                                    .attach(Grid::row(2))
                                    .on_activate(move |states, entity| {
                                        state(id, states).action(Action::EntryActivated(entity));
                                    })
                                    .build(ctx),
                            )
                            .child(
                                TextBox::new()
                                    .water_mark("TextBox...")
                                    .text(("text_two", id))
                                    .margin((0, 8, 0, 0))
                                    .attach(Grid::column(2))
                                    .attach(Grid::row(2))
                                    .on_activate(move |states, entity| {
                                        state(id, states).action(Action::EntryActivated(entity));
                                    })
                                    .build(ctx),
                            )
                            .child(
                                Button::new()
                                    .margin((0, 8, 0, 0))
                                    .style("button_single_content")
                                    .text("clear text")
                                    .icon(material_icons_font::MD_CLEAR)
                                    .on_click(move |states, _| {
                                        state(id, states).action(Action::ClearText);
                                        true
                                    })
                                    .build(ctx),
                            )
                            .child(
                                NumericBox::new()
                                    .margin((0, 8, 0, 0))
                                    .max(123)
                                    .step(0.123)
                                    .val(0.123)
                                    .build(ctx),
                            )
                            .child(
                                PasswordBox::new()
                                    .margin((0, 8, 0, 0))
                                    .water_mark("Password")
                                    .v_align("center")
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(
                        Grid::new()
                            .rows(
                                Rows::create()
                                    .push("auto")
                                    .push(32)
                                    .push(16)
                                    .push(204)
                                    .push("auto")
                                    .push(192)
                                    .push("auto"),
                            )
                            .columns(Columns::create().push("*").push(4).push("*"))
                            .attach(Grid::column(4))
                            .attach(Grid::row(0))
                            .child(
                                TextBlock::new()
                                    .text("Items")
                                    .style("header")
                                    .attach(Grid::column(0))
                                    .attach(Grid::column_span(3))
                                    .attach(Grid::row(0))
                                    .build(ctx),
                            )
                            .child(
                                ComboBox::new()
                                    .items_builder(move |bc, index| {
                                        let text = bc
                                            .get_widget(id)
                                            .get::<Vec<String>>("combo_box_list")[index]
                                            .clone();
                                        TextBlock::new()
                                            .style("small_text")
                                            .margin((0, 0, 0, 2))
                                            .v_align("center")
                                            .text(text)
                                            .build(bc)
                                    })
                                    .selected_index(0)
                                    .attach(Grid::column(0))
                                    .attach(Grid::column_span(3))
                                    .attach(Grid::row(1))
                                    .margin((0, 8, 0, 0))
                                    .count(("combo_box_list_count", id))
                                    .build(ctx),
                            )
                            .child(
                                ItemsWidget::new()
                                    .id("items")
                                    .padding((4, 4, 4, 2))
                                    .attach(Grid::column(0))
                                    .attach(Grid::column_span(3))
                                    .attach(Grid::row(3))
                                    .margin((0, 0, 0, 8))
                                    // bc = build-context
                                    .items_builder(move |bc, index| {
                                        let text = bc.get_widget(id).get::<Vec<String>>("list")
                                            [index]
                                            .clone();

                                        Button::new().margin((0, 0, 0, 2)).text(text).build(bc)
                                    })
                                    .count(("list_count", id))
                                    .build(ctx),
                            )
                            .child(
                                Button::new()
                                    .style("button_single_content")
                                    .id("remove-item-button")
                                    .icon(material_icons_font::MD_REMOVE_CIRCLE)
                                    .on_click(move |states, _| {
                                        state(id, states).action(Action::RemoveItem);
                                        true
                                    })
                                    .min_width(0)
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(4))
                                    .build(ctx),
                            )
                            .child(
                                Button::new()
                                    .style("button_single_content")
                                    .id("add-item-button")
                                    .icon(material_icons_font::MD_ADD_CIRCLE)
                                    .on_click(move |states, _| {
                                        state(id, states).action(Action::AddItem);
                                        true
                                    })
                                    .min_width(0)
                                    .attach(Grid::column(2))
                                    .attach(Grid::row(4))
                                    .build(ctx),
                            )
                            .child(
                                ListView::new()
                                    .attach(Grid::column(0))
                                    .attach(Grid::column_span(3))
                                    .attach(Grid::row(5))
                                    .selected_indices(id)
                                    .margin((0, 16, 0, 8))
                                    .items_builder(move |bc, index| {
                                        let text = bc
                                            .get_widget(id)
                                            .get::<Vec<String>>("selection_list")[index]
                                            .clone();
                                        TextBlock::new()
                                            .margin((0, 0, 0, 2))
                                            .v_align("center")
                                            .text(text)
                                            .build(bc)
                                    })
                                    .on_selection_changed(move |states, _, _| {
                                        states
                                            .get_mut::<MainViewState>(id)
                                            .action(Action::SelectionChanged)
                                    })
                                    .count(("selection_list_count", id))
                                    .build(ctx),
                            )
                            .child(
                                // todo: wrong text width????
                                TextBlock::new()
                                    .style("body")
                                    .id("selection")
                                    .max_width(120)
                                    .attach(Grid::column(0))
                                    .attach(Grid::column_span(3))
                                    .attach(Grid::row(6))
                                    .text("Selected:")
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(
                        Stack::new()
                            .orientation("horizontal")
                            .attach(Grid::row(1))
                            .attach(Grid::column(0))
                            .child(
                                TextBlock::new()
                                    .style("body")
                                    .text("Toggle theme: ")
                                    .v_align("center")
                                    .margin((0, 0, 4, 0))
                                    .build(ctx),
                            )
                            .child(
                                Switch::new()
                                    .on_changed("selected", move |states, entity| {
                                        state(id, states).action(Action::ToggleTheme(entity));
                                    })
                                    .v_align("center")
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
            Window::new()
                .title("OrbTk - widgets example")
                .position((100, 100))
                .size(468, 730)
                .resizeable(true)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}

// helper to request MainViewState
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut MainViewState {
    states.get_mut(id)
}
