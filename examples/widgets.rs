use orbtk::*;

// #[derive(Default)]
// struct MainViewState {
//     counter: Cell<i32>,
// }

// impl MainViewState {
//     pub fn increment(&self) {
//         self.counter.set(self.counter.get() + 1)
//     }
// }

// impl State for MainViewState {
//     fn update(&self, context: &mut Context<'_>) {
//         if let Ok(button_count_text) = context.widget().borrow_mut_property::<Text>() {
//             button_count_text.0 = format!("Button count: {}", self.counter.get());
//         }
//     }
// }

// fn create_header(text: &str, grid: usize, column: usize) -> Template {
//     TextBlock::create()
//         .text(text)
//         .selector(Selector::from("textblock").class("h1"))
//         .attach(GridColumn(grid))
//         .attach(GridRow(column))
//         .into()
// }

// widget!(MainView);

// impl Widget for MainView {
//     fn create() -> Self {
//         let state = Rc::new(MainViewState::default());
//         let button_count_text = Property::new(Text::from("Button count: 0"));

//         MainView::new()
//             .state(state.clone())
//             .child(
//                 Grid::create()
//                     .margin(8.0)
//                     .columns(
//                         Columns::create()
//                             .column("Auto")
//                             .column(32.0)
//                             .column("Auto")
//                             .column("*")
//                             .build(),
//                     )
//                     .rows(
//                         Rows::create()
//                             .row("Auto")
//                             .row("Auto")
//                             .row("Auto")
//                             .row("Auto")
//                             .row("Auto")
//                             .row("Auto")
//                             .build(),
//                     )
//                     // Column 0
//                     .child(create_header("Button", 0, 0))
//                     .child(
//                         Button::create()
//                             .text("Button")
//                             .margin((0.0, 8.0, 0.0, 0.0))
//                             .font_icon(material_font_icons::CHECK_FONT_ICON)
//                             .attach(GridColumn(0))
//                             .attach(GridRow(1))
//                             .on_click(move |_| {
//                                 state.increment();
//                                 true
//                             }),
//                     )
//                     .child(
//                         Button::create()
//                             .text("Primary")
//                             .selector(Selector::from("button").class("primary"))
//                             .margin((0.0, 8.0, 0.0, 0.0))
//                             .font_icon(material_font_icons::CHECK_FONT_ICON)
//                             .attach(GridColumn(0))
//                             .attach(GridRow(2)),
//                     )
//                     .child(
//                         ToggleButton::create()
//                             .text("ToggleButton")
//                             .margin((0.0, 8.0, 0.0, 0.0))
//                             .attach(GridColumn(0))
//                             .attach(GridRow(3)),
//                     )
//                     .child(
//                         CheckBox::create()
//                             .text("CheckBox")
//                             .margin((0.0, 8.0, 0.0, 0.0))
//                             .attach(GridColumn(0))
//                             .attach(GridRow(4)),
//                     )
//                     .child(
//                         Switch::create()
//                             .margin((0.0, 8.0, 0.0, 0.0))
//                             .attach(GridColumn(0))
//                             .attach(GridRow(5)),
//                     )
//                     // Column 2
//                     .child(create_header("Text", 2, 0))
//                     .child(
//                         TextBlock::create()
//                             .selector(Selector::new().class("body"))
//                             .text_prop(button_count_text.share())
//                             .margin((0.0, 8.0, 0.0, 0.0))
//                             .attach(GridColumn(2))
//                             .attach(GridRow(1)),
//                     )
//                     .child(
//                         TextBox::create()
//                             .water_mark("TextBox...")
//                             .margin((0.0, 8.0, 0.0, 0.0))
//                             .attach(GridColumn(2))
//                             .attach(GridRow(2)),
//                     ),
//             )
//             .attach(button_count_text)
//             .debug_name("MainView")
//     }
// }

widget!(MainView);

impl Template for MainView {
    fn template(self, _: Entity, context: &mut BuildContext) -> Self {
        self.name("MainView").child(
            Container::create()
                .background("#6195ED")
                .vertical_alignment("Start")
                .child(
                    TextBlock::create()
                        .text("TextBlock")
                        .font_size(38.0)
                        .build(context),
                )
                .build(context),
        )
    }
}

fn main() {
    let mut application = Application::default();

    application
        .create_window()
        .bounds((100.0, 100.0, 420.0, 730.0))
        .title("OrbTk - widgets example")
        .debug_flag(true)
        .build(MainView::create());
    application.run();
}
