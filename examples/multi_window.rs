use orbtk::prelude::*;

#[derive(AsAny, Default)]
struct MainState {
    show_dialog: bool,
}

impl MainState {
    fn show_dialog(&mut self) {
        self.show_dialog = true;
    }
}

impl State for MainState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if self.show_dialog {
            ctx.child("button").set("enabled", false);
            ctx.show_window(|ctx| {
                Window::create()
                    .title("Dialog")
                    .position((120.0, 120.0))
                    .size(100.0, 75.0)
                    .child(
                        Stack::create()
                            .child(TextBlock::create().text("Dialog").margin(4.0).build(ctx))
                            .build(ctx),
                    )
                    .build(ctx)
            });
            self.show_dialog = true;
        }
    }
}

widget!(MainView<MainState>);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            Stack::create()
                .child(TextBlock::create().text("Window 1").margin(4.0).build(ctx))
                .child(
                    Button::create()
                        .id("button")
                        .on_click(move |states, _| {
                            states.get_mut::<MainState>(id).show_dialog();
                            true
                        })
                        .margin(4.0)
                        .text("Show dialog")
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
                .title("OrbTk - multi window example window 1")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(MainView::create().build(ctx))
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
