use std::rc::Rc;

use event::Pressed;
use state::State;
use theme::Selector;
use widget::{
    add_selector_to_widget, remove_selector_from_widget, Center, Container, Label, Template,
    TextBlock, Widget, WidgetContainer,
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
    fn template() -> Template {
        print!("Button -> ");
        Template::default()
            .with_property(Label::from("Button"))
            .with_property(Selector::new().with("button"))
            .with_child(
                Container::template()
                    .with_child(Center::template().with_child(TextBlock::template())),
            )
            .with_state(ButtonState::default())
    }
}
