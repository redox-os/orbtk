mod state;

use self::state::{AppState, Event};

use orbtk::prelude::*;

widget! {
    MainView<AppState> {
        orientation: Orientation,
        spacing: f64
    }
}

impl Template for MainView {
    fn template(mut self, id: Entity, ctx: &mut BuildContext) -> Self {
        let entry = TextBox::create()
            .vertical_alignment(Alignment::End)
            .on_activate(move |ctxt, entity| {
                ctxt.get_mut::<AppState>(id).send(Event::Create(entity));
            })
            .build(ctx);

        let notes = Stack::create()
            .orientation(Orientation::Vertical)
            .spacing(4)
            .build(ctx);

        self.state.notes = notes;

        let scroll_viewer = ScrollViewer::create()
            .scroll_viewer_mode(("disabled", "auto"))
            .child(notes)
            .build(ctx);

        self.name("main-view")
            .orientation(Orientation::Vertical)
            .margin(4)
            .child(
                Container::create()
                    .child(scroll_viewer)
                    .child(
                        ScrollIndicator::create()
                            .padding(2.0)
                            .scroll_offset(scroll_viewer)
                            .build(ctx),
                    )
                    .build(ctx)
            )
            .child(entry)
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(StackLayout::new())
    }
}

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("Orbital To Do")
                .position((500.0, 500.0))
                .size(400.0, 400.0)
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}
