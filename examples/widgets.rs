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
    fn update(&self, widget: &mut WidgetContainer) {
        if let Ok(label) = widget.borrow_mut_property::<Label>() {
            label.0 = format!("Button count: {}", self.counter.get());
        }
    }
}

struct MainView;

impl Widget for MainView {
    fn create() -> Template {
        let state = Rc::new(MainViewState::default());
        let label = SharedProperty::new(Label::from("Button count: 0"));

        Template::default()
            .as_parent_type(ParentType::Single)
            .with_state(state.clone())
            .with_child(
                Column::create()
                    .with_child(
                        Row::create()
                            .with_child(
                                Container::create().with_child(
                                    Button::create()
                                        .with_property(Label::from("Click me"))
                                        .with_event_handler(MouseEventHandler::default().on_click(Rc::new( move |_pos: Point, _widget: &mut WidgetContainer| -> bool { state.increment(); true }))),
                                ),
                            )
                            .with_child(Container::create().with_child(
                                TextBox::create().with_property(WaterMark::from("Placeholder")),
                            )),
                    )
                    .with_child(
                        Row::create().with_child(
                            Container::create()
                                .with_child(TextBlock::create().with_shared_property(label.clone())),
                        ),
                    ),
            ).with_shared_property(label).with_debug_name("MainView")
    }
}

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .with_bounds(Rect::new(0, 0, 420, 730))
        .with_title("Orbtk")
        .with_root(MainView::create())
        .with_debug_flag(true)
        .build();
    application.run();
}
