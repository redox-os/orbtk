use std::cell::Cell;

use serde_derive::Serialize;

use orbtk::prelude::*;

#[derive(Default, Serialize)]
pub struct MainViewSettings {
    pub label: String
}

#[derive(Default)]
pub struct MainViewState {
    clear: Cell<bool>,
}

impl MainViewState {
    fn clear(&self) {
        self.clear.set(true);
    }
}

impl State for MainViewState {
    fn update(&self, registry: &mut Registry, ctx: &mut Context<'_>) {
        if self.clear.get() {
            let mut global = MainViewSettings::default();
            global.label = "test".to_string();
            registry.get_mut::<Settings>("settings").save("global", &global);
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
                .child(TextBox::create().height(32.0).text(id).build(ctx))
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
    Application::from_name("orbtk-settings")
        .window(|ctx| {
            Window::create()
                .title("OrbTk - settings example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(MainView::create().margin(4.0).build(ctx))
                .build(ctx)
        })
        .run();
}
