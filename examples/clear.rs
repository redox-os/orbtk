use std::cell::Cell;

use orbtk::prelude::*;

#[derive(Default)]
pub struct MainViewState {
    clear: Cell<bool>,
}

impl MainViewState {
    // Sets an action the state
    fn clear(&self) {
        self.clear.set(true);
    }
}

impl State for MainViewState {
    fn update(&self, _: &mut Registry, ctx: &mut Context<'_>) {
        if self.clear.get() {
            // Clears the text property of MainView and because
            // of the sharing also the text of the TextBox.
            ctx.widget().set("text", String16::from(""));
            self.clear.set(false);
        }
    }
}

widget!(MainView<MainViewState> {
    text: String16
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let state = self.clone_state();
        self.name("MainView").child(
            Stack::create()
                .orientation("horizontal")
                // By injecting the id of the parent the text property
                // is shared between the MainView and the TextBox. This
                // means both references the same String16 object.
                .child(TextBox::create().text(id).build(ctx))
                .child(
                    Button::create()
                        .margin((8.0, 0.0, 0.0, 0.0))
                        // mouse click event handler
                        .on_click(move |_| {
                            // Calls clear of the state of MainView
                            state.clear();
                            true
                        })
                        .text("Clear")
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - minimal example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(MainView::create().margin(4.0).build(ctx))
                .build(ctx)
        })
        .run();
}
