use std::any::TypeId;
use std::rc::Rc;

use event::{check_mouse_condition, EventBox, MouseDownEvent, MouseHandler, MouseUpEvent};
use state::State;
use theme::Selector;
use widget::{
    add_selector_to_widget, remove_selector_from_widget, Container, Label, Property,
    PropertyResult, Template, TextBlock, Widget, WidgetContainer,
};

#[derive(Copy, Clone)]
pub struct Pressed(pub bool);

#[derive(Default)]
pub struct ButtonState {
    pub on_mouse_down: Option<MouseHandler>,
    pub on_mouse_up: Option<MouseHandler>,
}

impl State for ButtonState {
    fn handles_event(&self, event: &EventBox, widget: &WidgetContainer) -> bool {
        if let Ok(event) = event.downcast_ref::<MouseDownEvent>() {
            return check_mouse_condition(event.position, widget);
        }

        if event.event_type() == TypeId::of::<MouseUpEvent>() {
            return widget.borrow_property::<Pressed>().unwrap().0;
        }

        false
    }

    fn update(&self, event: &EventBox, widget: &mut WidgetContainer) -> bool {
        if event.event_type() == TypeId::of::<MouseDownEvent>() {
            add_selector_to_widget("active", widget);
            widget.borrow_mut_property::<Pressed>().unwrap().0 = true;
            if let Some(handler) = &self.on_mouse_down {
                (handler)();
            }

            return true;
        }

        if let Ok(event) = event.downcast_ref::<MouseUpEvent>() {
            remove_selector_from_widget("active", widget);
            widget.borrow_mut_property::<Pressed>().unwrap().0 = false;

            if check_mouse_condition(event.position, widget) {
                if let Some(handler) = &self.on_mouse_up {
                    (handler)();
                }
            }

            return true;
        }

        false
    }

    fn properties(&self) -> Vec<PropertyResult> {
        vec![Property::new(Pressed(false)).build()]
    }
}

pub struct Button {
    pub label: Property<Label>,
    pub selector: Property<Selector>,
    pub state: Rc<State>,
}

impl Default for Button {
    fn default() -> Button {
        Button {
            label: Property::new(Label(String::from("label"))),
            selector: Property::new(Selector::new(Some(String::from("button")))),
            state: Rc::new(ButtonState {
                ..Default::default()
            }),
        }
    }
}

impl Widget for Button {
    fn template(&self) -> Template {
        Template::Single(Rc::new(Container {
            selector: self.selector.clone(),
            child: Some(Rc::new(TextBlock {
                label: self.label.clone(),
                selector: self.selector.clone(),
            })),
        }))
    }

    fn properties(&self) -> Vec<PropertyResult> {
        vec![self.selector.build(), self.label.build()]
    }

    fn state(&self) -> Option<Rc<State>> {
        Some(self.state.clone())
    }
}
