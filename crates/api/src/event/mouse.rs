use std::rc::Rc;

use crate::{prelude::*, proc_macros::Event, shell::MouseButton, utils::*};

/// Checks if the given point is inside of a widget.
pub fn check_mouse_condition(mouse_position: Point, widget: &WidgetContainer<'_>) -> bool {
    let bounds = widget.get::<Rectangle>("bounds");
    let position = widget.get::<Point>("position");

    let mut rect = Rectangle::new(0.0, 0.0, bounds.width(), bounds.height());

    rect.set_x(position.x);
    rect.set_y(position.y);

    rect.contains((mouse_position.x, mouse_position.y))
}

#[derive(Event)]
pub struct MouseMoveEvent {
    pub x: f64,
    pub y: f64,
}

#[derive(Event)]
pub struct ScrollEvent {
    pub delta: Point,
}

#[derive(Event)]
pub struct MouseUpEvent {
    pub button: MouseButton,
    pub x: f64,

    pub y: f64,
}

#[derive(Event)]
pub struct ClickEvent {
    pub position: Point,
}

#[derive(Event)]
pub struct MouseDownEvent {
    pub button: MouseButton,
    pub x: f64,

    pub y: f64,
}

#[derive(Event)]
pub struct GlobalMouseUpEvent {
    pub button: MouseButton,
    pub x: f64,

    pub y: f64,
}

pub type MouseHandlerFunction = dyn Fn(&mut StatesContext, Point) -> bool + 'static;

pub type GlobalMouseHandlerFunction = dyn Fn(&mut StatesContext, Point) + 'static;

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
    fn handle_event(&self, state_context: &mut StatesContext, event: &EventBox) -> bool {
        event
            .downcast_ref::<ClickEvent>()
            .ok()
            .map_or(false, |event| (self.handler)(state_context, event.position))
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
    fn handle_event(&self, state_context: &mut StatesContext, event: &EventBox) -> bool {
        event
            .downcast_ref::<MouseDownEvent>()
            .ok()
            .map_or(false, |event| {
                (self.handler)(state_context, Point::new(event.x, event.y))
            })
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<MouseDownEvent>()
    }
}

/// Defines an event handler for a global mouse up event. Global mouse up events could not be handled.
pub struct GlobalMouseUpEventHandler {
    handler: Rc<GlobalMouseHandlerFunction>,
}

impl Into<Rc<dyn EventHandler>> for GlobalMouseUpEventHandler {
    fn into(self) -> Rc<dyn EventHandler> {
        Rc::new(self)
    }
}

impl EventHandler for GlobalMouseUpEventHandler {
    fn handle_event(&self, state_context: &mut StatesContext, event: &EventBox) -> bool {
        event
            .downcast_ref::<GlobalMouseUpEvent>()
            .ok()
            .map_or(false, |event| {
                (self.handler)(state_context, Point::new(event.x, event.y));
                false
            })
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<GlobalMouseUpEvent>()
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
    fn handle_event(&self, state_context: &mut StatesContext, event: &EventBox) -> bool {
        event
            .downcast_ref::<MouseUpEvent>()
            .ok()
            .map_or(false, |event| {
                (self.handler)(state_context, Point::new(event.x, event.y))
            })
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<MouseUpEvent>()
    }
}

/// Used to handle mouse down events. Could be attached to a widget.
pub struct MouseMoveEventHandler {
    handler: Rc<MouseHandlerFunction>,
}

impl Into<Rc<dyn EventHandler>> for MouseMoveEventHandler {
    fn into(self) -> Rc<dyn EventHandler> {
        Rc::new(self)
    }
}

impl EventHandler for MouseMoveEventHandler {
    fn handle_event(&self, state_context: &mut StatesContext, event: &EventBox) -> bool {
        event
            .downcast_ref::<MouseMoveEvent>()
            .ok()
            .map_or(false, |event| {
                (self.handler)(state_context, Point::new(event.x, event.y))
            })
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<MouseMoveEvent>()
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
    fn handle_event(&self, state_context: &mut StatesContext, event: &EventBox) -> bool {
        event
            .downcast_ref::<ScrollEvent>()
            .ok()
            .map_or(false, |event| {
                (self.handler)(state_context, Point::new(event.delta.x, event.delta.y))
            })
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<ScrollEvent>()
    }
}

pub trait MouseHandler: Sized + Widget {
    /// Inserts a click handler.
    fn on_click<H: Fn(&mut StatesContext, Point) -> bool + 'static>(self, handler: H) -> Self {
        self.insert_handler(ClickEventHandler {
            handler: Rc::new(handler),
        })
    }

    /// Insert a mouse down handler.
    fn on_mouse_down<H: Fn(&mut StatesContext, Point) -> bool + 'static>(self, handler: H) -> Self {
        self.insert_handler(MouseDownEventHandler {
            handler: Rc::new(handler),
        })
    }

    /// Insert a mouse up handler.
    fn on_mouse_up<H: Fn(&mut StatesContext, Point) -> bool + 'static>(self, handler: H) -> Self {
        self.insert_handler(MouseUpEventHandler {
            handler: Rc::new(handler),
        })
    }

    /// Insert a mouse handler for global up event.s
    fn on_global_mouse_up<H: Fn(&mut StatesContext, Point) + 'static>(self, handler: H) -> Self {
        self.insert_handler(GlobalMouseUpEventHandler {
            handler: Rc::new(handler),
        })
    }

    /// Insert a mouse move handler.
    fn on_mouse_move<H: Fn(&mut StatesContext, Point) -> bool + 'static>(self, handler: H) -> Self {
        self.insert_handler(MouseMoveEventHandler {
            handler: Rc::new(handler),
        })
    }

    /// Insert a mouse up handler.
    fn on_scroll<H: Fn(&mut StatesContext, Point) -> bool + 'static>(self, handler: H) -> Self {
        self.insert_handler(ScrollEventHandler {
            handler: Rc::new(handler),
        })
    }
}
