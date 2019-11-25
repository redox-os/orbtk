use std::cell::Cell;

use serde_derive::{Deserialize, Serialize};

use orbtk::prelude::*;

#[derive(Copy, Clone, PartialEq)]
enum Action {
    Load,
    Save,
    Clear,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Global {
    pub label: String,
}

#[derive(Default, AsAny)]
pub struct MainViewState {
    action: Cell<Option<Action>>,
}

impl MainViewState {
    fn action(&self, action: Action) {
        self.action.set(Some(action));
    }
}

impl State for MainViewState {
    fn update(&self, registry: &mut Registry, ctx: &mut Context<'_>) {
        if let Some(action) = self.action.get() {
            match action {
                Action::Load => {
                    // load label from settings file.
                    if let Ok(global) = registry
                        .get::<Settings>("settings")
                        .load::<Global>("global")
                    {
                        ctx.widget().set("text", String16::from(global.label));
                    }

                    ctx.widget().set("info_text", String16::from("Label loaded from settings file."));
                }
                Action::Save => {
                    // save label to settings file.
                    registry.get_mut::<Settings>("settings").save(
                        "global",
                        &Global {
                            label: ctx.widget().get::<String16>("text").to_string(),
                        },
                    ).unwrap();
                    ctx.widget().set("info_text", String16::from("Label saved to settings file."));
                }
                Action::Clear => {
                    ctx.widget().set("text", String16::default());
                    ctx.widget().set("info_text", String16::from(""));
                }
            }

            self.action.set(None);
        }
    }
}

widget!(MainView<MainViewState> {
    text: String16,
    info_text: String16
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").child(
            Grid::create()
                .rows(Rows::create().row(32.0).row(4.0).row("auto").build())
                .columns(
                    Columns::create()
                        .column(160.0)
                        .column(4.0)
                        .column("Auto")
                        .column(4.0)
                        .column("Auto")
                        .column(4.0)
                        .column("Auto")
                        .build(),
                )
                .child(TextBox::create().text(id).build(ctx))
                .child(
                    Button::create()
                        .attach(Grid::row(0))
                        .attach(Grid::column(2))
                        .on_click(move |states, _| {
                            state(id, states).action(Action::Load);
                            true
                        })
                        .text("Load")
                        .build(ctx),
                )
                .child(
                    Button::create()
                        .attach(Grid::row(0))
                        .attach(Grid::column(4))
                        .on_click(move |states, _| {
                            state(id, states).action(Action::Save);
                            true
                        })
                        .text("Save")
                        .build(ctx),
                )
                .child(
                    Button::create()
                        .attach(Grid::row(0))
                        .attach(Grid::column(6))
                        .on_click(move |states, _| {
                            state(id, states).action(Action::Clear);
                            true
                        })
                        .text("Clear")
                        .build(ctx),
                )
                .child(
                    TextBlock::create()
                        .attach(Grid::row(2))
                        .attach(Grid::column(0))
                        .text(("info_text", id))
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

// helper to request MainViewState
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut MainViewState {
    states.get_mut(id)
}