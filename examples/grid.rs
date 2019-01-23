use orbtk::*;

struct MainView;

// Positing with horizontal / vertical alignment

//impl Widget for MainView {
//    fn create() -> Template {
//        Template::default()
//            .as_parent_type(ParentType::Single)
//            .with_debug_name("MainView")
//            .with_child(
//                Grid::create()
//                    .with_child(Grid::create().with_property(Selector::from("test")))
//                    .with_child(
//                        Grid::create()
//                            .with_property(Selector::from("testa"))
//                            .with_property(HorizontalAlignment::Center)
//                            .with_property(VerticalAlignment::Center)
//                    ),
//            )
//    }
//}

// todo: use ParentType::Single as default

// todo: column definitions are not functional now (wip)
impl Widget for MainView {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_debug_name("MainView")
            .with_child(
                Grid::create()
                    .with_property(
                        Columns::create()
                            .with(Column::create().with_width(ColumnWidth::Stretch).build())
                            .with(Column::create().with_width(ColumnWidth::Auto).build())
                            .with(
                                Column::create()
                                    .with_width(ColumnWidth::Width(50.0))
                                    .build(),
                            )
                            .build(),
                    )
                    .with_child(
                        Grid::create()
                            .with_property(Selector::from("test"))
                            .with_property(GridColumn(0)),
                    )
                    .with_child(
                        Grid::create()
                            .with_property(Selector::from("testa"))
                            .with_property(GridColumn(1)),
                    )
                    .with_child(
                        Grid::create()
                            .with_property(Selector::from("testb"))
                            .with_property(GridColumn(2)),
                    ),
            )
    }
}

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .with_bounds(Bounds::new(100.0, 100.0, 420.0, 730.0))
        .with_title("OrbTk - grid example")
        .with_root(MainView::create())
        .with_theme(
            Theme::create()
                .with_extenstion_path("examples/res/grid.css")
                .build(),
        )
        .with_debug_flag(true)
        .build();
    application.run();
}
