use orbtk::{
    prelude::*,
    shell::Key,
    widgets::themes::theme_orbtk::{colors, material_icons_font},
};

// constants
pub static ID_RECEIVER: &str = "receiver";
pub static ID_RECEIVER_STRING: &str = "Receiver Widget";
pub static ID_RECEIVER_CONTAINER: &str = "reciever_container";
pub static ID_RECEIVER_CONTAINER_MESSAGES: &str = "reciever_container_messages";
pub static ID_RECEIVER_CONTAINER_WIDGET: &str = "reciever_container_widget";
pub static ID_RECEIVER_COUNTER: &str = "receiver_message_counter";
pub static ID_RECEIVER_GRID: &str = "receiver_grid";
pub static ID_RECEIVER_HEADER: &str = "receiver_header";
pub static ID_RECEIVER_PROGRESS_BAR: &str = "receiver_progress_bar";
pub static ID_RECEIVER_TEXT_BLOCK: &str = "receiver_text_block";
pub static ID_RECEIVER_LABEL_COUNTER: &str = "receiver_label_message";
pub static ID_RECEIVER_LABEL_MESSAGE: &str = "receiver_label_message";
pub static ID_RECEIVER_MESSAGE_BLOCK: &str = "receiver_message_block";
pub static ID_RECEIVER_MESSAGE_COUNTER: &str = "receiver_message_counter";
pub static ID_RECEIVER_STACK_COUNTER: &str = "reciever_stack_counter";
pub static ID_RECEIVER_STACK_MESSAGE: &str = "reciever_stack_message";

pub static ID_SENDER: &str = "sender";
pub static ID_SENDER_STRING: &str = "Sender Widget";
pub static ID_SENDER_ACTION_BUTTON_SEND: &str = "sender_action_button_send";
pub static ID_SENDER_ACTION_BUTTON_CLEAR: &str = "sender_action_button_clear";
pub static ID_SENDER_CONTAINER: &str = "sender_container";
pub static ID_SENDER_CONTAINER_ACTION: &str = "sender_container_action";
pub static ID_SENDER_CONTAINER_WIDGET: &str = "sender_container_widget";
pub static ID_SENDER_COUNTER: &str = "sender_message_counter";
pub static ID_SENDER_GRID: &str = "sender_grid";
pub static ID_SENDER_HEADER: &str = "sender_header";
pub static ID_SENDER_STACK: &str = "sender_stack";
pub static ID_SENDER_STACK_ACTION: &str = "sender_stack_action";
pub static ID_SENDER_STACK_COUNTER: &str = "sender_stack_counter";
pub static ID_SENDER_LABEL_COUNTER: &str = "sender_label_message";
pub static ID_SENDER_MESSAGE: &str = "sender_message";
pub static ID_SENDER_TEXT_BOX: &str = "sender_text_box";

// [ReceiverView]

widget!(
    /// The receiver view is a form to present messages.
    ///
    /// The messages are send from the sender view, where
    /// the ReceiverView entity is the message_target.
    ///
    /// See also: [`message_target`](struct@SenderState)
    ReceiverView<ReceiverState> {
        /// Counter of received messages.
        counter_text: String,
        /// The Entity of the widget that will receive the messages.
        message_target: u32,
        /// The amount that will be incremented inside the targeted ProgressBar.
        progross_step: f64
    }
);

impl Template for ReceiverView {
    fn template(self, id: Entity, build_context: &mut BuildContext) -> Self {
        let receiver_header = TextBlock::new()
            .id(ID_RECEIVER_HEADER)
            .font_size(24)
            .text(ID_RECEIVER_STRING)
            .build(build_context);

        let receiver_progress_bar = ProgressBar::new()
            .id(ID_RECEIVER_PROGRESS_BAR)
            .build(build_context);

        let receiver_stack_counter = Container::new()
            .child(
                Stack::new()
                    .id(ID_RECEIVER_STACK_COUNTER)
                    .name(ID_RECEIVER_STACK_COUNTER)
                    .orientation("horizontal")
                    .spacing(5)
                    .h_align("end")
                    .child(
                        TextBlock::new()
                            .name(ID_RECEIVER_LABEL_COUNTER)
                            .text("Messages received:")
                            .build(build_context),
                    )
                    .child(
                        TextBlock::new()
                            .id(ID_RECEIVER_COUNTER)
                            .name(ID_RECEIVER_COUNTER)
                            .text(("counter_text", id))
                            .build(build_context),
                    )
                    .build(build_context),
            )
            .build(build_context);

        let receiver_container_msg = Container::new()
            .id(ID_RECEIVER_CONTAINER_MESSAGES)
            .child(
                Stack::new()
                    .id(ID_RECEIVER_STACK_MESSAGE)
                    .name(ID_RECEIVER_STACK_MESSAGE)
                    .orientation("horizontal")
                    .spacing(4)
                    .child(
                        TextBlock::new()
                            .id(ID_RECEIVER_LABEL_MESSAGE)
                            .name(ID_RECEIVER_LABEL_MESSAGE)
                            .text("Message:")
                            .build(build_context),
                    )
                    .child(
                        TextBlock::new()
                            .id(ID_RECEIVER_MESSAGE_BLOCK)
                            .name(ID_RECEIVER_MESSAGE_BLOCK)
                            .water_mark("Awaiting first message.")
                            .text("")
                            .build(build_context),
                    )
                    .build(build_context),
            )
            .build(build_context);

        self.id(ID_RECEIVER)
            .name(ID_RECEIVER_STRING)
            .counter_text("0")
            .child(
                Container::new()
                    .id(ID_RECEIVER_CONTAINER)
                    .padding(18)
                    .child(
                        Container::new()
                            .id(ID_RECEIVER_CONTAINER_WIDGET)
                            .border_brush(colors::BOMBAY_COLOR)
                            .border_width(2)
                            .padding(18)
                            .child(
                                Stack::new()
                                    .spacing(8)
                                    .child(receiver_header)
                                    .child(receiver_progress_bar)
                                    .child(receiver_stack_counter)
                                    .child(receiver_container_msg)
                                    .build(build_context),
                            )
                            .build(build_context),
                    )
                    .build(build_context),
            )
    }
}

// [ReceiverState]

/// The receiver state structure.
#[derive(Default, AsAny)]
struct ReceiverState {
    /// Entity id of the progress bar.
    progress_bar: Entity,
    /// The message counter.
    message_counter: i32,
    /// Entity id of the reveived message.
    message_block: Entity,
}

impl State for ReceiverState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.progress_bar = ctx
            .entity_of_child(ID_RECEIVER_PROGRESS_BAR)
            .expect("Cannot find 'ID_RECEIVER_PROGRESS_BAR'!");
        self.message_block = ctx
            .entity_of_child(ID_RECEIVER_MESSAGE_BLOCK)
            .expect("Cannot find 'ID_RECEIVER_MESSAGE_BLOCK'!");
    }

    fn messages(
        &mut self,
        mut messages: MessageReader,
        _registry: &mut Registry,
        ctx: &mut Context,
    ) {
        // Evaluate the messages we got from foreign state via `SenderAction`
        for message in messages.read::<SenderAction>() {
            match message {
                SenderAction::ClearMessage => {
                    TextBox::text_mut(&mut ctx.child(ID_RECEIVER_MESSAGE_BLOCK)).clear();
                }
                SenderAction::UpdateProgress(amount) => {
                    let progress_bar = ctx.get_widget(self.progress_bar);
                    let current_progress = progress_bar.clone::<f64>("val");
                    ProgressBar::val_set(
                        &mut ctx.child(ID_RECEIVER_PROGRESS_BAR),
                        current_progress + amount,
                    );
                    println!(
                        "Receiver message: Updating progress bar ({} percent)",
                        current_progress + amount
                    );

                    self.message_counter += 1;
                    println!(
                        "Receiver message: Updating received message counter ({} messages)",
                        &self.message_counter
                    );
                    TextBlock::text_set(
                        &mut ctx.child(ID_RECEIVER_COUNTER),
                        self.message_counter.to_string(),
                    );
                }
                SenderAction::UpdateMessage(text) => {
                    TextBlock::text_set(&mut ctx.child(ID_RECEIVER_MESSAGE_BLOCK), text);
                }
                _catch_all => {
                    println!("ReceiverView: message ignored!");
                }
            }
        }
    }
}

// [SenderView]

widget!(
    /// The sender view is a form capturing messages.
    SenderView<SenderState> {
        /// Counter of sended messages.
        counter_text: String,
        /// The Entity of the widget that will receive the messages.
        message_target: u32,
        /// The amount that will be incremented inside the targeted ProgressBar.
        progross_step: f64
    }
);

impl Template for SenderView {
    fn template(self, id: Entity, build_context: &mut BuildContext) -> Self {
        let sender_header = TextBlock::new()
            .id(ID_SENDER_HEADER)
            .font_size(24)
            .text(ID_SENDER_STRING)
            .build(build_context);

        let sender_message = TextBox::new()
            .id(ID_SENDER_TEXT_BOX)
            .name(ID_SENDER_TEXT_BOX)
            .text("")
            .water_mark("Please capture your message ...")
            .on_key_down(move |states, key_event| {
                if key_event.key == Key::Enter {
                    println!("KeyHandler: send action");
                    states.send_message(SenderAction::IncrementMessage, id);
                    states.send_message(
                        SenderAction::UpdateMessage("Default message!".to_string()),
                        id,
                    );
                    states.send_message(SenderAction::UpdateProgress(0.1), id);
                };
                true
            })
            // TODO: combined key handling (eg: Ctrl+s )
            .build(build_context);

        let sender_stack_counter = Stack::new()
            .id(ID_SENDER_STACK_COUNTER)
            .name(ID_SENDER_STACK_COUNTER)
            .orientation("horizontal")
            .spacing(5)
            .h_align("end")
            .child(
                TextBlock::new()
                    .name(ID_SENDER_LABEL_COUNTER)
                    .text("Messages send:")
                    .build(build_context),
            )
            .child(
                TextBlock::new()
                    .id(ID_SENDER_COUNTER)
                    .name(ID_SENDER_COUNTER)
                    .text(("counter_text", id))
                    .build(build_context),
            )
            .build(build_context);

        let sender_container_action = Container::new()
            .id(ID_SENDER_CONTAINER_ACTION)
            .h_align("center")
            .child(
                Stack::new()
                    .id(ID_SENDER_STACK_ACTION)
                    .orientation("horizontal")
                    .spacing(25)
                    .child(
                        Button::new()
                            .name(ID_SENDER_ACTION_BUTTON_CLEAR)
                            .font_size(8)
                            .text("Clear")
                            .icon(material_icons_font::MD_CLEAR)
                            .on_click(move |states, _entity| -> bool {
                                states.send_message(SenderAction::ClearMessage, id);
                                true
                            })
                            .build(build_context),
                    )
                    .child(
                        Button::new()
                            .name(ID_SENDER_ACTION_BUTTON_SEND)
                            .font_size(8)
                            .text("Send")
                            .icon(material_icons_font::MD_SEND)
                            .on_click(move |states, _entity| -> bool {
                                states.send_message(SenderAction::IncrementMessage, id);
                                states.send_message(
                                    SenderAction::UpdateMessage("Default message!".to_string()),
                                    id,
                                );
                                states.send_message(SenderAction::UpdateProgress(0.1), id);
                                true
                            })
                            .build(build_context),
                    )
                    .build(build_context),
            )
            .build(build_context);

        self.id(ID_SENDER).name(ID_SENDER).counter_text("0").child(
            Container::new()
                .id(ID_SENDER_CONTAINER)
                .padding(18)
                .child(
                    Container::new()
                        .id(ID_SENDER_CONTAINER_WIDGET)
                        .border_brush(colors::BOMBAY_COLOR)
                        .border_width(2)
                        .padding(14)
                        .child(
                            Stack::new()
                                .spacing(8)
                                .child(sender_header)
                                .child(sender_message)
                                .child(sender_stack_counter)
                                .child(sender_container_action)
                                .build(build_context),
                        )
                        .build(build_context),
                )
                .build(build_context),
        )
    }
}

// [SenderState]

/// Valid actions handled via [SenderState] messages
enum SenderAction {
    /// Clear the message TextBox.
    ClearMessage,
    /// Increment the message counter.
    IncrementMessage,
    /// The amount, that will be incremented inside a ProgressBar.
    UpdateProgress(f64),
    /// Update the target TextBlock with given message.
    UpdateMessage(String),
}

/// The sender state structure.
#[derive(Default, AsAny)]
struct SenderState {
    /// The message counter.
    message_counter: i32,
    /// Entity id of the message target.
    message_target: Entity,
}

impl State for SenderState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        self.message_target =
            Entity::from(ctx.widget().try_clone::<u32>("message_target").expect(
                "ERROR: SenderState::init(): expected message target entity id not found!",
            ));
    }

    fn messages(
        &mut self,
        mut messages: MessageReader,
        _registry: &mut Registry,
        ctx: &mut Context,
    ) {
        // Evaluate our own state messages
        for message in messages.read::<SenderAction>() {
            match message {
                SenderAction::ClearMessage => {
                    println!("Sender message: ClearMessage");
                    TextBox::text_mut(&mut ctx.child(ID_SENDER_TEXT_BOX)).clear();
                }
                SenderAction::IncrementMessage => {
                    println!("Sender message: increment message_counter");
                    self.message_counter += 1;
                    TextBlock::text_set(
                        &mut ctx.child(ID_SENDER_COUNTER),
                        self.message_counter.to_string(),
                    );
                    //SenderView::counter_text_set(&mut ctx.widget(), self.message_counter.to_string());
                }
                SenderAction::UpdateProgress(amount) => {
                    ctx.send_message(SenderAction::UpdateProgress(amount), self.message_target);
                    println!(
                        "Sender message: increment target ProgressBar ({} percent)",
                        amount
                    );
                }
                SenderAction::UpdateMessage(message) => {
                    // get text value from SenderView propery ID_SENDER_TEXT_BOX
                    let childs_message = ctx
                        .child(ID_SENDER_TEXT_BOX)
                        .get::<String>("text")
                        .to_string();
                    if childs_message.chars().count() > 0 {
                        println!("Sender message: {:?}", childs_message);
                        ctx.send_message(
                            SenderAction::UpdateMessage(childs_message),
                            self.message_target,
                        );
                    } else {
                        println!("Sender message: {:?}", message);
                        ctx.send_message(SenderAction::UpdateMessage(message), self.message_target);
                    }
                }
            }
        }
    }
}

/// The message handler window.
///
/// This OrbTK example application presents two views:
/// * The [SenderView]: The user will capture a message string. When
///   triggering the send button that message will be send to a target
///   entity, the ReceiverView. Every time a message is send,
///   SenderView will increment a counter to keep track of the amount
///   of sended messages. Beside the message string, a given progress
///   value will be transmitted as well (percent value).
/// * The [ReceiverView]: If a message is triggerd, the view will
///   presents a message area. A ProgressBar renders the received
///   stepping value.
pub fn main() {
    Application::from_name("orbtk_msg_handler");
    Application::new()
        .window(|ctx| {
            let receiver = ReceiverView::new().build(ctx);

            let sender = SenderView::new()
                // the entity of the target (receiver)
                .message_target(receiver.0)
                .build(ctx);

            Window::new()
                .title("OrbTK - Message Handler")
                .position((100.0, 100.0))
                .resizable(true)
                .size(450.0, 400.0)
                .child(
                    Stack::new()
                        .orientation("vertical")
                        .child(sender)
                        .child(receiver)
                        .build(ctx),
                )
                .build(ctx)
        })
        .run();
}
