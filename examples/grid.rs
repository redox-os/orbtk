use orbtk::*;

struct MainView;

// todo: check running by each system

// todo: use ParentType::Single as default

impl Widget for MainView {
    fn create() -> Template {
        Template::default().debug_name("MainView").child(
            Grid::create()
                .selector("green")
                .property(
                    Columns::create()
                        .column("*")
                        .column("Auto")
                        .column(50.0)
                        .build(),
                )
                .property(Rows::create().row("*").row("*").build())
                .child(
                    Grid::create()
                        .selector("blue")
                        .property(GridColumn(0))
                        .child(TextBlock::create().property(Label::from("(0,0)"))),
                )
                .child(
                    Grid::create()
                        .selector("yellow")
                        .property(GridColumn(1))
                        .child(
                            TextBlock::create()
                                .property(Label::from("(1,0)"))
                                .horizontal_alignment("Center")
                                .vertical_alignment("Center")
                        ),
                )
                .child(
                    Grid::create()
                        .selector("red")
                        .property(GridColumn(2))
                        .child(TextBlock::create().property(Label::from("(1,1)"))),
                )
                .child(
                    Grid::create()
                        .selector("olive")
                        .property(GridColumn(1))
                        .property(GridRow(1))
                        .property(ColumnSpan(2))
                        .child(TextBlock::create().property(Label::from("(1,1)"))),
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
