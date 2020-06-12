use orbtk::prelude::*;

widget!(MainView);

impl Template for MainView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            Stack::new()
                .spacing(8.0)
                .orientation("vertical")
                .horizontal_alignment("center")
                .child(
                    TextBlock::new()
                        .text("Tyre pressure")
                        .font_size(20.0)
                        .build(ctx),
                )
                .child(
                    NumericBox::new()
                        .max(123.0)
                        .step(0.123)
                        .val(0.123)
                        .build(ctx),
                )
                .child(Button::new().text("Blow air").build(ctx))
                .build(ctx),
        )
    }
}

fn main() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk - NumericBox example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .resizeable(true)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
