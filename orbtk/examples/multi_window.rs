/// TODO: Fix mutli window to work properly as web instance.
use orbtk::prelude::*;

// constants
pub static ID_WINDOW_1: &str = "Window 1";
pub static ID_WINDOW_1_BUTTON: &str = "Window-1 Button";
pub static ID_WINDOW_1_STACK: &str = "Window-1 Stack";
pub static ID_WINDOW_1_TEXTBLOCK: &str = "Window-1 TextBlock";
pub static ID_WINDOW_2: &str = "Window 2";
pub static ID_WINDOW_2_BUTTON: &str = "Window-2 Button";
pub static ID_WINDOW_2_STACK: &str = "Window-2 Stack";
pub static ID_WINDOW_2_TEXTBLOCK: &str = "Window-2 TextBlock";
pub static ID_WINDOW_3: &str = "Window 3";
pub static ID_WINDOW_3_BUTTON: &str = "Window-3 Button";
pub static ID_WINDOW_3_STACK: &str = "Window-3 Stack";
pub static ID_WINDOW_3_TEXTBLOCK: &str = "Window-3 TextBlock";

// [Window1View]

widget!(Window1View<Window1State>);

impl Template for Window1View {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            Stack::new()
                .id(ID_WINDOW_1_STACK)
                .name(ID_WINDOW_1_STACK)
                .child(
                    TextBlock::new()
                        .id(ID_WINDOW_1_TEXTBLOCK)
                        .name(ID_WINDOW_1_TEXTBLOCK)
                        .text("Window 1")
                        .margin(4.0)
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .id(ID_WINDOW_1_BUTTON)
                        .name(ID_WINDOW_1_BUTTON)
                        .on_click(move |states, _| {
                            states.get_mut::<Window1State>(id).toggle_window();
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

// [Window1ViewState]

#[derive(AsAny, Default)]
struct Window1State {
    /// Toggle visibility of given window.
    toggle_window: bool,
}

impl Window1State {
    fn toggle_window(&mut self) {
        self.toggle_window = true;
    }
}

impl State for Window1State {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if self.toggle_window {
            Button::enabled_set(&mut ctx.child(ID_WINDOW_1_BUTTON), false);
            ctx.show_window(|ctx| {
                Window::new()
                    .id(ID_WINDOW_3)
                    .name(ID_WINDOW_3)
                    .title("OrbTk - Dialog")
                    .position((120.0, 120.0))
                    .size(420.0, 75.0)
                    .child(
                        Stack::new()
                            .id(ID_WINDOW_3_STACK)
                            .name(ID_WINDOW_3_STACK)
                            .child(
                                TextBlock::new()
                                    .id(ID_WINDOW_3_TEXTBLOCK)
                                    .name(ID_WINDOW_3_TEXTBLOCK)
                                    .text("New window")
                                    .margin(4.0)
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx)
            });
            self.toggle_window = false;
        }
    }
}

/// The multi-windows example.
///
/// This OrbTk example application implements two side by side
/// windows:
///
/// * [Window 1]: The view will show an ordenary window that has an
/// associated state. Via a button inside its view you may trigger an
/// associated function, that will open a new child window (`dialog`).
///
/// * [Window 2]: The view will show another ordenary window. This one
/// is static, no state is associated.
pub fn main() {
    // use this only if you want to run it as web application.
    // orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::new()
                .id(ID_WINDOW_1)
                .name(ID_WINDOW_1)
                .title("OrbTk - multi-window 1")
                .position((100.0, 100.0))
                .size(420.0, 130.0)
                .child(Window1View::new().build(ctx))
                .build(ctx)
        })
        .window(|ctx| {
            Window::new()
                .id(ID_WINDOW_2)
                .name(ID_WINDOW_2)
                .title("OrbTk - multi-window 2")
                .position((600.0, 100.0))
                .size(420.0, 130.0)
                .child(
                    Stack::new()
                        .id(ID_WINDOW_2_STACK)
                        .name(ID_WINDOW_2_STACK)
                        .child(
                            TextBlock::new()
                                .id(ID_WINDOW_2_TEXTBLOCK)
                                .name(ID_WINDOW_2_TEXTBLOCK)
                                .text("Window 2")
                                .margin(4.0)
                                .build(ctx),
                        )
                        .child(
                            Button::new()
                                .id(ID_WINDOW_2_BUTTON)
                                .name(ID_WINDOW_2_BUTTON)
                                .margin(4.0)
                                .text("Click me")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx)
        })
        .run();
}
