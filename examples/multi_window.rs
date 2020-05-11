use orbtk::prelude::*;

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - multi window example window 1")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(
                    Stack::create()
                        .child(TextBlock::create().text("Window 1").margin(4.0).build(ctx))
                        .child(Button::create().margin(4.0).text("Click me").build(ctx))
                        .build(ctx),
                )
                .build(ctx)
        })
        .window(|ctx| {
            Window::create()
                .title("OrbTk - multi window example window 2")
                .position((600.0, 100.0))
                .size(420.0, 730.0)
                .child(
                    Stack::create()
                        .child(TextBlock::create().text("Window 2").margin(4.0).build(ctx))
                        .child(Button::create().margin(4.0).text("Click me").build(ctx))
                        .build(ctx),
                )
                .build(ctx)
        })
        .run();
}
