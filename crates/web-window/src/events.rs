use stdweb::web::event::*;

/// Defines web mouse events.
pub enum Mouse {
    Up(MouseUpEvent),
    Down(MouseDownEvent),
    Move(MouseMoveEvent)
}

/// Defines web key events.
pub enum Key {
    Up(KeyUpEvent),
    Down(KeyDownEvent)
}

/// Defines a web window event.
pub enum Event {
    Mouse(Mouse),
    Key(Key)
}

