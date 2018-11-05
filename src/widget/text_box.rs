use std::rc::Rc;

use super::Property;
use event::Key;
use event::{EventHandler, MouseEventHandler, KeyEventHandler};
use structs::Point;
use theme::Selector;
use widget::{
    add_selector_to_widget, Container, HorizontalOffset, Label, PropertyResult, ScrollViewer,
    Template, TextBlock, Widget, WidgetContainer,
};

fn update_label(key: &Key, widget: &mut WidgetContainer) {
    let mut label_offset = 0;
    if let Ok(label) = widget.borrow_mut_parent_property::<Label>() {
        let old_label_width = label.0.len() * 8;

        if key.to_string() != "" {
            label.0.push_str(&key.to_string());
        } else {
            match *key {
                Key::Backspace => {
                    label.0.pop();
                }
                _ => {}
            }
        }

        label_offset = label.0.len() as i32 * 8 - old_label_width as i32;
    }

    if let Ok(horizontal_offset) = widget.borrow_mut_property::<HorizontalOffset>() {
        horizontal_offset.0 = (horizontal_offset.0 - label_offset).min(0);
    }
}

pub struct TextBox {
    pub label: Property<Label>,
    pub selector: Property<Selector>,
    pub event_handlers: Vec<Rc<EventHandler>>,
    pub horizontal_offset: Property<HorizontalOffset>,
}

impl Default for TextBox {
    fn default() -> TextBox {
        TextBox {
            label: Property::new(Label(String::from("TextBox"))),
            selector: Property::new(Selector::new(Some(String::from("textbox")))),
            horizontal_offset: Property::new(HorizontalOffset(0)),
            event_handlers: vec![],
        }
    }
}

impl Widget for TextBox {
    fn template(&self) -> Template {
        Template::Single(Rc::new(Container {
            selector: self.selector.clone(),
            child: Some(Rc::new(ScrollViewer {
                child: Some(Rc::new(TextBlock {
                    label: self.label.clone(),
                    selector: self.selector.clone(),
                })),
                horizontal_offset: self.horizontal_offset.clone(),
                ..Default::default()
            })),
            event_handlers: vec![Rc::new(MouseEventHandler {
                on_mouse_down: Some(Rc::new(
                    |_pos: Point, widget: &mut WidgetContainer| -> bool {
                        add_selector_to_widget("focus", widget);
                        false
                    },
                )),
                ..Default::default()
            }),
            Rc::new(KeyEventHandler {
                on_key_down: Some(Rc::new(
                    |key: &Key, widget: &mut WidgetContainer| -> bool {
                        update_label(key, widget);
                        true
                    },
                )),
                ..Default::default()
            })],
            ..Default::default()
        }))
    }

    fn properties(&self) -> Vec<PropertyResult> {
        vec![
            self.label.build(),
            self.selector.build(),
            self.horizontal_offset.build(),
        ]
    }

    fn event_handlers(&self) -> Vec<Rc<EventHandler>> {
        self.event_handlers.iter().by_ref().map(|handler| handler.clone()).collect()
    }
}
