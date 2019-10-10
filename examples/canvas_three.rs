use orbtk::prelude::*;

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - canvas three example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(CanvasThree::create().build(ctx))
                .build(ctx)
        })
        .run();
}
