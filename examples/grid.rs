use orbtk::*;

struct MainView;

// todo: check running by each system

// todo: use ParentType::Single as default

impl Widget for MainView {
    type Template = Template;

    fn create() -> Self::Template {
        Template::new().child(
            Grid::create()
                .selector("green")
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
                        .selector("blue")
                        .attach_property(GridColumn(0))
                        .child(TextBlock::create().text("(0,0)")),
                )
                .child(
                    Grid::create()
                        .selector("yellow")
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
                        .selector("red")
                        .attach_property(GridColumn(2))
                        .child(TextBlock::create().text("(1,1)")),
                )
                .child(
                    Grid::create()
                        .selector("olive")
                        .attach_property(GridColumn(1))
                        .attach_property(GridRow(1))
                        .attach_property(ColumnSpan(2))
                        .child(TextBlock::create().text("(1,1)")),
                ),
        ).debug_name("MainView")
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
