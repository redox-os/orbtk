use orbtk::prelude::*;

widget!(MainView);

impl Template for MainView {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").child(
            Stack::create()
                .child(
                    TextBlock::create()
                        .margin((0.0, 0.0, 0.0, 8.0))
                        .text("Stack vertical")
                        .selector("h1")
                        .build(ctx),
                )
                .child(
                    Stack::create()
                        .spacing(4.0)
                        .child(
                            Button::create()
                                .text("left")
                                .horizontal_alignment("start")
                                .build(ctx),
                        )
                        .child(
                            Button::create()
                                .text("center")
                                .horizontal_alignment("center")
                                .build(ctx),
                        )
                        .child(
                            Button::create()
                                .text("stretch")
                                .horizontal_alignment("stretch")
                                .build(ctx),
                        )
                        .child(
                            Button::create()
                                .text("right")
                                .horizontal_alignment("end")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .child(
                    TextBlock::create()
                        .margin((0.0, 0.0, 0.0, 8.0))
                        .text("Stack horizontal")
                        .selector("h1")
                        .build(ctx),
                )
                .child(
                    Stack::create()
                        .orientation("horitzontal")
                        .spacing(4.0)
                        .child(
                            Button::create()
                                .text("top")
                                .vertical_alignment("start")
                                .build(ctx),
                        )
                        .child(
                            Button::create()
                                .text("center")
                                .vertical_alignment("center")
                                .build(ctx),
                        )
                        .child(
                            Button::create()
                                .text("stretch")
                                .vertical_alignment("stretch")
                                .build(ctx),
                        )
                        .child(
                            Button::create()
                                .text("bottom")
                                .vertical_alignment("end")
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
        .window(|ctx| {
            Window::create()
                .title("OrbTk - stack example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .resizeable(true)
                .child(MainView::create().margin(4.0).build(ctx))
                .build(ctx)
        })
        .run();
}
