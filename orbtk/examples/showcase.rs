use orbtk::{prelude::*, widgets::themes::*};

// German localization file.
static SHOWCASE_DE_DE: &str = include_str!("assets/showcase/showcase_de_DE.ron");

fn main() {
    // if no dictionary is set for the default language e.g. english the content of the text property will drawn.
    let localization = RonLocalization::create()
        .language("en_US")
        .dictionary("de_DE", SHOWCASE_DE_DE)
        .build();

    Application::new()
        // .theme(theme_default_light())
        .localization(localization)
        .window(|ctx| {
            Window::new()
                .title("OrbTk - showcase example")
                .position((100, 100))
                .size(1000, 730)
                .resizeable(true)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}

// [START] views

// Represents the main wrapper view with main navigation.
widget!(MainView {});

impl Template for MainView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            TabWidget::new()
                .tab("Buttons / Text", ButtonView::new().build(ctx))
                .tab("Items", ItemsView::new().build(ctx))
                .tab("Layouts", LayoutView::new().build(ctx))
                .tab("Image", ImageView::new().build(ctx))
                .tab("Localization", LocalizationView::new().build(ctx))
                .tab("Navigation", NavigationView::new().build(ctx))
                .tab("Interactive", InteractiveView::new().build(ctx))
                .build(ctx),
        )
    }
}

// Represents an overview with button and text widgets.
widget!(ButtonView {});

impl Template for ButtonView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        let slider = Slider::new().min(0.0).max(1.0).build(ctx);
        self.child(
            Grid::new()
                .margin(16)
                .columns("140, 32, 140")
                .child(
                    Stack::new()
                        .spacing(8)
                        .child(
                            Button::new()
                                .text("Button")
                                .icon(material_icons_font::MD_CHECK)
                                .on_enter(|_, _| {
                                    println!("Enter Button boundries");
                                })
                                .on_leave(|_, _| {
                                    println!("Leave Button boundries");
                                })
                                .build(ctx),
                        )
                        .child(
                            Button::new()
                                .enabled(false)
                                .text("disabled")
                                .icon(material_icons_font::MD_CHECK)
                                .build(ctx),
                        )
                        .child(
                            Button::new()
                                .text("Primary")
                                .style("button_primary")
                                .icon(material_icons_font::MD_360)
                                .build(ctx),
                        )
                        .child(
                            Button::new()
                                .text("Text only")
                                .style("button_single_content")
                                .build(ctx),
                        )
                        .child(
                            Button::new()
                                .icon(material_icons_font::MD_CHECK)
                                .style("button_single_content")
                                .build(ctx),
                        )
                        .child(
                            ToggleButton::new()
                                .text("ToggleButton")
                                .icon(material_icons_font::MD_ALARM_ON)
                                .build(ctx),
                        )
                        .child(CheckBox::new().text("CheckBox").build(ctx))
                        .child(CheckBox::new().enabled(false).text("disabled").build(ctx))
                        .child(Switch::new().build(ctx))
                        .child(Switch::new().enabled(false).build(ctx))
                        .child(slider)
                        .child(ProgressBar::new().val(slider).build(ctx))
                        .build(ctx),
                )
                .child(
                    Stack::new()
                        .attach(Grid::column(2))
                        .spacing(8)
                        .child(TextBlock::new().text("Header").style("header").build(ctx))
                        .child(TextBlock::new().text("Text").style("body").build(ctx))
                        .child(TextBox::new().water_mark("Insert text...").build(ctx))
                        .child(
                            PasswordBox::new()
                                .water_mark("Insert password...")
                                .build(ctx),
                        )
                        .child(NumericBox::new().max(123).step(0.123).val(0.123).build(ctx))
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

type List = Vec<String>;

// Represents an overview of list widgets like ListView, ItemsWidget and ComboBox.
widget!(ItemsView { items: List });

impl Template for ItemsView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let items = vec![
            "Item 1".to_string(),
            "Item 2".to_string(),
            "Item 4".to_string(),
            "Item 5".to_string(),
        ];
        let count = items.len();
        self.items(items).child(
            Stack::new()
                .width(140)
                .margin(16)
                .spacing(8)
                .child(
                    TextBlock::new()
                        .text("ItemsWidget")
                        .style("header")
                        .build(ctx),
                )
                .child(
                    ItemsWidget::new()
                        .count(count)
                        .items_builder(move |bc, index| {
                            let text = bc.get_widget(id).get::<Vec<String>>("items")[index].clone();
                            TextBlock::new()
                                .style("body")
                                .v_align("center")
                                .text(text)
                                .build(bc)
                        })
                        .build(ctx),
                )
                .child(TextBlock::new().text("ListView").style("header").build(ctx))
                .child(
                    ListView::new()
                        .count(count)
                        .items_builder(move |bc, index| {
                            let text = bc.get_widget(id).get::<Vec<String>>("items")[index].clone();
                            TextBlock::new().v_align("center").text(text).build(bc)
                        })
                        .build(ctx),
                )
                .child(TextBlock::new().text("ComboBox").style("header").build(ctx))
                .child(
                    ComboBox::new()
                        .count(count)
                        .items_builder(move |bc, index| {
                            let text = ItemsView::items_ref(&bc.get_widget(id))[index].clone();
                            TextBlock::new().v_align("center").text(text).build(bc)
                        })
                        .selected_index(0)
                        .build(ctx),
                )
                .child(
                    ComboBox::new()
                        .enabled(false)
                        .count(count)
                        .items_builder(move |bc, index| {
                            let text = ItemsView::items_ref(&bc.get_widget(id))[index].clone();
                            TextBlock::new().v_align("center").text(text).build(bc)
                        })
                        .selected_index(0)
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

// Represents an overview of layout widgets.
widget!(LayoutView {});

impl Template for LayoutView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            Stack::new()
                .width(400)
                .margin(16)
                .spacing(8)
                .child(TextBlock::new().text("Grid").style("header").build(ctx))
                .child(
                    Container::new()
                        .width(300)
                        .height(150)
                        .background("black")
                        .child(
                            Grid::new()
                                .columns("*, auto, 50")
                                .rows("*, *")
                                .child(
                                    Container::new()
                                        .background("lynch")
                                        .margin((10, 0, 0, 4))
                                        .attach(Grid::column(0))
                                        .child(
                                            TextBlock::new()
                                                .text("(0,0)")
                                                .style("light_text")
                                                .h_align("center")
                                                .v_align("center")
                                                .build(ctx),
                                        )
                                        .build(ctx),
                                )
                                .child(
                                    Container::new()
                                        .background("bluebayoux")
                                        .margin(10)
                                        .constraint(Constraint::create().width(150))
                                        .attach(Grid::column(1))
                                        .child(
                                            TextBlock::new()
                                                .text("(1,0)")
                                                .style("body")
                                                .h_align("center")
                                                .v_align("center")
                                                .build(ctx),
                                        )
                                        .build(ctx),
                                )
                                .child(
                                    Container::new()
                                        .background("linkwater")
                                        .attach(Grid::column(2))
                                        .child(
                                            TextBlock::new()
                                                .text("(2,0)")
                                                .foreground("black")
                                                .h_align("center")
                                                .v_align("center")
                                                .build(ctx),
                                        )
                                        .build(ctx),
                                )
                                .child(
                                    Container::new()
                                        .background("goldendream")
                                        .attach(Grid::column(0))
                                        .attach(Grid::row(1))
                                        .attach(Grid::column_span(3))
                                        .child(
                                            TextBlock::new()
                                                .text("(0,1) - ColumnSpan 3")
                                                .foreground("black")
                                                .h_align("center")
                                                .v_align("center")
                                                .build(ctx),
                                        )
                                        .build(ctx),
                                )
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .child(TextBlock::new().text("Stack").style("header").build(ctx))
                .child(
                    Container::new()
                        .background("black")
                        .width(300)
                        .child(
                            Stack::new()
                                .spacing(4)
                                .child(Container::new().background("lynch").height(50).build(ctx))
                                .child(
                                    Container::new()
                                        .background("bluebayoux")
                                        .height(50)
                                        .build(ctx),
                                )
                                .child(
                                    Container::new()
                                        .background("linkwater")
                                        .height(50)
                                        .build(ctx),
                                )
                                .child(
                                    Container::new()
                                        .background("goldendream")
                                        .height(50)
                                        .build(ctx),
                                )
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .child(TextBlock::new().text("Padding").style("header").build(ctx))
                .child(
                    Container::new()
                        .background("black")
                        .width(300)
                        .height(150)
                        .padding((16, 8, 16, 8))
                        .child(Container::new().background("lynch").build(ctx))
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

// Represents an overview of the image widget.
widget!(ImageView {});

impl Template for ImageView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            ImageWidget::new()
                .margin(16)
                .image("orbtk/examples/assets/showcase/orbtk_logo.png")
                .build(ctx),
        )
    }
}

// Contains an example how to use localization in OrbTk.
widget!(LocalizationView<LocalizationState> { languages: List, selected_index: i32 });

impl Template for LocalizationView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let languages = vec!["English".to_string(), "German".to_string()];
        let count = languages.len();

        self.languages(languages).selected_index(0).child(
            Stack::new()
                .width(120)
                .margin(16)
                .spacing(8)
                .child(
                    TextBlock::new()
                        .style("small_text")
                        .text("Hello")
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("small_text")
                        .text("world")
                        .build(ctx),
                )
                .child(TextBlock::new().style("small_text").text("I").build(ctx))
                .child(TextBlock::new().style("small_text").text("love").build(ctx))
                .child(
                    TextBlock::new()
                        .style("small_text")
                        .text("localization")
                        .build(ctx),
                )
                .child(TextBlock::new().style("small_text").text("!").build(ctx))
                .child(
                    ComboBox::new()
                        .count(count)
                        .items_builder(move |bc, index| {
                            let text =
                                LocalizationView::languages_ref(&bc.get_widget(id))[index].clone();
                            TextBlock::new().v_align("center").text(text).build(bc)
                        })
                        .on_changed("selected_index", move |states, _| {
                            states.get_mut::<LocalizationState>(id).change_language();
                        })
                        .selected_index(id)
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

// Represents an overview of navigation widgets.
widget!(
    NavigationView<NavigationState> {
        md_navigation_visibility: Visibility
    }
);

impl Template for NavigationView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let pager = Pager::new()
            .attach(Grid::row(1))
            .child(
                Container::new()
                    .padding(8)
                    .background("lynch")
                    .child(TextBlock::new().text("Page 1").build(ctx))
                    .build(ctx),
            )
            .child(
                Container::new()
                    .padding(8)
                    .background("goldendream")
                    .child(
                        TextBlock::new()
                            .foreground("black")
                            .text("Page 2")
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .child(
                Container::new()
                    .padding(8)
                    .background("linkwater")
                    .child(
                        TextBlock::new()
                            .foreground("black")
                            .text("Page 3")
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .build(ctx);

        self.child(
            Grid::new()
                .margin(16)
                .rows("32, *, 8, auto, 8, 32, *")
                .child(TextBlock::new().text("Pager").style("header").build(ctx))
                .child(pager)
                .child(
                    Button::new()
                        .style("button_single_content")
                        .icon(material_icons_font::MD_KEYBOARD_ARROW_LEFT)
                        .h_align("start")
                        .attach(Grid::row(3))
                        .on_click(move |states, _| {
                            states.send_message(PagerAction::Previous, pager);
                            true
                        })
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .style("button_single_content")
                        .icon(material_icons_font::MD_KEYBOARD_ARROW_RIGHT)
                        .h_align("end")
                        .attach(Grid::row(3))
                        .on_click(move |states, _| {
                            states.send_message(PagerAction::Next, pager);
                            true
                        })
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .text("MasterDetail")
                        .attach(Grid::row(5))
                        .style("header")
                        .build(ctx),
                )
                .child(
                    MasterDetail::new()
                        .id(ID_NAVIGATION_MASTER_DETAIL)
                        .responsive(true)
                        .break_point(1000)
                        .navigation_visibility(("md_navigation_visibility", id))
                        .attach(Grid::row(6))
                        .master_detail(
                            Container::new()
                                .padding(8)
                                .background("lynch")
                                .child(
                                    Stack::new()
                                        .orientation("vertical")
                                        .h_align("center")
                                        .v_align("center")
                                        .child(TextBlock::new().text("Content inside the master pane")
                                               .font_size(16)
                                               .build(ctx))
                                        .child(TextBlock::new().text("Resize the window: Pane brake is set to 800 pixel")
                                               .font_size(14)
                                               .build(ctx))
                                        .build(ctx))
                                .child(TextBlock::new().text("Master Pane").v_align("end").build(ctx))
                                .child(
                                    Button::new()
                                        .style("button_primary_single_content")
                                        .visibility(("md_navigation_visibility", id))
                                        .h_align("start")
                                        .text("show detail pane")
                                        .on_click(move |ctx, _| {
                                            ctx.send_message(MasterDetailAction::ShowDetail, id);
                                            true
                                        })
                                        .build(ctx),
                                )
                                .build(ctx),
                            Container::new()
                                .padding(8)
                                .background("goldendream")
                                .child(TextBlock::new().text("Content inside the detail pane")
                                       .h_align("center")
                                       .v_align("center")
                                       .foreground("black")
                                       .font_size(14)
                                       .build(ctx))
                                .child(
                                    TextBlock::new()
                                        .text("Detail Pane")
                                        .v_align("end")
                                        .foreground("black")
                                        .margin(8)
                                        .build(ctx),
                                )
                                .child(
                                    Button::new()
                                        .text("back")
                                        .style("button_single_content")
                                        .visibility(("md_navigation_visibility", id))
                                        .h_align("start")
                                        .on_click(move |ctx, _| {
                                            ctx.send_message(MasterDetailAction::ShowMaster, id);
                                            true
                                        })
                                        .build(ctx),
                                )
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

// Contains examples how interaction works in OrbTk.
widget!(
    InteractiveView<InteractiveState> {
        settings_text: String,
        themes: List,
        selected_index: i32,
        count_text: String
    }
);

impl Template for InteractiveView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let themes = vec![
            "default_dark".to_string(),
            "default_light".to_string(),
            "redox".to_string(),
            "fluent_dark".to_string(),
            "fluent_light".to_string(),
        ];
        let themes_count = themes.len();

        self.count_text("0").themes(themes).child(
            Grid::new()
                .margin(8)
                .rows("auto, 4, 32, 8, auto, 3, 32, 8, auto, 4, 32, 4, auto, 4, 32")
                .columns("auto, 4, auto, 4, auto, *")
                // theme selection
                .child(
                    TextBlock::new()
                        .style("header")
                        .attach(Grid::row(0))
                        .attach(Grid::column(0))
                        .style("small_text")
                        .text("Select theme")
                        .build(ctx),
                )
                .child(
                    ComboBox::new()
                        .attach(Grid::row(2))
                        .attach(Grid::column(0))
                        .count(themes_count)
                        .items_builder(move |bc, index| {
                            let text =
                                InteractiveView::themes_ref(&bc.get_widget(id))[index].clone();
                            TextBlock::new().v_align("center").text(text).build(bc)
                        })
                        .on_changed("selected_index", move |ctx, _| {
                            ctx.send_message(InteractiveAction::ChangeTheme, id);
                        })
                        .selected_index(id)
                        .build(ctx),
                )
                // Settings
                .child(
                    TextBlock::new()
                        .h_align("start")
                        .attach(Grid::row(4))
                        .attach(Grid::column(0))
                        .attach(Grid::column_span(5))
                        .text("Settings")
                        .style("header")
                        .build(ctx),
                )
                .child(
                    TextBox::new()
                        .text(("settings_text", id))
                        .attach(Grid::row(6))
                        .attach(Grid::column(0))
                        .water_mark("Insert text...")
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .text("load")
                        .style("button_single_content")
                        .attach(Grid::row(6))
                        .attach(Grid::column(2))
                        .on_click(move |ctx, _| {
                            ctx.send_message(InteractiveAction::LoadSettings, id);
                            true
                        })
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .text("save")
                        .style("button_single_content")
                        .attach(Grid::row(6))
                        .attach(Grid::column(4))
                        .on_click(move |ctx, _| {
                            ctx.send_message(InteractiveAction::SaveSettings, id);
                            true
                        })
                        .build(ctx),
                )
                // Counter
                .child(
                    TextBlock::new()
                        .h_align("start")
                        .attach(Grid::row(8))
                        .attach(Grid::column(0))
                        .attach(Grid::column_span(5))
                        .text("Counter")
                        .style("header")
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .style("button_single_content")
                        .attach(Grid::row(10))
                        .attach(Grid::column(0))
                        .icon(material_icons_font::MD_PLUS)
                        .on_click(move |ctx, _| {
                            ctx.send_message(InteractiveAction::Increment, id);
                            true
                        })
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("body")
                        .h_align("center")
                        .v_align("center")
                        .attach(Grid::row(12))
                        .attach(Grid::column(0))
                        .text(("count_text", id))
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .style("button_single_content")
                        .attach(Grid::row(14))
                        .attach(Grid::column(0))
                        .icon(material_icons_font::MD_MINUS)
                        .on_click(move |ctx, _| {
                            ctx.send_message(InteractiveAction::Decrement, id);
                            true
                        })
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

// [END] views

// [START] states

static ID_NAVIGATION_MASTER_DETAIL: &str = "id_navigation_master_detail";

#[derive(Debug, Default, AsAny)]
struct NavigationState {
    master_detail: Entity,
}

impl State for NavigationState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        self.master_detail = ctx.child(ID_NAVIGATION_MASTER_DETAIL).entity();
    }

    fn messages(
        &mut self,
        mut messages: MessageReader,
        _registry: &mut Registry,
        ctx: &mut Context,
    ) {
        for message in messages.read::<MasterDetailAction>() {
            ctx.send_message(message, self.master_detail);
        }
    }
}

#[derive(Debug, Default, AsAny)]
struct LocalizationState {
    change_language: bool,
}

impl LocalizationState {
    fn change_language(&mut self) {
        self.change_language = true;
    }
}

impl State for LocalizationState {
    fn update(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        if !self.change_language {
            return;
        }

        let index = *LocalizationView::selected_index_ref(&ctx.widget()) as usize;
        let selected_language = LocalizationView::languages_ref(&ctx.widget())[index].clone();

        match selected_language.as_str() {
            "English" => ctx.set_language("en_US"),
            "German" => ctx.set_language("de_DE"),
            _ => (),
        }

        self.change_language = false;
    }
}

#[derive(Debug, Default, AsAny)]
struct InteractiveState {
    count: i32,
}

impl State for InteractiveState {
    fn messages(
        &mut self,
        mut messages: MessageReader,
        registry: &mut Registry,
        ctx: &mut Context,
    ) {
        for message in messages.read::<InteractiveAction>() {
            match message {
                InteractiveAction::LoadSettings => registry
                    .get::<Settings>("settings")
                    .load_async::<SettingsData>("settings_data".to_string(), ctx.entity()),
                InteractiveAction::SaveSettings => {
                    let text: String = InteractiveView::settings_text_clone(&ctx.widget());
                    registry.get::<Settings>("settings").save_async(
                        "settings_data".to_string(),
                        SettingsData(text),
                        ctx.entity(),
                    );
                }
                InteractiveAction::ChangeTheme => {
                    let theme_index = *InteractiveView::selected_index_ref(&ctx.widget());

                    match theme_index {
                        0 => ctx.switch_theme(Rc::new(theme_default_dark())),
                        1 => ctx.switch_theme(Rc::new(theme_default_light())),
                        2 => ctx.switch_theme(Rc::new(theme_redox())),
                        3 => ctx.switch_theme(Rc::new(theme_fluent_dark())),
                        4 => ctx.switch_theme(Rc::new(theme_fluent_light())),
                        _ => {}
                    }
                }
                InteractiveAction::Increment => {
                    self.count += 1;
                    InteractiveView::count_text_set(&mut ctx.widget(), self.count.to_string());
                }
                InteractiveAction::Decrement => {
                    self.count -= 1;
                    InteractiveView::count_text_set(&mut ctx.widget(), self.count.to_string());
                }
            }
        }

        // save result
        for message in messages.read::<SettingsResult<()>>() {
            println!("Result {:?}", message);
        }

        // load result
        for message in messages.read::<SettingsResult<SettingsData>>() {
            if let Ok(data) = message {
                InteractiveView::settings_text_set(&mut ctx.widget(), data.0);
            }
        }
    }
}

// [END] states

// [START] Dummy data

#[derive(Clone, Debug)]
enum InteractiveAction {
    SaveSettings,
    LoadSettings,
    ChangeTheme,
    Increment,
    Decrement,
}

use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SettingsData(pub String);

// [END] Dummy data
