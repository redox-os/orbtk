use std::rc::Rc;

use widget::WidgetContainer;
use event::Event;
use structs::{Rect, Point};

pub fn check_mouse_condition(
    position: Point,
    widget: &WidgetContainer,
) -> bool {
    if let Ok(bounds) = widget.borrow_property::<Rect>() {
        let mut rect = Rect::new(0, 0, bounds.width, bounds.height);

        if let Ok(g_pos) = widget.borrow_property::<Point>() {
            rect.x = g_pos.x;
            rect.y = g_pos.y;
        }

        return rect.contains(position)
    }

    false  
}

pub enum MouseButton {
    Left,
    Middle,
    Right,
}

pub struct MouseMouveEvent {
    pub position: Point,
}

impl Event for MouseMouveEvent {}

pub struct MouseUpEvent {
    pub button: MouseButton,
    pub position: Point,
}

impl Event for MouseUpEvent {}

pub struct MouseDownEvent {
    pub button: MouseButton,
    pub position: Point,
}

impl Event for MouseDownEvent {}

pub type MouseHandler = Rc<Fn(Point, &mut WidgetContainer) -> bool + 'static>;

pub type OnMouseUp = Rc<Fn() + 'static>;