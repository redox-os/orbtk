use enums::ParentType;
use event::{Key, KeyEventHandler, MouseEventHandler};
use properties::{Focused, Label, Offset, Point, TextSelection, WaterMark};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use theme::Selector;
use widget::{
    Container, Cursor, ScrollViewer, SharedProperty, Stack, State, Template, WaterMarkTextBlock,
    Widget, WidgetContainer,
};

/// The `TextBoxState` handles the text processing of the `TextBox` widget.
#[derive(Default)]
pub struct TextBoxState {
    text: RefCell<String>,
    focused: Cell<bool>,
    updated: Cell<bool>,
    selection_start: Cell<usize>,
    selection_end: Cell<usize>,
}

impl Into<Rc<State>> for TextBoxState {
    fn into(self) -> Rc<State> {
        Rc::new(self)
    }
}

impl TextBoxState {
    fn click(&self, point: Point) {
        println!("Clicked text box point: ({}, {})", point.x, point.y);
    }

    fn update_selection_start(&self, selection: i32) {
        self.selection_start
            .set(selection.max(0).min(self.text.borrow().len() as i32) as usize);
    }

    fn update_text(&self, key: Key) -> bool {
        if !self.focused.get() {
            return false;
        }

        match <Option<u8>>::from(key) {
            Some(byte) => {
                (*self.text.borrow_mut()).push(byte as char);
                self.update_selection_start(self.selection_start.get() as i32 + 1);
            }
            None => match key {
                Key::Backspace => {
                    (*self.text.borrow_mut()).pop();
                    self.update_selection_start(self.selection_start.get() as i32 - 1);
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

        if let Ok(selection) = widget.borrow_mut_property::<TextSelection>() {
            selection.start_index = self.selection_start.get();
            selection.end_index = self.selection_end.get();
        }
    }
}

/// The `TextBox` represents a single line text input widget.
///
/// # Shared Properties
///
/// * `Label` - String used to display the text of the text box.
/// * `Watermark` - String used to display a placeholder text if `Label` string is empty.
/// * `Selector` - CSS selector with  element name `textbox`, used to request the theme of the widget.
/// * `TextSelection` - Represents the current selection of the text used by the cursor.
/// * `Focused` - Defines if the widget is focues and handles the current text input.
/// 
/// # Others
///
/// * `TextBoxState` - Handles the inner state of the widget.
/// * `KeyEventHandler` - Process the text input of the control if it is focuesd.
pub struct TextBox;

impl Widget for TextBox {
    fn create() -> Template {
        let label = SharedProperty::new(Label::default());
        let water_mark = SharedProperty::new(WaterMark::default());
        let selector = SharedProperty::new(Selector::from("textbox"));
        let selection = SharedProperty::new(TextSelection::default());
        let offset = SharedProperty::new(Offset::default());
        let focused = SharedProperty::new(Focused(false));
        let state = Rc::new(TextBoxState::default());
        let click_state = state.clone();

        Template::default()
            .as_parent_type(ParentType::Single)
            .with_property(Focused(false))
            .with_child(
                Container::create()
                    .with_child(
                        Stack::create()
                            .with_child(
                                ScrollViewer::create()
                                    .with_child(
                                        WaterMarkTextBlock::create()
                                            .with_shared_property(label.clone())
                                            .with_shared_property(selector.clone())
                                            .with_shared_property(water_mark.clone()),
                                    )
                                    .with_shared_property(offset.clone()),
                            )
                            .with_child(
                                Cursor::create()
                                    .with_shared_property(label.clone())
                                    .with_shared_property(selection.clone())
                                    .with_shared_property(offset.clone())
                                    .with_shared_property(focused.clone())
                            )
                            .with_event_handler(MouseEventHandler::default().on_mouse_down(Rc::new(
                                move |pos: Point| -> bool {
                                    click_state.click(pos);
                                    false
                                },
                            ))),
                    )
                    .with_shared_property(selector.clone()),
            )
            .with_state(state.clone())
            .with_debug_name("TextBox")
            .with_shared_property(label)
            .with_shared_property(selector)
            .with_shared_property(water_mark)
            .with_shared_property(selection)
            .with_shared_property(offset)
            .with_shared_property(focused)
            .with_event_handler(KeyEventHandler::default().on_key_down(Rc::new(
                move |key: Key| -> bool { state.update_text(key) },
            )))
    }
}
