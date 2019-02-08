// This examples shows how you could send messages between different widgets / views.

use std::{cell::Cell, rc::Rc};

use orbtk::*;

#[derive(Default)]
struct SenderState {
    send_message: Cell<bool>,
}

impl State for SenderState {
    fn update(&self, context: &mut Context<'_>) {
        if self.send_message.get() {
            context.send_message("receiver_view", StringMessage::from("Hello from sender."));
        }
    }
}

struct SenderView;

impl Widget for SenderView {
    type Template = Template;

    fn create() -> Self::Template {
        let state = Rc::new(SenderState::default());
        let send_state = state.clone();

        Template::new()
            .debug_name("SenderView")
            .child(
                Button::create()
                    .text("Send message")
                    .on_click(move |_| -> bool {
                        send_state.send_message.set(true);
                        true
                    }),
            )
            .state(state)
    }
}

struct ReceiverState;

impl State for ReceiverState {
    fn receive_messages(&self, _context: &mut Context<'_>, messages: &Vec<MessageBox>) {
        for message in messages {
            if let Ok(message) = message.downcast_ref::<StringMessage>() {
                println!("Message received: {}", message.0);
            }
        }
    }
}

struct ReceiverView;

impl Widget for ReceiverView {
    type Template = Template;

    fn create() -> Self::Template {
        Template::new()
            .debug_name("ReceiverView")
            .child(Container::create().child(SenderView::create()))
            .property(Selector::default().id("receiver_view"))
            .state(Rc::new(ReceiverState))
    }
}

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .bounds((100.0, 100.0, 420.0, 730.0))
        .title("OrbTk - minimal example")
        .root(ReceiverView::create())
        .debug_flag(true)
        .build();
    application.run();
}
