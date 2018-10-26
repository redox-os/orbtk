use std::rc::Rc;

use state::State;
use structs::Point;
use theme::Selector;
use widget::{
    add_selector_to_widget, remove_selector_from_widget, Container, Label, Property,
    PropertyResult, Template, TextBlock, Widget, WidgetContainer,
};

#[derive(Copy, Clone)]
pub struct Pressed(pub bool);

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
            state: Rc::new(State::default()),
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
            state: Rc::new(State {
                on_mouse_down: Some(Rc::new(|_pos: Point, widget: &mut WidgetContainer|  -> bool {
                    add_selector_to_widget("active", widget);
                    false
                })),
                on_mouse_up: Some(Rc::new(|_pos: Point, widget: &mut WidgetContainer|  -> bool {
                    remove_selector_from_widget("active", widget);
                    false
                })),
                ..Default::default()
            }),
        }))
    }

    fn properties(&self) -> Vec<PropertyResult> {
        vec![self.selector.build(), self.label.build()]
    }

    fn state(&self) -> Option<Rc<State>> {
        Some(self.state.clone())
    }
}
