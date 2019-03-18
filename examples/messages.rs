// // This examples shows how you could send messages between different widgets / views.

// use orbtk::*;

// mod sender {
//     use orbtk::*;
//     use std::{cell::Cell, rc::Rc};

//     #[derive(Default)]
//     struct SenderState {
//         send_message: Cell<bool>,
//     }

//     impl State for SenderState {
//         fn update(&self, context: &mut Context<'_>) {
//             if self.send_message.get() {
//                 context.send_message("receiver_view", StringMessage::from("Hello from sender."));
//             }
//         }
//     }

//     widget!(SenderView);

//     impl Widget for SenderView {
//         fn create() -> Self {
//             let state = Rc::new(SenderState::default());
//             let send_state = state.clone();

//             SenderView::new()
//                 .debug_name("SenderView")
//                 .child(
//                     Button::create()
//                         .text("Send message")
//                         .on_click(move |_| -> bool {
//                             send_state.send_message.set(true);
//                             true
//                         }),
//                 )
//                 .state(state)
//         }
//     }
// }

// mod receiver {
//     use super::sender;
//     use orbtk::*;
//     use std::rc::Rc;

//     struct ReceiverState;

//     impl State for ReceiverState {
//         fn receive_messages(&self, _context: &mut Context<'_>, messages: &Vec<MessageBox>) {
//             for message in messages {
//                 if let Ok(message) = message.downcast_ref::<StringMessage>() {
//                     println!("Message received: {}", message.0);
//                 }
//             }
//         }
//     }

//     widget!(ReceiverView);

//     impl Widget for ReceiverView {
//         fn create() -> Self {
//             ReceiverView::new()
//                 .debug_name("ReceiverView")
//                 .child(Container::create().child(sender::SenderView::create()))
//                 .attach(Selector::default().id("receiver_view"))
//                 .state(Rc::new(ReceiverState))
//         }
//     }
// }

// fn main() {
//     let mut application = Application::default();
//     application
//         .create_window()
//         .bounds((100.0, 100.0, 420.0, 730.0))
//         .title("OrbTk - minimal example")
//         .root(receiver::ReceiverView::create())
//         .debug_flag(true)
//         .build();
//     application.run();
// }

fn main() {
    
}