use orbtk::prelude::*;

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - minimal example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(
                    Container::create()
                        .width(100.0)
                        .height(100.0)
                        .border_radius(2.0)
                        .border_thickness(1.0)
                        .border_brush("#000000")
                        .child(
                          CanvasThree::create().build(ctx)
                        )
                        .build(ctx),
                )
                .build(ctx)
        })
        .run();
}
