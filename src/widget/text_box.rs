use enums::ParentType;
use event::{Focused, Key, KeyEventHandler};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use structs::Label;
use theme::Selector;
use widget::{
    add_selector_to_widget, remove_selector_from_widget, Container, ScrollViewer, SharedProperty,
    Stack, State, Template, TextBlock, Widget, WidgetContainer,
};

/// The `TextBoxState` handles the text processing of the `TextBox` widget.
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
    fn create() -> Template {
        let label = SharedProperty::new(Label::from("TextBox"));
        let selector = SharedProperty::new(Selector::new().with("textbox"));
        let state = Rc::new(TextBoxState::default());

        Template::default()
            .as_parent_type(ParentType::Single)
            .with_property(Focused(false))
            .with_child(
                Container::create()
                    .with_child(
                        Stack::create().with_child(
                            ScrollViewer::create().with_child(
                                TextBlock::create()
                                    .with_shared_property(label.clone())
                                    .with_shared_property(selector.clone()),
                            ),
                        ),
                    )
                    .with_shared_property(selector.clone()),
            )
            .with_state(state.clone())
            .with_debug_name("TextBox")
            .with_shared_property(label)
            .with_shared_property(selector)
            .with_event_handler(KeyEventHandler::default().on_key_down(Rc::new(
                move |key: Key, _widget: &mut WidgetContainer| -> bool { state.update_text(key) },
            )))
    }
}
