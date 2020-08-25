use orbtk::{
    prelude::*,
    theme::{COLORS_RON, DARK_THEME_RON, FONTS_RON},
    theming::config::ThemeConfig,
};

static DARK_EXT: &str = include_str!("../res/grid.ron");

fn theme() -> Theme {
    Theme::from_config(
        ThemeConfig::from(DARK_THEME_RON)
            .extend(ThemeConfig::from(DARK_EXT))
            .extend(ThemeConfig::from(COLORS_RON))
            .extend(ThemeConfig::from(FONTS_RON)),
    )
}

widget!(MainView);

impl Template for MainView {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").child(
            Grid::new()
                .columns(Columns::create().push("*").push("auto").push(50))
                .rows(Rows::create().push("*").push("*"))
                .child(
                    Grid::new()
                        .style("lynch")
                        .margin((10, 0, 0, 4))
                        .attach(Grid::column(0))
                        .child(
                            TextBlock::new()
                                .text("(0,0)")
                                .style("light_text")
                                .h_align("center")
                                .v_align("center")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .child(
                    Grid::new()
                        .style("bluebayoux")
                        .margin(10)
                        .constraint(Constraint::create().width(150))
                        .attach(Grid::column(1))
                        .child(
                            TextBlock::new()
                                .text("(1,0)")
                                .style("body")
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
                                .style("dark_text")
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
                                .style("dark_text")
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
        .theme(theme())
        .window(|ctx| {
            Window::new()
                .title("OrbTk - grid example")
                .position((100, 100))
                .size(420, 730)
                .resizeable(true)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
