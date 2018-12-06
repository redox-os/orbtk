extern crate orbtk;
use orbtk::*;

use std::cell::Cell;
use std::rc::Rc;

#[derive(Default)]
struct MainViewState {
    counter: Cell<i32>,
}

impl MainViewState {
    pub fn increment(&self) {
        self.counter.set(self.counter.get() + 1)
    }
}

impl State for MainViewState {
    fn update(&self, context: &mut Context) {
        if let Ok(button_count_label) = context.widget.borrow_mut_property::<Label>() {
            button_count_label.0 = format!("Button count: {}", self.counter.get());
        }
    }
}

fn create_header(text: &str) -> Template {
    TextBlock::create()
        .with_property(Label::from(text))
        .with_property(Selector::from("textblock").with_class("h1"))
}

fn create_space_row() -> Template {
    Row::create().with_property(Selector::from("row").with_class("space"))
}

struct MainView;

impl Widget for MainView {
    fn create() -> Template {
        let state = Rc::new(MainViewState::default());
        let button_count_label = SharedProperty::new(Label::from("Button count: 0"));

        Template::default()
            .as_parent_type(ParentType::Single)
            .with_state(state.clone())
            .with_child(
                create_space_row()
                    .with_child(
                        Column::create()
                            .with_child(Container::create().with_child(create_header("Buttons")))
                            .with_child(
                                Container::create().with_child(
                                    Button::create()
                                        .with_property(Label::from("Button"))
                                        .with_property(FontIcon::from(
                                            theme::material_font_icons::CHECK_FONT_ICON,
                                        ))
                                        .with_event_handler(MouseEventHandler::default().on_click(
                                            Rc::new(move |_pos: Point| -> bool {
                                                state.increment();
                                                true
                                            }),
                                        )),
                                ),
                            )
                            .with_child(Container::create().with_child(
                                ToggleButton::create().with_property(Label::from("ToggleButton")),
                            ))
                            .with_child(Container::create().with_child(
                                CheckBox::create().with_property(Label::from("CheckBox")),
                            )),
                    )
                    .with_child(
                        Column::create()
                            .with_child(Container::create().with_child(create_header("Text")))
                            .with_child(
                                Container::create().with_child(
                                    TextBlock::create()
                                        .with_shared_property(button_count_label.clone())
                                        .with_property(
                                            Selector::from("textblock").with_class("fheight"),
                                        ),
                                ),
                            )
                            .with_child(Container::create().with_child(
                                TextBox::create().with_property(WaterMark::from("TextBox...")),
                            ))
                            // .with_child(Container::create().with_child(
                            //     TextBox::create().with_property(WaterMark::from("TextBox...")),
                            // ))
                    ),
            )
            .with_shared_property(button_count_label)
            .with_debug_name("MainView")
    }
}

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .with_bounds(Rect::new(0, 0, 420, 730))
        .with_title("Orbtk")
        .with_root(MainView::create())
        .with_debug_flag(false)
        .build();
    application.run();
}
