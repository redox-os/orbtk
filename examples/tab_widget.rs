use orbtk::prelude::*;

fn main() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk - tab widget example")
                .position((100.0, 100.0))
                .size(600.0, 500.0)
                .resizeable(true)
                .child(
                    TabWidget::new()
                        .tab(
                            "Tab header 1",
                            TextBlock::new().text("Tab content 1").build(ctx),
                        )
                        .tab(
                            "Tab header 2",
                            TextBlock::new().text("Tab content 2").build(ctx),
                        )
                        .tab(
                            "Tab header 3",
                            TextBlock::new().text("Tab content 3").build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx)
        })
        .run();
}
