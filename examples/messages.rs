// // This examples shows how you could send messages between different widgets / views.

// use std::{cell::Cell, rc::Rc};

// use orbtk::*;

// #[derive(Default)]
// struct SenderState {
//     send_message: Cell<bool>,
// }

// impl State for SenderState {
//     fn update(&self, context: &mut Context<'_>) {
//         if self.send_message.get() {
//             context.send_message("receiver_view", StringMessage::from("Hello from sender."));
//         }
//     }
// }

// struct SenderView;

// impl Widget for SenderView {
//     fn create() -> Template {
//         let state = Rc::new(SenderState::default());
//         let send_state = state.clone();

//         Template::default()
//             .debug_name("SenderView")
//             .child(
//                 Button::create()
//                     .property(Text::from("Send message"))
//                     .event_handler(MouseEventHandler::default().on_click(Rc::new(
//                         move |_pos: Point| -> bool {
//                             send_state.send_message.set(true);
//                             true
//                         },
//                     ))),
//             )
//             .state(state)
//     }
// }

// struct ReceiverState;

// impl State for ReceiverState {
//     fn receive_messages(&self, _context: &mut Context<'_>, messages: &Vec<MessageBox>) {
//         for message in messages {
//             if let Ok(message) = message.downcast_ref::<StringMessage>() {
//                 println!("Message received: {}", message.0);
//             }
//         }
//     }
// }

// struct ReceiverView;

// impl Widget for ReceiverView {
//     fn create() -> Template {
//         Template::default()
//            .parent_type(ParentType::Single)
//             .debug_name("ReceiverView")
//             .child(
//                 Container::create()
//                    .parent_type(ParentType::Single)
//                     .child(SenderView::create()),
//             )
//             .property(Selector::default().id("receiver_view"))
//             .state(Rc::new(ReceiverState))
//     }
// }

// fn main() {
//     let mut application = Application::default();
//     application
//         .create_window()
//         .bounds(Bounds::new(100, 100, 420, 730))
//         .title("OrbTk - minimal example")
//         .root(ReceiverView::create())
//         .debug_flag(true)
//         .build();
//     application.run();
// }

fn main() {}
