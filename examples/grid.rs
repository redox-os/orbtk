use orbtk::*;

struct MainView;

impl Widget for MainView {
    type Template = Template;

    fn create() -> Self::Template {
        Template::new()
            .child(
                Grid::create()
                    .columns(
                        Columns::create()
                            .column("*")
                            .column("Auto")
                            .column(50.0)
                            .build(),
                    )
                    .rows(Rows::create().row("*").row("*").build())
                    .child(
                        Grid::create()
                            .selector("lynch")
                            .margin((10.0, 0.0, 0.0, 4.0))
                            .attach_property(GridColumn(0))
                            .child(
                                TextBlock::create()
                                    .text("(0,0)")
                                    .horizontal_alignment("Center")
                                    .vertical_alignment("Center"),
                            ),
                    )
                    .child(
                        Grid::create()
                            .selector("bluebayoux")
                            .margin(10.0)
                            .constraint(Constraint::create().width(150.0).build())
                            .attach_property(GridColumn(1))
                            .child(
                                TextBlock::create()
                                    .text("(1,0)")
                                    .horizontal_alignment("Center")
                                    .vertical_alignment("Center"),
                            ),
                    )
                    .child(
                        Grid::create()
                            .selector("linkwater")
                            .attach_property(GridColumn(2))
                            .child(
                                TextBlock::create()
                                    .text("(2,0)")
                                    .selector("linkwater")
                                    .horizontal_alignment("Center")
                                    .vertical_alignment("Center"),
                            ),
                    )
                    .child(
                        Grid::create()
                            .selector("goldendream")
                            .attach_property(GridColumn(0))
                            .attach_property(GridRow(1))
                            .attach_property(ColumnSpan(3))
                            .child(
                                TextBlock::create()
                                    .text("(0,1) - ColumnSpan 3")
                                    .selector("goldendream")
                                    .horizontal_alignment("Center")
                                    .vertical_alignment("Center"),
                            ),
                    ),
            )
            .debug_name("MainView")
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
                .extension_path("examples/res/grid.css")
                .build(),
        )
        .resizable(true)
        .build();
    application.run();
}
