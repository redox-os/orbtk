use std::rc::Rc;

use widget::WidgetContainer;
use event::{Event, EventHandler, EventBox};
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

#[derive(Default)]
pub struct MouseEventHandler {
    pub on_mouse_up: Option<MouseHandler>,
    pub on_mouse_down: Option<MouseHandler>,
}

impl EventHandler for MouseEventHandler {
    fn handles_event(&self, event: &EventBox, widget: &WidgetContainer) -> bool {
        if let Some(_) = self.on_mouse_down {
            if let Ok(event) = event.downcast_ref::<MouseDownEvent>() {
                return check_mouse_condition(event.position, widget);
            }
        }

        if let Some(_) = self.on_mouse_up {
            if let Ok(event) = event.downcast_ref::<MouseUpEvent>() {
                return check_mouse_condition(event.position, widget);
            }
        }

        false
    }

    fn handle_event(&self, event: &EventBox, widget: &mut WidgetContainer) -> bool {
        if let Ok(event) = event.downcast_ref::<MouseDownEvent>() {
            if let Some(handler) = &self.on_mouse_down {
                return (handler)(event.position, widget);
            }
        }

        if let Ok(event) = event.downcast_ref::<MouseUpEvent>() {
            if let Some(handler) = &self.on_mouse_up {
                return (handler)(event.position, widget);
            }
        }

        false
    }
}