use orbtk::prelude::*;

widget!(MainView);

impl Template for MainView {
    fn template(self, _: Entity, context: &mut BuildContext) -> Self {
        self.name("MainView").child(
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
                        .attach(GridColumn(0))
                        .child(
                            TextBlock::create()
                                .text("(0,0)")
                                .selector("light-text")
                                .horizontal_alignment("Center")
                                .vertical_alignment("Center")
                                .build(context),
                        )
                        .build(context),
                )
                .child(
                    Grid::create()
                        .selector("bluebayoux")
                        .margin(10.0)
                        .constraint(Constraint::create().width(150.0).build())
                        .attach(GridColumn(1))
                        .child(
                            TextBlock::create()
                                .text("(1,0)")
                                .selector("white")
                                .horizontal_alignment("Center")
                                .vertical_alignment("Center")
                                .build(context),
                        )
                        .build(context),
                )
                .child(
                    Grid::create()
                        .selector("linkwater")
                        .attach(GridColumn(2))
                        .child(
                            TextBlock::create()
                                .text("(2,0)")
                                .selector("linkwater")
                                .horizontal_alignment("Center")
                                .vertical_alignment("Center")
                                .build(context),
                        )
                        .build(context),
                )
                .child(
                    Grid::create()
                        .selector("goldendream")
                        .attach(GridColumn(0))
                        .attach(GridRow(1))
                        .attach(ColumnSpan(3))
                        .child(
                            TextBlock::create()
                                .text("(0,1) - ColumnSpan 3")
                                .selector("goldendream")
                                .horizontal_alignment(HorizontalAlignment(Alignment::Center))
                                .vertical_alignment(VerticalAlignment(Alignment::Center))
                                .build(context),
                        )
                        .build(context),
                )
                .build(context),
        )
    }
}

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - grid example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .theme(
                    ThemeValue::create()
                        .extension_css(include_str!("res/grid.css"))
                        .build(),
                )
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}
