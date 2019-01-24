use orbtk::*;

struct MainView;

// todo: check running by each system

// todo: use ParentType::Single as default

impl Widget for MainView {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_debug_name("MainView")
            .with_child(
                Grid::create()
                    .with_property(Selector::from("green"))
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
                            .with_property(Selector::from("blue"))
                            .with_property(GridColumn(0))
                            .with_property(ColumnSpan(3)),
                    )
                    .with_child(
                        Grid::create()
                            .with_property(Selector::from("yellow"))
                            .with_property(GridColumn(1)),
                    )
                    .with_child(
                        Grid::create()
                            .with_property(Selector::from("red"))
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
        .with_resizable(true)
        .with_debug_flag(true)
        .build();
    application.run();
}
