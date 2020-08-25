use orbtk::prelude::*;

widget!(MainView);

impl Template for MainView {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").child(
            Stack::new()
                .child(
                    TextBlock::new()
                        .margin((0.0, 0.0, 0.0, 8.0))
                        .text("Stack vertical")
                        .style("text_block_header")
                        .build(ctx),
                )
                .child(
                    Stack::new()
                        .spacing(4.0)
                        .child(
                            Button::new()
                                .style("button_single_content")
                                .text("left")
                                .h_align("start")
                                .build(ctx),
                        )
                        .child(
                            Button::new()
                                .style("button_single_content")
                                .text("center")
                                .h_align("center")
                                .build(ctx),
                        )
                        .child(
                            Button::new()
                                .style("button_single_content")
                                .text("stretch")
                                .h_align("stretch")
                                .build(ctx),
                        )
                        .child(
                            Button::new()
                                .style("button_single_content")
                                .text("right")
                                .h_align("end")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .margin((0.0, 0.0, 0.0, 8.0))
                        .text("Stack horizontal")
                        .style("text_block_header")
                        .build(ctx),
                )
                .child(
                    Stack::new()
                        .orientation("horizontal")
                        .spacing(4.0)
                        .height(100.0)
                        .child(
                            Button::new()
                                .style("button_single_content")
                                .text("top")
                                .v_align("start")
                                .build(ctx),
                        )
                        .child(
                            Button::new()
                                .style("button_single_content")
                                .text("center")
                                .v_align("center")
                                .build(ctx),
                        )
                        .child(
                            Button::new()
                                .style("button_single_content")
                                .height(0.0)
                                .text("stretch")
                                .v_align("stretch")
                                .build(ctx),
                        )
                        .child(
                            Button::new()
                                .style("button_single_content")
                                .text("bottom")
                                .v_align("end")
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
            Window::new()
                .title("OrbTk - stack example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .resizeable(true)
                .child(MainView::new().margin(4.0).build(ctx))
                .build(ctx)
        })
        .run();
}
