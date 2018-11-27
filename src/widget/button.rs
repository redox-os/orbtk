use std::rc::Rc;

use enums::ParentType;
use event::Pressed;
use structs::Label;
use theme::Selector;
use widget::{
    add_selector_to_widget, remove_selector_from_widget, Center, Container, SharedProperty,
    Template, TextBlock, Widget, WidgetContainer, State
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

impl Into<Rc<State>> for ButtonState {
    fn into(self) -> Rc<State> {
        Rc::new(self)
    }
}

/// The `Button` struct represents a widget that can be clicked by user. It's used to peform an action.
pub struct Button;

impl Widget for Button {
    fn create() -> Template {
        let label = SharedProperty::new(Label::from("Button"));
        let selector = SharedProperty::new(Selector::new().with("button"));
        let state = Rc::new(ButtonState::default());

        Template::default()
            .as_parent_type(ParentType::Single)
            .with_child(
                Container::create()
                    .with_shared_property(selector.clone())
                    .with_child(
                        Center::create().with_child(
                            TextBlock::create()
                                .with_shared_property(label.clone())
                                .with_shared_property(selector.clone()),
                        ),
                    ),
            )
            .with_shared_property(label)
            .with_shared_property(selector)
            .with_property(Pressed(false))
            .with_state(state)
            .with_debug_name("Button")
    }
}
