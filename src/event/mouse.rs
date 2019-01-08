use std::rc::Rc;

use crate::event::{Event, EventBox, EventHandler};
use crate::properties::{Point, Bounds};
use crate::widget::WidgetContainer;

pub fn check_mouse_condition(position: Point, widget: &WidgetContainer) -> bool {
    if let Ok(bounds) = widget.borrow_property::<Bounds>() {
        let mut rect = Bounds::new(0, 0, bounds.width, bounds.height);

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

pub struct ClickEvent {
    pub position: Point,
}

impl Event for ClickEvent {}

pub struct MouseDownEvent {
    pub button: MouseButton,
    pub position: Point,
}

impl Event for MouseDownEvent {}

pub type MouseHandler = Rc<Fn(Point) -> bool + 'static>;

pub type OnMouseUp = Rc<Fn() + 'static>;

#[derive(Default)]
pub struct MouseEventHandler {
    mouse_up: Option<MouseHandler>,
    mouse_down: Option<MouseHandler>,
    click: Option<MouseHandler>,
}

impl MouseEventHandler {
    pub fn on_mouse_up(mut self, handler: MouseHandler) -> Self {
        self.mouse_up = Some(handler);
        self
    } 

    pub fn on_mouse_down(mut self, handler: MouseHandler) -> Self {
        self.mouse_down = Some(handler);
        self
    } 

     pub fn on_click(mut self, handler: MouseHandler) -> Self {
        self.click = Some(handler);
        self
    } 
}

impl Into<Rc<EventHandler>> for MouseEventHandler {
    fn into(self) -> Rc<EventHandler> {
        Rc::new(self)
    }
}

impl EventHandler for MouseEventHandler {
    fn handle_event(&self, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<ClickEvent>() {
            if let Some(handler) = &self.click {
                return (handler)(event.position);
            }
        }

        if let Ok(event) = event.downcast_ref::<MouseDownEvent>() {
            if let Some(handler) = &self.mouse_down {
                return (handler)(event.position);
            }
        }

        if let Ok(event) = event.downcast_ref::<MouseUpEvent>() {
            if let Some(handler) = &self.mouse_up {
                (handler)(event.position);
                return true;
            }
        }

        false
    }
}
