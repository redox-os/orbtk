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
                    progress_bar(ctx.child("pgbar")).set_val(new_width);
                }
                ProgressEvent::Reset => {
                    progress_bar(ctx.child("pgbar")).set_val(0.0);
                }
                ProgressEvent::SetToFull => {
                    progress_bar(ctx.child("pgbar")).set_val(1.0);
                }
            }
            self.action = None;
        }
    }
}

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.child(
            Stack::new()
                .h_align("center")
                .margin((16.0, 16.0, 16.0, 16.0))
                .spacing(8.0)
                .child(
                    ProgressBar::new()
                        .id("pgbar")
                        .val(0.0)
                        .width(512.0)
                        .build(ctx),
                )
                .child(
                    Stack::new()
                        .h_align("center")
                        .spacing(8.0)
                        .child(
                            Button::new()
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
                            Button::new()
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
                            Button::new()
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
            Window::new()
                .title("OrbTk - ProgressBar example")
                .position((0.0, 0.0))
                .size(720.0, 576.0)
                .borderless(false)
                .resizeable(true)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
