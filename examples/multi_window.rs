/// mutli window will not work properly on web now. But it will be fixed.
use orbtk::prelude::*;

#[derive(AsAny, Default)]
struct MainState {
    show_window: bool,
}

impl MainState {
    fn show_window(&mut self) {
        self.show_window = true;
    }
}

impl State for MainState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if self.show_window {
            Button::enabled_set(&mut ctx.child("button"), false);
            ctx.show_window(|ctx| {
                Window::new()
                    .title("Dialog")
                    .position((120.0, 120.0))
                    .size(100.0, 75.0)
                    .child(
                        Stack::new()
                            .child(TextBlock::new().text("New window").margin(4.0).build(ctx))
                            .build(ctx),
                    )
                    .build(ctx)
            });
            self.show_window = false;
        }
    }
}

widget!(MainView<MainState>);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            Stack::new()
                .child(TextBlock::new().text("Window 1").margin(4.0).build(ctx))
                .child(
                    Button::new()
                        .id("button")
                        .on_click(move |states, _| {
                            states.get_mut::<MainState>(id).show_window();
                            true
                        })
                        .margin(4.0)
                        .text("Show window")
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
                .title("OrbTk - multi window example window 1")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .window(|ctx| {
            Window::new()
                .title("OrbTk - multi window example window 2")
                .position((600.0, 100.0))
                .size(420.0, 730.0)
                .child(
                    Stack::new()
                        .child(TextBlock::new().text("Window 2").margin(4.0).build(ctx))
                        .child(Button::new().margin(4.0).text("Click me").build(ctx))
                        .build(ctx),
                )
                .build(ctx)
        })
        .run();
}
