use orbtk::prelude::*;

widget!(MainView);

impl Template for MainView {
    fn template(self, _: Entity, ctx: &mut BuildContext) -> Self {
        let container = Container::create()
            .background("#dfebf5")
            .width(200.0)
            .height(200.0)
            .child(
                TextBlock::create()
                    .foreground("#3b434a")
                    .text("Overlay")
                    .vertical_alignment("center")
                    .horizontal_alignment("center")
                    .build(ctx),
            )
            .build(ctx);

        ctx.append_child_to_overlay(container).unwrap();
        self.name("MainView").child(
            Container::create()
                .background("#e1bc21")
                .child(
                    TextBlock::create()
                        .text("MainView")
                        .element("h1")
                        .vertical_alignment("center")
                        .horizontal_alignment("center")
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
                .title("OrbTk - overlay example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}
