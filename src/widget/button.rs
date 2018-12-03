use enums::ParentType;
use properties::{FontIcon, Label, Pressed};
use theme::Selector;
use widget::{Center, Container, FontIconBlock, Row, SharedProperty, Template, TextBlock, Widget};

/// The `Button` widget can be clicked by user. It's used to peform an action.
///
/// # Shared Properties
///
/// * `Label` - String used to display the text of the button.
/// * `FontIcon` - String used to display the font icon of the button.
/// * `Selector` - CSS selector with  element name `button`, used to request the theme of the widget.
///
/// # Properties
///
/// * `Pressed` - Bool value represents the pressed state of the button.
///
/// # Others
///
/// * `ParentType`- Single.
pub struct Button;

impl Widget for Button {
    fn create() -> Template {
        let label = SharedProperty::new(Label::default());
        let icon = SharedProperty::new(FontIcon::default());
        let selector = SharedProperty::new(Selector::from("button"));

        Template::default()
            .as_parent_type(ParentType::Single)
            .with_child(
                Container::create()
                    .with_shared_property(selector.clone())
                    .with_child(
                        Center::create().with_child(
                            Row::create()
                                .with_child(
                                    FontIconBlock::create()
                                        .with_shared_property(icon.clone())
                                        .with_shared_property(selector.clone()),
                                )
                                .with_child(
                                    TextBlock::create()
                                        .with_shared_property(label.clone())
                                        .with_shared_property(selector.clone()),
                                ),
                        ),
                    ),
            )
            .with_shared_property(label)
            .with_shared_property(icon)
            .with_shared_property(selector)
            .with_property(Pressed(false))
            .with_debug_name("Button")
    }
}
