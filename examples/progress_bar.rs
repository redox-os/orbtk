use orbtk::prelude::*;

#[derive(Copy, Clone)]
enum ProgressEvent {
    Advance(f64),
    Reset,
    SetToFull,
}

#[derive(Default, AsAny)]
struct MainViewState {
    action: Option<ProgressEvent>,
}

widget!(MainView<MainViewState>);

impl MainViewState {
    fn action(&mut self, action: impl Into<Option<ProgressEvent>>) {
        self.action = action.into();
    }
}

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if let Some(action) = self.action {
            match action {
                ProgressEvent::Advance(amount) => {
                    let old_width = ctx.child("pgbar").clone_or_default::<f64>("val");
                    let new_width = old_width + amount;
                    // Set the ProgressBar's val property to the calculated percentage
                    // (whereas 0.0 means 0 %, and 1.0 means 100 %) to advance the progress
                    ctx.child("pgbar").set::<f64>("val", new_width);
                }
                ProgressEvent::Reset => {
                    ctx.child("pgbar").set::<f64>("val", 0.0);
                }
                ProgressEvent::SetToFull => {
                    ctx.child("pgbar").set::<f64>("val", 1.0);
                }
            }
            self.action = None;
        }
    }
}

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            Stack::create()
                .horizontal_alignment("center")
                .margin((16.0, 16.0, 16.0, 16.0))
                .spacing(8.0)
                .child(
                    ProgressBar::create()
                        .id("pgbar")
                        .val(0.0)
                        .width(512.0)
                        .build(ctx),
                )
                .child(
                    Stack::create()
                        .horizontal_alignment("center")
                        .spacing(8.0)
                        .child(
                            Button::create()
                                .text("Progress by 25%")
                                .width(256.0)
                                .on_click(move |states, _| -> bool {
                                    states
                                        .get_mut::<MainViewState>(id)
                                        .action(ProgressEvent::Advance(0.25));
                                    true
                                })
                                .build(ctx),
                        )
                        .child(
                            Button::create()
                                .text("Reset")
                                .width(256.0)
                                .on_click(move |states, _| -> bool {
                                    states
                                        .get_mut::<MainViewState>(id)
                                        .action(ProgressEvent::Reset);
                                    true
                                })
                                .build(ctx),
                        )
                        .child(
                            Button::create()
                                .text("Set to 100%")
                                .width(256.0)
                                .on_click(move |states, _| -> bool {
                                    states
                                        .get_mut::<MainViewState>(id)
                                        .action(ProgressEvent::SetToFull);
                                    true
                                })
                                .build(ctx),
                        )
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
                .title("OrbTk - ProgressBar example")
                .position((0.0, 0.0))
                .size(720.0, 576.0)
                .borderless(false)
                .resizeable(true)
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}
