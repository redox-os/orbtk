use event::{Focused, Key};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use theme::Selector;
use widget::{
    add_selector_to_widget, remove_selector_from_widget, Container, Label, ScrollViewer, Stack,
    State, Template, TextBlock, Widget, WidgetContainer,
};

/// The `TextBoxState`handles the text processing of the `TextBox` widget.
#[derive(Default)]
pub struct TextBoxState {
    text: RefCell<String>,
    focused: Cell<bool>,
    updated: Cell<bool>,
}

impl Into<Rc<State>> for TextBoxState {
    fn into(self) -> Rc<State> {
        Rc::new(self)
    }
}

impl TextBoxState {
    fn update_text(&self, key: Key) -> bool {
        if !self.focused.get() {
            return false;
        }

        match <Option<u8>>::from(key) {
            Some(byte) => {
                (*self.text.borrow_mut()).push(byte as char);
            }
            None => match key {
                Key::Backspace => {
                    (*self.text.borrow_mut()).pop();
                }
                _ => {}
            },
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

/// The `TextBoxState`handles the text processing of the `TextBox` widget.
pub struct TextBox;

impl Widget for TextBox {
    fn template() -> Template {
        print!("TextBox -> ");
        Template::default()
            .with_property(Label::from("TextBox"))
            .with_property(Selector::new().with("textbox"))
            .with_property(Focused(false))
            .with_child(
                Container::template().with_child(
                    Stack::template()
                        .with_child(ScrollViewer::template().with_child(TextBlock::template())),
                ),
            )
            .with_state(TextBoxState::default())
        // .with_event_handler(KeyEventHandler::default().on_key_down(Rc::new(
        //     move |key: Key, _widget: &mut WidgetContainer| -> bool { state.update_text(key) },
        // )))
    }
}
