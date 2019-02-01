use std::rc::Rc;

use crate::{
    event::{Event, EventBox, EventHandler},
    properties::Bounds,
    structs::{Point, Position, Size},
    widget::WidgetContainer,
};

pub fn check_mouse_condition(position: Point, widget: &WidgetContainer<'_>) -> bool {
    if let Ok(bounds) = widget.borrow_property::<Bounds>() {
        let mut rect = Bounds::new(0.0, 0.0, bounds.width(), bounds.height());

        if let Ok(g_pos) = widget.borrow_property::<Point>() {
            rect.set_x(g_pos.x);
            rect.set_y(g_pos.y);
        }

        return rect.contains((position.x, position.y));
    }

    false
}

pub enum MouseButton {
    Left,
    Middle,
    Right,
}

pub struct MouseMoveEvent {
    pub position: Point,
}

impl Event for MouseMoveEvent {}

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

pub type MouseHandler = Fn(Point) -> bool + 'static;

use crate::Template;

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

pub trait ClickHandler: Sized + From<Template> + Into<Template> {
    /// Transforms the handler into a template.
    fn template<F: FnOnce(Template) -> Template>(self, transform: F) -> Self {
        Self::from(transform(self.into()))
    }

    /// Inserts a handler.
    fn on_click<H: Fn(Point) -> bool + 'static>(self, handler: H) -> Self {
        self.template(|template| {
            template.event_handler(ClickEventHandler {
                handler: Rc::new(handler),
            })
        })
    }
}
