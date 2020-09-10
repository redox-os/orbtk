use orbtk::prelude::*;

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            let pager = Pager::new()
                .width(100)
                .height(100)
                .child(TextBlock::new().text("test 1").build(ctx))
                .child(TextBlock::new().text("test 2").build(ctx))
                .child(TextBlock::new().text("test 3").build(ctx))
                .margin(4.0)
                .build(ctx);
            Window::new()
                .title("OrbTk - minimal example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(
                    Stack::new()
                        .child(
                            Button::new()
                                // .enabled(("next_enabled", pager))
                                .text("next")
                                .on_click(move |states, _| {
                                    states.get_mut::<PagerState>(pager).next();
                                    true
                                })
                                .build(ctx),
                        )
                        .child(
                            Button::new()
                                // .enabled(("previous_enabled", pager))
                                .text("previous")
                                .on_click(move |states, _| {
                                    states.get_mut::<PagerState>(pager).previous();
                                    true
                                })
                                .build(ctx),
                        )
                        .child(pager)
                        .build(ctx),
                )
                .build(ctx)
        })
        .run();
}
