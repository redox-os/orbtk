use orbtk::prelude::*;

widget!(MainView);

impl Template for MainView {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        let container = Container::new()
            .background("#dfebf5")
            .width(200.0)
            .height(200.0)
            .child(
                TextBlock::new()
                    .foreground("#3b434a")
                    .text("Overlay")
                    .v_align("center")
                    .h_align("center")
                    .build(ctx),
            )
            .build(ctx);

        ctx.append_child_to_overlay(container).unwrap();
        self.name("MainView").child(
            Container::new()
                .background("#e1bc21")
                .child(
                    TextBlock::new()
                        .text("MainView")
                        .style("text_block_header")
                        .v_align("center")
                        .h_align("center")
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
                .title("OrbTk - overlay example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
