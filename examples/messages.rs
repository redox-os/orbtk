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
//             .as_parent_type(ParentType::Single)
//             .with_debug_name("SenderView")
//             .with_child(
//                 Button::create()
//                     .with_property(Label::from("Send message"))
//                     .with_event_handler(MouseEventHandler::default().on_click(Rc::new(
//                         move |_pos: Point| -> bool {
//                             send_state.send_message.set(true);
//                             true
//                         },
//                     ))),
//             )
//             .with_state(state)
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
//             .as_parent_type(ParentType::Single)
//             .with_debug_name("ReceiverView")
//             .with_child(
//                 Container::create()
//                     .as_parent_type(ParentType::Single)
//                     .with_child(SenderView::create()),
//             )
//             .with_property(Selector::default().with_id("receiver_view"))
//             .with_state(Rc::new(ReceiverState))
//     }
// }

// fn main() {
//     let mut application = Application::default();
//     application
//         .create_window()
//         .with_bounds(Bounds::new(100, 100, 420, 730))
//         .with_title("OrbTk - minimal example")
//         .with_root(ReceiverView::create())
//         .with_debug_flag(true)
//         .build();
//     application.run();
// }

fn main() {
    
}