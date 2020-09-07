use orbtk::prelude::*;

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

widget!(MainView {});

impl Template for MainView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            TabWidget::new()
                .tab("Buttons", ButtonView::new().build(ctx))
                .tab("Text", TextView::new().build(ctx))
                .tab("Items", ItemsView::new().build(ctx))
                .tab("Layouts", LayoutView::new().build(ctx))
                .tab("Image", ImageView::new().build(ctx))
                .build(ctx),
        )
    }
}

widget!(ButtonView {});

impl Template for ButtonView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            Stack::new()
                .width(140)
                .margin(16)
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
                .build(ctx),
        )
    }
}

widget!(TextView {});

impl Template for TextView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            Stack::new()
                .width(140)
                .margin(16)
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
                            TextBlock::new().margin((0, 0, 0, 2)).text(text).build(bc)
                        })
                        .build(ctx),
                )
                .child(TextBlock::new().text("ListView").style("header").build(ctx))
                .child(
                    ListView::new()
                        .count(count)
                        .items_builder(move |bc, index| {
                            let text = bc.get_widget(id).get::<Vec<String>>("items")[index].clone();
                            TextBlock::new().margin((0, 0, 0, 2)).text(text).build(bc)
                        })
                        .build(ctx),
                )
                .child(TextBlock::new().text("ComboBox").style("header").build(ctx))
                .child(
                    ComboBox::new()
                        .count(count)
                        .items_builder(move |bc, index| {
                            let text = bc.get_widget(id).get::<Vec<String>>("items")[index].clone();
                            TextBlock::new().margin((0, 0, 0, 2)).text(text).build(bc)
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
                .image("res/orbtk_logo.png")
                .build(ctx),
        )
    }
}
