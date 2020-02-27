use orbtk::prelude::*;
use orbtk::theme::DEFAULT_THEME_CSS;

static CSS_EXT: &'static str = include_str!("../res/grid.css");

fn get_theme() -> ThemeValue {
    ThemeValue::create_from_css(DEFAULT_THEME_CSS)
        .extension_css(CSS_EXT)
        .build()
}

widget!(MainView);

impl Template for MainView {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").child(
            Grid::create()
                .columns(
                    Columns::create()
                        .column("*")
                        .column("auto")
                        .column(50.0)
                        .build(),
                )
                .rows(Rows::create().row("*").row("*").build())
                .child(
                    Grid::create()
                        .element("lynch")
                        .margin((10.0, 0.0, 0.0, 4.0))
                        .attach(Grid::column(0))
                        .child(
                            TextBlock::create()
                                .text("(0,0)")
                                .element("light-text")
                                .horizontal_alignment("center")
                                .vertical_alignment("center")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .child(
                    Grid::create()
                        .element("bluebayoux")
                        .margin(10.0)
                        .constraint(Constraint::create().width(150.0).build())
                        .attach(Grid::column(1))
                        .child(
                            TextBlock::create()
                                .text("(1,0)")
                                .element("white")
                                .horizontal_alignment("center")
                                .vertical_alignment("center")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .child(
                    Grid::create()
                        .element("linkwater")
                        .attach(Grid::column(2))
                        .child(
                            TextBlock::create()
                                .text("(2,0)")
                                .element("linkwater")
                                .horizontal_alignment("center")
                                .vertical_alignment("center")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .child(
                    Grid::create()
                        .element("goldendream")
                        .attach(Grid::column(0))
                        .attach(Grid::row(1))
                        .attach(Grid::column_span(3))
                        .child(
                            TextBlock::create()
                                .text("(0,1) - ColumnSpan 3")
                                .element("goldendream")
                                .horizontal_alignment("center")
                                .vertical_alignment("center")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx),
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
                .theme(get_theme())
                .resizeable(true)
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}
