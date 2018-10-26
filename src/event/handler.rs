use std::any::TypeId;

use event::{
    check_mouse_condition, EventBox, KeyDownEvent, KeyHandler, KeyUpEvent, MouseDownEvent,
    MouseHandler, MouseUpEvent,
};
use theme::Selector;
use widget::WidgetContainer;

// todo: Add properties to Handler and handle custom events maybe by trait object

#[derive(Default)]
pub struct Handler {
    pub on_mouse_up: Option<MouseHandler>,
    pub on_mouse_down: Option<MouseHandler>,
    pub on_key_up: Option<KeyHandler>,
    pub on_key_down: Option<KeyHandler>,
}

impl Handler {
    pub fn handles_event(&self, event: &EventBox, widget: &WidgetContainer) -> bool {
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

        if let Some(_) = self.on_key_down {
            if event.event_type() == TypeId::of::<KeyDownEvent>() {
                let mut is_focused = false;

                if let Ok(selector) = widget.borrow_property::<Selector>() {
                    if selector.pseudo_classes.contains("focus") {
                        is_focused = true;
                    }
                }

                return is_focused;
            }
        }

        if let Some(_) = self.on_key_up {
            if event.event_type() == TypeId::of::<KeyUpEvent>() {
                let mut is_focused = false;

                if let Ok(selector) = widget.borrow_property::<Selector>() {
                    if selector.pseudo_classes.contains("focus") {
                        is_focused = true;
                    }
                }

                return is_focused;
            }
        }

        false
    }

    pub fn handle_event(&self, event: &EventBox, widget: &mut WidgetContainer) -> bool {
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

        if let Ok(event) = event.downcast_ref::<KeyDownEvent>() {
            if let Some(handler) = &self.on_key_down {
                return (handler)(&event.key, widget);
            }
        }

        if let Ok(event) = event.downcast_ref::<KeyUpEvent>() {
            if let Some(handler) = &self.on_key_up {
                return (handler)(&event.key, widget);
            }
        }

        false
    }
}
