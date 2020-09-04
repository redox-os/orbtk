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
    action: Option<Action>,
}

impl MainViewState {
    fn action(&mut self, action: Action) {
        self.action = Some(action);
    }
}

impl State for MainViewState {
    fn update(&mut self, registry: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                Action::Load => {
                    // load label from settings file.
                    if let Ok(global) = registry
                        .get::<Settings>("settings")
                        .load::<Global>("global")
                    {
                        MainView::text_set(&mut ctx.widget(), global.label);
                    }

                    MainView::info_text_set(
                        &mut ctx.widget(),
                        String::from("Label loaded from settings file."),
                    );
                }
                Action::Save => {
                    // save label to settings file.
                    registry
                        .get_mut::<Settings>("settings")
                        .save(
                            "global",
                            &Global {
                                label: ctx.widget().get::<String>("text").to_string(),
                            },
                        )
                        .unwrap();

                    MainView::info_text_set(
                        &mut ctx.widget(),
                        String::from("Label saved to settings file."),
                    );
                }
                Action::Clear => {
                    MainView::text_set(&mut ctx.widget(), String::default());
                    MainView::info_text_set(&mut ctx.widget(), String::default());
                }
            }

            self.action = None;
        }
    }
}

widget!(MainView<MainViewState> {
    text: String,
    info_text: String
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").child(
            Grid::new()
                .rows(Rows::create().push(36).push(4).push("auto").build())
                .columns(
                    Columns::create()
                        .push(160)
                        .push(4)
                        .push("auto")
                        .push(4)
                        .push("auto")
                        .push(4)
                        .push("auto"),
                )
                .child(TextBox::new().v_align("center").text(id).build(ctx))
                .child(
                    Button::new()
                        .style("button_single_content")
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
                    Button::new()
                        .style("button_single_content")
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
                    Button::new()
                        .style("button_single_content")
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
                    TextBlock::new()
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
    Application::from_name("orbtk_settings")
        .window(|ctx| {
            Window::new()
                .title("OrbTk - settings example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(MainView::new().margin(4.0).build(ctx))
                .build(ctx)
        })
        .run();
}

// helper to request MainViewState
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut MainViewState {
    states.get_mut(id)
}
