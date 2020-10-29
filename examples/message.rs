use orbtk::prelude::*;

use std::{thread, time};

enum Message {
    Increment,
}

#[derive(Default, AsAny)]
struct MainState {
    count: i32,
    my_thread: Option<thread::JoinHandle<()>>,
}

impl State for MainState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        let entity = ctx.widget().entity();
        let message_adapter = ctx.message_adapter();

        // increments a counter and send the result as message to `MainView`.
        self.my_thread = Some(thread::spawn(move || {
            let duration = time::Duration::from_secs(1);
            loop {
                thread::sleep(duration);
                message_adapter.send_message(Message::Increment, entity);
            }
        }));
    }
    fn messages(
        &mut self,
        mut messages: MessageReader,
        _registry: &mut Registry,
        ctx: &mut Context,
    ) {
        for message in messages.read::<Message>() {
            match message {
                Message::Increment => {
                    self.count += 1;
                    MainView::text_set(&mut ctx.widget(), self.count.to_string());
                }
            }
        }
    }
}

widget!(MainView<MainState> { text: String });

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.text("0").child(
            Stack::new()
                .margin(8)
                .spacing(4)
                .child(
                    TextBlock::new()
                        .text("Message counter example")
                        .style("header")
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("body")
                        .margin((0, 8, 0, 0))
                        .text(id)
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
                .title("OrbTk - message example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
