use std::rc::Rc;

use event::{EventHandler, Pressed, MouseOver};
use state::State;
use theme::Selector;
use widget::{
    add_selector_to_widget, remove_selector_from_widget, Center, Container, Label, Property,
    PropertyResult, Template, TextBlock, Widget, WidgetContainer,
};

/// The `ButtonState` handles the pressed state of the `Button` widget.
#[derive(Default)]
pub struct ButtonState;
impl State for ButtonState {
    fn update(&self, widget: &mut WidgetContainer) {
        let mut pressed = false;
        if let Ok(pres) = widget.borrow_mut_property::<Pressed>() {
            pressed = pres.0;
        }

        if pressed {
            add_selector_to_widget("active", widget);
        } else {
            remove_selector_from_widget("active", widget);
        }
    }
}

/// The `Button` struct represents a widget that can be clicked by user. It's used to peform an action.
pub struct Button {
    pub label: Property<Label>,
    pub selector: Property<Selector>,
    pub event_handlers: Vec<Rc<EventHandler>>,
    pub state: Rc<ButtonState>,
}

impl Default for Button {
    fn default() -> Button {
        Button {
            label: Property::new(Label(String::from("label"))),
            selector: Property::new(Selector::new(Some(String::from("button")))),
            event_handlers: vec![],
            state: Rc::new(ButtonState::default()),
        }
    }
}

impl Widget for Button {
    fn template(&self) -> Template {
        print!("Button -> ");
        Template::Single(Rc::new(Container {
            selector: self.selector.clone(),
            child: Some(Rc::new(Center {
                child: Some(Rc::new(TextBlock {
                    label: self.label.clone(),
                    selector: self.selector.clone(),
                })),
            })),
            ..Default::default()
        }))
    }

    fn properties(&self) -> Vec<PropertyResult> {
        vec![
            self.selector.build(),
            self.label.build(),
            Property::new(Pressed::default()).build(),
            Property::new(MouseOver::default()).build(),
        ]
    }

    fn state(&self) -> Option<Rc<State>> {
        Some(self.state.clone())
    }

    fn event_handlers(&self) -> Vec<Rc<EventHandler>> {
        self.event_handlers
            .iter()
            .by_ref()
            .map(|handler| handler.clone())
            .collect()
    }
}
