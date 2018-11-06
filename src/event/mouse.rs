use std::rc::Rc;

use event::{Event, EventBox, EventHandler};
use structs::{Point, Rect};
use widget::WidgetContainer;

#[derive(Default, Copy, Clone)]
pub struct Pressed(pub bool);

#[derive(Default, Copy, Clone)]
pub struct Focused(pub bool);

pub fn check_mouse_condition(position: Point, widget: &WidgetContainer) -> bool {
    if let Ok(bounds) = widget.borrow_property::<Rect>() {
        let mut rect = Rect::new(0, 0, bounds.width, bounds.height);

        if let Ok(g_pos) = widget.borrow_property::<Point>() {
            rect.x = g_pos.x;
            rect.y = g_pos.y;
        }

        return rect.contains(position);
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

#[derive(Default)]
pub struct MouseEventHandler {
    pub on_mouse_up: Option<MouseHandler>,
    pub on_mouse_down: Option<MouseHandler>,
}

impl EventHandler for MouseEventHandler {
    fn handle_event(&self, event: &EventBox, widget: &mut WidgetContainer) -> bool {
        if let Ok(event) = event.downcast_ref::<MouseDownEvent>() {
            if let Some(handler) = &self.on_mouse_down {
                return (handler)(event.position, widget);
            }
        }

        if let Ok(event) = event.downcast_ref::<MouseUpEvent>() {
            if let Some(handler) = &self.on_mouse_up {
                (handler)(event.position, widget);
                return true;
            }
        }

        false
    }
}
