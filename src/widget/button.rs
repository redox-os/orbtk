use std::rc::Rc;

use event::Handler;
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
    pub handler: Rc<Handler>,
}

impl Default for Button {
    fn default() -> Button {
        Button {
            label: Property::new(Label(String::from("label"))),
            selector: Property::new(Selector::new(Some(String::from("button")))),
            handler: Rc::new(Handler::default()),
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
            handler: Rc::new(Handler {
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

    fn handler(&self) -> Option<Rc<Handler>> {
        Some(self.handler.clone())
    }
}
