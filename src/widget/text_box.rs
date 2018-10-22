use std::any::TypeId;
use std::rc::Rc;

use super::Property;
use event::{check_mouse_condition, EventBox, Key, KeyDownEvent, MouseDownEvent};
use state::State;
use theme::Selector;
use widget::{
    add_selector_to_widget, Container, Label, PropertyResult, ScrollViewer, Template, TextBlock,
    HorizontalOffset, Widget, WidgetContainer,
};

// todo: cursor struct with position and selection length
#[derive(Clone, Copy)]
pub struct Focused(pub bool);

#[derive(Clone, Copy, Default)]
pub struct TextCursor {
    pub position: u32,
    pub start: Option<u32>,
    pub end: Option<u32>,
}

#[derive(Default)]
pub struct TextBoxState {}

impl State for TextBoxState {
    fn handles_event(&self, event: &EventBox, widget: &WidgetContainer) -> bool {
        if let Ok(event) = event.downcast_ref::<MouseDownEvent>() {
            let mut focused = false;
            if let Ok(foc) = widget.borrow_property::<Focused>() {
                focused = foc.0;
            }

            return !focused && check_mouse_condition(event.position, widget);
        }

        if event.event_type() == TypeId::of::<KeyDownEvent>() {
            if let Ok(foc) = widget.borrow_property::<Focused>() {
                return foc.0;
            }
        }

        false
    }

    fn update(&self, event: &EventBox, widget: &mut WidgetContainer) -> bool {
        fn update_label(key: &Key, widget: &mut WidgetContainer) {
            let mut label_offset = 0;
            if let Ok(label) = widget.borrow_mut_property::<Label>() {
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

        if event.event_type() == TypeId::of::<MouseDownEvent>() {
            add_selector_to_widget("active", widget);
            widget.borrow_mut_property::<Focused>().unwrap().0 = true;

            return true;
        }

        if let Ok(key_event) = event.downcast_ref::<KeyDownEvent>() {
            update_label(&key_event.key, widget);
            return true;
        }

        false
    }

    fn properties(&self) -> Vec<PropertyResult> {
        vec![
            Property::new(Focused(false)).build(),
            Property::new(TextCursor::default()).build(),
        ]
    }
}

pub struct TextBox {
    pub label: Property<Label>,
    pub selector: Property<Selector>,
    pub state: Rc<State>,
    pub horizontal_offset: Property<HorizontalOffset>,
}

impl Default for TextBox {
    fn default() -> TextBox {
        TextBox {
            label: Property::new(Label(String::from("TextBox"))),
            selector: Property::new(Selector::new(Some(String::from("textbox")))),
            horizontal_offset: Property::new(HorizontalOffset(0)),
            state: Rc::new(TextBoxState {
                ..Default::default()
            }),
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
        }))
    }

    fn properties(&self) -> Vec<PropertyResult> {
        vec![
            self.label.build(),
            self.selector.build(),
            self.horizontal_offset.build(),
        ]
    }

    fn state(&self) -> Option<Rc<State>> {
        Some(self.state.clone())
    }
}
