use orbtk::prelude::*;

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk - image example")
                .position((100.0, 100.0))
                .size(800.0, 420.0)
                .child(ImageWidget::new().image("res/orbtk_space.png").build(ctx))
                .build(ctx)
        })
        .run();
}
