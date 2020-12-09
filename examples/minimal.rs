use orbtk::prelude::*;

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk - minimal example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(
                    Grid::new()
                        .rows("32, *")
                        .child(Container::new().background("green").build(ctx))
                        .child(
                            Container::new()
                                .attach(Grid::column(1))
                                .background("blue")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx)
        })
        .run();
}
