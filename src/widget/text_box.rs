use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::Property;
use event::{EventHandler, KeyEventHandler};
use event::{Focused, Key};
use theme::Selector;
use widget::{
    add_selector_to_widget, remove_selector_from_widget, Container, Label, PropertyResult,
    ScrollViewer, Stack, State, Template, TextBlock, Widget, WidgetContainer,
};

/// The `TextBoxState`handles the text processing of the `TextBox` widget.
#[derive(Default)]
pub struct TextBoxState {
    text: RefCell<String>,
    focused: Cell<bool>,
    updated: Cell<bool>,
}

impl TextBoxState {
    fn update_text(&self, key: &Key) -> bool {
        if !self.focused.get() {
            return false;
        }

        if key.to_string() != "" {
            (*self.text.borrow_mut()).push_str(&key.to_string());
        } else {
            match key {
                Key::Backspace => {
                    (*self.text.borrow_mut()).pop();
                }
                _ => {}
            }
        }

        self.updated.set(true);

        true
    }
}

impl State for TextBoxState {
    fn update(&self, widget: &mut WidgetContainer) {
        if let Ok(focused) = widget.borrow_property::<Focused>() {
            self.focused.set(focused.0);
        }

        if self.focused.get() {
            add_selector_to_widget("focus", widget);
        } else {
            remove_selector_from_widget("focus", widget);
        }

        if let Ok(label) = widget.borrow_mut_property::<Label>() {
            if label.0 == *self.text.borrow() {
                return;
            }

            if self.updated.get() {
                label.0 = self.text.borrow().clone();
            } else {
                *self.text.borrow_mut() = label.0.clone();
            }

            self.updated.set(false);
        }
    }
}

/// A single line text input widget.
pub struct TextBox {
    pub label: Property<Label>,
    pub selector: Property<Selector>,
    pub event_handlers: Vec<Rc<EventHandler>>,
    pub state: Rc<TextBoxState>,
}

impl Default for TextBox {
    fn default() -> TextBox {
        TextBox {
            label: Property::new(Label(String::from("TextBox"))),
            selector: Property::new(Selector::new(Some(String::from("textbox")))),
            event_handlers: vec![],
            state: Rc::new(TextBoxState::default()),
        }
    }
}

impl Widget for TextBox {
    fn template(&self) -> Template {
        print!("TextBox -> ");

        // Initial set state text to textbox label.
        if let Some(text) = &self.label.property {
            *self.state.text.borrow_mut() = text.0.clone();
        }

        Template::Single(Rc::new(Container {
            selector: self.selector.clone(),
            child: Some(Rc::new(Stack {
                children: vec![
                    Rc::new(ScrollViewer {
                        child: Some(Rc::new(TextBlock {
                            label: self.label.clone(),
                            selector: self.selector.clone(),
                        })),
                        ..Default::default()
                    }),
                    // todo: Cursor as rectangle -> predifined bounds
                    // scroll handling by cursor
                    // Rc::new(TextBlock {
                    //     ..Default::default()
                    // }),
                ],
            })),
            ..Default::default()
        }))
    }

    fn properties(&self) -> Vec<PropertyResult> {
        vec![
            self.label.build(),
            self.selector.build(),
            Property::new(Focused(false)).build(),
        ]
    }

    fn state(&self) -> Option<Rc<State>> {
        Some(self.state.clone())
    }

    fn event_handlers(&self) -> Vec<Rc<EventHandler>> {
        let state = self.state.clone();

        let mut event_handlers: Vec<Rc<EventHandler>> = vec![Rc::new(KeyEventHandler {
            on_key_down: Some(Rc::new(
                move |key: &Key, _widget: &mut WidgetContainer| -> bool { state.update_text(key) },
            )),
            ..Default::default()
        })];

        for handler in &self.event_handlers {
            event_handlers.push(handler.clone());
        }

        event_handlers
    }
}
