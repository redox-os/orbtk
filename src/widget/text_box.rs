use std::cell::Cell;
use std::rc::Rc;

use super::Property;
use event::{EventHandler, KeyEventHandler};
use event::{Focused, Key};
use theme::Selector;
use widget::{
    add_selector_to_widget, remove_selector_from_widget, Container, HorizontalOffset, Label,
    PropertyResult, ScrollViewer, State, Template, TextBlock, Widget, WidgetContainer,
};

pub struct TextBoxState {
    input_key: Cell<Option<Key>>,
}

impl TextBoxState {
    fn update_label(&self, key: &Key) {
        self.input_key.set(Some(key.clone()));
    }
}

impl State for TextBoxState {
    fn update(&self, widget: &mut WidgetContainer) {
        let mut focused = false;
        if let Ok(focu) = widget.borrow_mut_property::<Focused>() {
            focused = focu.0;
        }

        if focused {
            add_selector_to_widget("focus", widget);
        } else {
            remove_selector_from_widget("focus", widget);
        }

        if let Some(key) = self.input_key.get() {
            let mut label_offset = 0;
            if let Ok(label) = widget.borrow_mut_property::<Label>() {
                let old_label_width = label.0.len() * 8;

                if key.to_string() != "" {
                    label.0.push_str(&key.to_string());
                } else {
                    match key {
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
    }
}

pub struct TextBox {
    pub label: Property<Label>,
    pub selector: Property<Selector>,
    pub event_handlers: Vec<Rc<EventHandler>>,
    pub horizontal_offset: Property<HorizontalOffset>,
    pub state: Rc<TextBoxState>,
}

impl Default for TextBox {
    fn default() -> TextBox {
        TextBox {
            label: Property::new(Label(String::from("TextBox"))),
            selector: Property::new(Selector::new(Some(String::from("textbox")))),
            horizontal_offset: Property::new(HorizontalOffset(0)),
            event_handlers: vec![],
            state: Rc::new(TextBoxState {
                input_key: Cell::new(None),
            }),
        }
    }
}

impl Widget for TextBox {
    fn template(&self) -> Template {
        print!("TextBox -> ");

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
            ..Default::default()
        }))
    }

    fn properties(&self) -> Vec<PropertyResult> {
        vec![
            self.label.build(),
            self.selector.build(),
            self.horizontal_offset.build(),
            Property::new(Focused(false)).build(),
        ]
    }

    fn state(&self) -> Option<Rc<State>> {
        Some(self.state.clone())
    }

    fn event_handlers(&self) -> Vec<Rc<EventHandler>> {
        let state = self.state.clone();
        vec![Rc::new(KeyEventHandler {
            on_key_down: Some(Rc::new(
                move |key: &Key, widget: &mut WidgetContainer| -> bool {
                    if let Ok(focused) = widget.borrow_mut_property::<Focused>() {
                        if !focused.0 {
                            return false;
                        }
                    }
                    state.update_label(key);
                    true
                },
            )),
            ..Default::default()
        })]
    }
}
