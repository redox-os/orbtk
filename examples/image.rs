use orbtk::prelude::*;

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - minimal example")
                .position((100.0, 100.0))
                .size(800.0, 420.0)
                .child(ImageWidget::create().image("res/orbtk-space.png").build(ctx)) 
                .build(ctx)
        })
        .run();
}