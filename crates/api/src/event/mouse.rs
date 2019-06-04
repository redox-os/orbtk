use std::rc::Rc;

use crate::{prelude::*, shell::MouseButton, utils::*};

/// Checks if the given point is inside of a widget.
pub fn check_mouse_condition(mouse_position: Point, widget: &WidgetContainer<'_>) -> bool {
    let bounds = widget.get::<Bounds>();
    let position = widget.get::<Point>();

    let mut rect = Bounds::from(Rect::new(0.0, 0.0, bounds.width(), bounds.height()));

    rect.set_x(position.x);
    rect.set_y(position.y);

    return rect.contains((mouse_position.x, mouse_position.y));
}

pub struct MouseMoveEvent {
    pub position: Point,
}

impl Event for MouseMoveEvent {}

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

pub type MouseHandler = Fn(Point) -> bool + 'static;

/// Used to handle click events. Could be attached to a widget.
pub struct ClickEventHandler {
    handler: Rc<MouseHandler>,
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
}

// pub trait ClickHandler: Sized + From<Template> + Into<Template> {
//     /// Transforms the handler into a template.
//     fn template<F: FnOnce(Template) -> Template>(self, transform: F) -> Self {
//         Self::from(transform(self.into()))
//     }

//     /// Inserts a handler.
//     fn on_click<H: Fn(Point) -> bool + 'static>(self, handler: H) -> Self {
//         self.template(|template| {
//             template.event_handler(ClickEventHandler {
//                 handler: Rc::new(handler),
//             })
//         })
//     }
// }

pub trait ClickHandler: Sized + Widget {
    /// Inserts a handler.
    fn on_click<H: Fn(Point) -> bool + 'static>(self, handler: H) -> Self {
        self.insert_handler(ClickEventHandler {
            handler: Rc::new(handler),
        })
    }
}
