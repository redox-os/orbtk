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
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                Action::AddItem => {
                    let len = main_view(ctx.widget()).list().len();

                    if len < 5 {
                        main_view(ctx.widget())
                            .list_mut()
                            .push(format!("Item {}", len + 1));
                        ctx.child("items").clone_or_default::<usize>("Item");
                        items_widget(ctx.child("items")).set_count(len + 1);
                        button(ctx.child("remove-item-button")).set_enabled(true);
                        button(ctx.child("remove-item-button")).set_visibility(Visibility::Visible);

                        if len == 4 {
                            button(ctx.child("add-item-button")).set_enabled(false);
                            button(ctx.child("add-item-button")).set_visibility(Visibility::Collapsed);
                        }
                    }
                }
                Action::RemoveItem => {
                    let len = main_view(ctx.widget()).list().len();
                    if len > 0 {
                        main_view(ctx.widget()).list_mut().remove(len - 1);
                        items_widget(ctx.child("items")).set_count(len - 1);
                        button(ctx.child("add-item-button")).set_enabled(true);
                        button(ctx.child("add-item-button")).set_visibility(Visibility::Visible);

                        if len == 1 {
                            button(ctx.child("remove-item-button")).set_enabled(false);
                            button(ctx.child("remove-item-button")).set_visibility(Visibility::Collapsed);
                        }
                    }
                }
                Action::IncrementCounter => {
                    *main_view(ctx.widget()).counter_mut() += 1;

                    let counter = *main_view(ctx.widget()).counter();

                    main_view(ctx.widget())
                        .set_result(String16::from(format!("Button count: {}", counter)));
                }
                Action::ClearText => {
                    main_view(ctx.widget()).set_text_one(String16::default());
                    main_view(ctx.widget()).set_text_two(String16::default());
                }
                Action::EntryActivated(entity) => {
                    let mut text_box = text_box(ctx.get_widget(entity));
                    let text = text_box.text_mut();
                    println!("submitting {}", text);
                    text.clear();
                }
                Action::EntryChanged(entity) => {
                    println!("entry changed: {}", text_box(ctx.get_widget(entity)).text());
                }
                Action::ValueChanged(entity) => {
                    let val = ((slider(ctx.get_widget(entity)).val()).floor() as i32).to_string();

                    text_block(ctx.child("value_text")).set_text(String16::from(val));
                }
            }

            self.action = None;
        }
    }

    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context) {
        let mut selection_string = "Selected:".to_string();

        for index in &main_view(ctx.widget()).selected_indices().0 {
            selection_string = format!("{} {}", selection_string, index);
        }

        text_block(ctx.child("selection")).set_text(selection_string);
    }
}

fn create_header(ctx: &mut BuildContext, text: &str) -> Entity {
    TextBlock::new()
        .text(text)
        .element("text-block")
        .class("h1")
        .build(ctx)
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
                    .margin(8.)
                    .columns(
                        Columns::new()
                            .add(132.)
                            .add(16.)
                            .add(132.)
                            .add(16.)
                            .add(132.),
                    )
                    .child(
                        Stack::new()
                            .attach(Grid::column(0))
                            // Column 0
                            .child(create_header(ctx, "Buttons"))
                            .child(
                                Button::new()
                                    .text("Button")
                                    .margin((0., 8., 0., 0.))
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
                                    .element("button")
                                    .class("primary")
                                    .margin((0., 8., 0., 0.))
                                    .icon(material_icons_font::MD_360)
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(2))
                                    .build(ctx),
                            )
                            .child(
                                ToggleButton::new()
                                    .class("single_content")
                                    .text("ToggleButton")
                                    .margin((0., 8., 2., 0.))
                                    .icon(material_icons_font::MD_ALARM_ON)
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(3))
                                    .build(ctx),
                            )
                            .child(
                                CheckBox::new()
                                    .text("CheckBox")
                                    .margin((0., 8., 0., 0.))
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(4))
                                    .build(ctx),
                            )
                            .child(
                                Switch::new()
                                    .margin((0., 8., 0., 0.))
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(5))
                                    .build(ctx),
                            )
                            .child(
                                TextBlock::new()
                                    .margin((0., 8., 0., 0.))
                                    .element("h1")
                                    .id("value_text")
                                    .text("0")
                                    .h_align("center")
                                    .build(ctx),
                            )
                            .child(
                                Slider::new()
                                    .on_changed(move |states, entity| {
                                        state(id, states).action(Action::ValueChanged(entity));
                                    })
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(
                        Stack::new()
                            .attach(Grid::column(2))
                            .child(create_header(ctx, "Text"))
                            .child(
                                TextBlock::new()
                                    .class("body")
                                    .text(("result", id))
                                    .margin((0., 8., 0., 0.))
                                    .attach(Grid::column(2))
                                    .attach(Grid::row(1))
                                    .build(ctx),
                            )
                            .child(
                                TextBox::new()
                                    .water_mark("TextBox...")
                                    .text(("text_one", id))
                                    .margin((0., 8., 0., 0.))
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
                                TextBox::new()
                                    .water_mark("TextBox...")
                                    .text(("text_two", id))
                                    .margin((0., 8., 0., 0.))
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
                                Button::new()
                                    .margin((0., 8., 0., 0.))
                                    .class("single_content")
                                    .margin((0., 8., 8., 0.))
                                    .icon(material_icons_font::MD_CLEAR)
                                    .text("clear text")
                                    .on_click(move |states, _| {
                                        state(id, states).action(Action::ClearText);
                                        true
                                    })
                                    .build(ctx),
                            )
                            .child(
                                NumericBox::new()
                                    .margin((-0., 8., 0., 0.))
                                    .max(123.)
                                    .step(0.123)
                                    .val(0.123)
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(
                        Grid::new()
                            .rows(
                                Rows::new()
                                    .add("auto")
                                    .add(32.)
                                    .add(16.)
                                    .add(204.)
                                    .add("auto")
                                    .add(192.)
                                    .add("auto"),
                            )
                            .columns(Columns::new().add("*").add(4.).add("*"))
                            .attach(Grid::column(4))
                            .child(
                                TextBlock::new()
                                    .text("Items")
                                    .element("text-block")
                                    .class("h1")
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
                                            .margin((0., 0., 0., 2.))
                                            .v_align("center")
                                            .text(text)
                                            .build(bc)
                                    })
                                    .selected_index(0)
                                    .attach(Grid::column(0))
                                    .attach(Grid::column_span(3))
                                    .attach(Grid::row(1))
                                    .margin((0., 8., 0., 0.))
                                    .count(("combo_box_list_count", id))
                                    .build(ctx),
                            )
                            .child(
                                ItemsWidget::new()
                                    .element("items-widget")
                                    .id("items")
                                    .padding((4., 4., 4., 2.))
                                    .attach(Grid::column(0))
                                    .attach(Grid::column_span(3))
                                    .attach(Grid::row(3))
                                    .margin((0., 0., 0., 8.))
                                    // bc = build-context
                                    .items_builder(move |bc, index| {
                                        let text = bc.get_widget(id).get::<Vec<String>>("list")
                                            [index]
                                            .clone();

                                        Button::new()
                                            .margin((0., 0., 0., 2.))
                                            .text(text)
                                            .build(bc)
                                    })
                                    .count(("list_count", id))
                                    .build(ctx),
                            )
                            .child(
                                Button::new()
                                    .element("button")
                                    .class("single_content")
                                    .id("remove-item-button")
                                    .icon(material_icons_font::MD_REMOVE_CIRCLE)
                                    .on_click(move |states, _| {
                                        state(id, states).action(Action::RemoveItem);
                                        true
                                    })
                                    .min_width(0.)
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(4))
                                    .build(ctx),
                            )
                            .child(
                                Button::new()
                                    .element("button")
                                    .class("single_content")
                                    .id("add-item-button")
                                    .icon(material_icons_font::MD_ADD_CIRCLE)
                                    .on_click(move |states, _| {
                                        state(id, states).action(Action::AddItem);
                                        true
                                    })
                                    .min_width(0.)
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
                                    .margin((0., 16., 0., 8.))
                                    .items_builder(move |bc, index| {
                                        let text = bc
                                            .get_widget(id)
                                            .get::<Vec<String>>("selection_list")[index]
                                            .clone();
                                        TextBlock::new()
                                            .margin((0., 0., 0., 2.))
                                            .v_align("center")
                                            .text(text)
                                            .build(bc)
                                    })
                                    .count(("selection_list_count", id))
                                    .build(ctx),
                            )
                            .child(
                                // todo: wrong text width????
                                TextBlock::new()
                                    .element("text-block")
                                    .id("selection")
                                    .max_width(120.)
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
            Window::new()
                .title("OrbTk - widgets example")
                .position((100., 100.))
                .size(468., 730.)
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
