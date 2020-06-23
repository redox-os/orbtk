use orbtk::prelude::*;
use orbtk::theme::DEFAULT_THEME_CSS;

static CSS_EXT: &'static str = include_str!("../res/grid.css");

fn get_theme() -> Theme {
    Theme::create_from_css(DEFAULT_THEME_CSS)
        .extension_css(CSS_EXT)
        .build()
}

widget!(MainView);

impl Template for MainView {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").child(
            Grid::new()
                .columns(Columns::new().add("*").add("auto").add(50.0))
                .rows(Rows::new().add("*").add("*"))
                .child(
                    Grid::new()
                        .style("lynch")
                        .margin((10.0, 0.0, 0.0, 4.0))
                        .attach(Grid::column(0))
                        .child(
                            TextBlock::new()
                                .text("(0,0)")
                                .style("light-text")
                                .h_align("center")
                                .v_align("center")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .child(
                    Grid::new()
                        .style("bluebayoux")
                        .margin(10.0)
                        .constraint(Constraint::new().width(150.0))
                        .attach(Grid::column(1))
                        .child(
                            TextBlock::new()
                                .text("(1,0)")
                                .style("white")
                                .h_align("center")
                                .v_align("center")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .child(
                    Grid::new()
                        .style("linkwater")
                        .attach(Grid::column(2))
                        .child(
                            TextBlock::new()
                                .text("(2,0)")
                                .style("linkwater")
                                .h_align("center")
                                .v_align("center")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .child(
                    Grid::new()
                        .style("goldendream")
                        .attach(Grid::column(0))
                        .attach(Grid::row(1))
                        .attach(Grid::column_span(3))
                        .child(
                            TextBlock::new()
                                .text("(0,1) - ColumnSpan 3")
                                .style("goldendream")
                                .h_align("center")
                                .v_align("center")
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
            Window::new()
                .title("OrbTk - grid example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .theme(get_theme())
                .resizeable(true)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
