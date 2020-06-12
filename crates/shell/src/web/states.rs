use std::{cell::RefCell, rc::Rc};

use stdweb::web::event;

/// Used to store and read web events.
pub struct EventState {
    pub mouse_move_events: Rc<RefCell<Vec<event::MouseMoveEvent>>>,
    pub mouse_up_events: Rc<RefCell<Vec<event::MouseUpEvent>>>,
    pub touch_start_events: Rc<RefCell<Vec<event::TouchStart>>>,
    pub touch_end_events: Rc<RefCell<Vec<event::TouchEnd>>>,
    pub touch_move_events: Rc<RefCell<Vec<event::TouchMove>>>,
    pub mouse_down_events: Rc<RefCell<Vec<event::MouseDownEvent>>>,
    pub scroll_events: Rc<RefCell<Vec<event::MouseWheelEvent>>>,
    pub key_up_events: Rc<RefCell<Vec<event::KeyUpEvent>>>,
    pub key_down_events: Rc<RefCell<Vec<event::KeyDownEvent>>>,
    pub resize_events: Rc<RefCell<Vec<event::ResizeEvent>>>,
}
