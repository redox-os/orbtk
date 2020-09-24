use orbtk::prelude::*;

// German localization file.
static SHOWCASE_DE_DE: &str = include_str!("../res/showcase/showcase_de_DE.ron");

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    // if no dictionary is set for the default language e.g. english the content of the text property will drawn.
    let localization = RonLocalization::create()
        .language("en_US")
        .dictionary("de_DE", SHOWCASE_DE_DE)
        .build();

    Application::new()
        .localization(localization)
        .window(|ctx| {
            Window::new()
                .title("OrbTk - showcase example")
                .position((100, 100))
                .size(600, 730)
                .resizeable(true)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}

// [START] views

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
                .build(ctx),
        )
    }
}

widget!(ButtonView {});

impl Template for ButtonView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        let slider = Slider::new().min(0.0).max(1.0).build(ctx);
        self.child(
            Grid::new()
                .margin(16)
                .columns(Columns::create().push(140).push(32).push(140))
                .child(
                    Stack::new()
                        .spacing(8)
                        .child(
                            Button::new()
                                .text("Button")
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
                        .child(Switch::new().build(ctx))
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
                            TextBlock::new().v_align("center").text(text).build(bc)
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
                            let text = bc.get_widget(id).get::<Vec<String>>("items")[index].clone();
                            TextBlock::new().v_align("center").text(text).build(bc)
                        })
                        .selected_index(0)
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

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
                                .columns(Columns::create().push("*").push("auto").push(50))
                                .rows(Rows::create().push("*").push("*"))
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

widget!(ImageView {});

impl Template for ImageView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            ImageWidget::new()
                .margin(16)
                .image("res/showcase/orbtk_logo.png")
                .build(ctx),
        )
    }
}

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
                .child(TextBlock::new().text("Hello").build(ctx))
                .child(TextBlock::new().text("world").build(ctx))
                .child(TextBlock::new().text("I").build(ctx))
                .child(TextBlock::new().text("love").build(ctx))
                .child(TextBlock::new().text("localization").build(ctx))
                .child(TextBlock::new().text("!").build(ctx))
                .child(
                    ComboBox::new()
                        .count(count)
                        .items_builder(move |bc, index| {
                            let text =
                                bc.get_widget(id).get::<Vec<String>>("languages")[index].clone();
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

widget!(NavigationView {});

impl Template for NavigationView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        let pager = Pager::new()
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
                .rows(
                    Rows::create()
                        .push("*")
                        .push(8)
                        .push("auto")
                        .push(8)
                        .push("*")
                        .build(),
                )
                .child(pager)
                .child(
                    Button::new()
                        .style("button_single_content")
                        .icon(material_icons_font::MD_KEYBOARD_ARROW_LEFT)
                        .h_align("start")
                        .attach(Grid::row(2))
                        .on_click(move |states, _| {
                            states.get_mut::<PagerState>(pager).previous();
                            true
                        })
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .style("button_single_content")
                        .icon(material_icons_font::MD_KEYBOARD_ARROW_RIGHT)
                        .h_align("end")
                        .attach(Grid::row(2))
                        .on_click(move |states, _| {
                            states.get_mut::<PagerState>(pager).next();
                            true
                        })
                        .build(ctx),
                )
                .child(
                    MasterDetail::new()
                        .responsive(true)
                        .break_point(620)
                        .attach(Grid::row(4))
                        .master_detail(
                            Container::new()
                                .background("lynch")
                                .child(TextBlock::new().margin(8).text("master").build(ctx))
                                .build(ctx),
                            Container::new()
                                .padding(8)
                                .background("goldendream")
                                .child(
                                    TextBlock::new()
                                        .foreground("black")
                                        .margin(8)
                                        .text("detail")
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

// [END] views

// [START] states

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
            _ => {}
        }

        self.change_language = false;
    }
}

// [END] states
