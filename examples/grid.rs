use orbtk::*;

struct MainView;

// todo: check running by each system

// todo: use ParentType::Single as default

impl Widget for MainView {
    fn create() -> Template {
        Template::default()
            .debug_name("MainView")
            .child(
                Grid::create()
                    .property(Selector::from("green"))
                    .property(
                        Columns::create()
                            .column(Column::create().width(ColumnWidth::Stretch).build())
                            .column(Column::create().width(ColumnWidth::Auto).build())
                            .column(Column::create().width(ColumnWidth::Width(50.0)).build())
                            .build(),
                    )
                    .property(
                        Rows::create()
                            .row(Row::create().height(RowHeight::Stretch).build())
                            .row(Row::create().height(RowHeight::Stretch).build())
                            .build(),
                    )
                    .child(
                        Grid::create()
                            .property(Selector::from("blue"))
                            .property(GridColumn(0)),
                    )
                    .child(
                        Grid::create()
                            .property(Selector::from("yellow"))
                            .property(GridColumn(1)),
                    )
                    .child(
                        Grid::create()
                            .property(Selector::from("red"))
                            .property(GridColumn(2)),
                    )
                    .child(
                        Grid::create()
                            .property(Selector::from("olive"))
                            .property(GridColumn(1))
                            .property(GridRow(1))
                            .property(ColumnSpan(2)),
                    ),
            )
    }
}

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .bounds(Bounds::new(100.0, 100.0, 420.0, 730.0))
        .title("OrbTk - grid example")
        .root(MainView::create())
        .theme(
            Theme::create()
                .extenstion_path("examples/res/grid.css")
                .build(),
        )
        .resizable(true)
        .debug_flag(true)
        .build();
    application.run();
}
