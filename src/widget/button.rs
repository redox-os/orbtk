use theme::Selector;
use std::rc::Rc;
use {Container, PressedState, EventHandler, MouseUpHandler, ReleaseState, OnMouseDown, MouseDownHandler, Property, Template, TextBlock, Widget};

pub struct Button {
    pub label: String,
    pub class: String,
    pub on_mouse_down: OnMouseDown,
}

impl Default for Button {
    fn default() -> Button {
        Button {
            label: String::from("Button"),
            class: String::from("button"),
            on_mouse_down: Rc::new(|| {}),
        }
    }
}

impl Widget for Button {
    fn template(&self) -> Template {
        Template::Single(Rc::new(Container {
            class: self.class.clone(),
            child: Some(Rc::new(TextBlock {
                label: self.label.clone(),
                class: self.class.clone(),
            })),
        }))
    }

    fn properties(&self) -> Vec<Property> {
        vec![Property::new(Selector::new(Some(self.class.clone())))]
    }

    fn event_handlers(&self) -> Vec<Rc<EventHandler>> {
        vec![
            MouseDownHandler::new(Box::new(PressedState {}), self.on_mouse_down.clone()),
            MouseUpHandler::new(Box::new(ReleaseState {}), Rc::new(||{})),
        ]
    }
}

