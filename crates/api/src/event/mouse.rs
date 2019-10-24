use std::rc::Rc;

use crate::{prelude::*, shell::MouseButton, utils::*};

/// Checks if the given point is inside of a widget.
pub fn check_mouse_condition(mouse_position: Point, widget: &WidgetContainer<'_>) -> bool {
    let bounds = widget.get::<Bounds>("bounds");
    let position = widget.get::<Pos>("position");

    let mut rect = Bounds::from(Rect::new(0.0, 0.0, bounds.width(), bounds.height()));

    rect.set_x(position.0.x);
    rect.set_y(position.0.y);

    return rect.contains((mouse_position.x, mouse_position.y));
}

pub struct MouseMoveEvent {
    pub position: Point,
}

impl Event for MouseMoveEvent {}

pub struct ScrollEvent {
    pub delta: Point,
}

impl Event for ScrollEvent {}

pub struct MouseUpEvent {
    pub button: MouseButton,
    pub x: f64,

    pub y: f64,
}

impl Event for MouseUpEvent {}

pub struct ClickEvent {
    pub position: Point,
}

impl Event for ClickEvent {}

pub struct MouseDownEvent {
    pub button: MouseButton,
    pub x: f64,

    pub y: f64,
}

impl Event for MouseDownEvent {}

pub type MouseHandlerFunction = dyn Fn(Point) -> bool + 'static;

/// Used to handle click events. Could be attached to a widget.
pub struct ClickEventHandler {
    handler: Rc<MouseHandlerFunction>,
}

impl Into<Rc<dyn EventHandler>> for ClickEventHandler {
    fn into(self) -> Rc<dyn EventHandler> {
        Rc::new(self)
    }
}

impl EventHandler for ClickEventHandler {
    fn handle_event(&self, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<ClickEvent>() {
            return (self.handler)(event.position);
        }

        return false;
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<ClickEvent>()
    }
}

/// Used to handle mouse down events. Could be attached to a widget.
pub struct MouseDownEventHandler {
    handler: Rc<MouseHandlerFunction>,
}

impl Into<Rc<dyn EventHandler>> for MouseDownEventHandler {
    fn into(self) -> Rc<dyn EventHandler> {
        Rc::new(self)
    }
}

impl EventHandler for MouseDownEventHandler {
    fn handle_event(&self, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<MouseDownEvent>() {
            return (self.handler)(Point::new(event.x, event.y));
        }

        return false;
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<MouseDownEvent>()
    }
}

/// Used to handle mouse down events. Could be attached to a widget.
pub struct MouseUpEventHandler {
    handler: Rc<MouseHandlerFunction>,
}

impl Into<Rc<dyn EventHandler>> for MouseUpEventHandler {
    fn into(self) -> Rc<dyn EventHandler> {
        Rc::new(self)
    }
}

impl EventHandler for MouseUpEventHandler {
    fn handle_event(&self, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<MouseUpEvent>() {
            return (self.handler)(Point::new(event.x, event.y));
        }

        return false;
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<MouseUpEvent>()
    }
}

pub struct ScrollEventHandler {
    handler: Rc<MouseHandlerFunction>,
}

impl Into<Rc<dyn EventHandler>> for ScrollEventHandler {
    fn into(self) -> Rc<dyn EventHandler> {
        Rc::new(self)
    }
}

impl EventHandler for ScrollEventHandler {
    fn handle_event(&self, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<ScrollEvent>() {
            return (self.handler)(Point::new(event.delta.x, event.delta.y));
        }

        return false;
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<ScrollEvent>()
    }
}

pub trait MouseHandler: Sized + Widget {
    /// Inserts a click handler.
    fn on_click<H: Fn(Point) -> bool + 'static>(self, handler: H) -> Self {
        self.insert_handler(ClickEventHandler {
            handler: Rc::new(handler),
        })
    }

    /// Insert a mouse down handler.
    fn on_mouse_down<H: Fn(Point) -> bool + 'static>(self, handler: H) -> Self {
        self.insert_handler(MouseDownEventHandler {
            handler: Rc::new(handler),
        })
    }

    /// Insert a mouse up handler.
    fn on_mouse_up<H: Fn(Point) -> bool + 'static>(self, handler: H) -> Self {
        self.insert_handler(MouseUpEventHandler {
            handler: Rc::new(handler),
        })
    }

    /// Insert a mouse up handler.
    fn on_scroll<H: Fn(Point) -> bool + 'static>(self, handler: H) -> Self {
        self.insert_handler(ScrollEventHandler {
            handler: Rc::new(handler),
        })
    }
}
