use orbtk::prelude::*;

widget!(MainView);

impl Template for MainView {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").child(
            Stack::create()
                .child(
                    Button::create()
                        .text("left")
                        .horizontal_alignment("start")
                        .build(ctx),
                )
                .child(
                    Button::create()
                        .margin((0.0, 8.0, 0.0, 0.0))
                        .text("center")
                        .horizontal_alignment("center")
                        .build(ctx),
                )
                .child(
                    Button::create()
                        .margin((0.0, 8.0, 0.0, 0.0))
                        .text("stretch")
                        .horizontal_alignment("stretch")
                        .build(ctx),
                )
                .child(
                    Button::create()
                        .margin((0.0, 8.0, 0.0, 0.0))
                        .text("right")
                        .horizontal_alignment("end")
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
            Window::create()
                .title("OrbTk - stack example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(MainView::create().margin(4.0).build(ctx))
                .build(ctx)
        })
        .run();
}
