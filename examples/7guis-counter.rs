// https://eugenkiss.github.io/7guis/tasks#counter

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
        if let Ok(button_count_label) = context.widget().borrow_mut_property::<Label>() {
            button_count_label.0 = format!("    {}     ", self.counter.get());
        }
    }
}

fn create_space_row() -> Template {
    Row::create().with_property(Selector::from("row").with_class("space"))
}

fn create_counter_label(button_count_label: &SharedProperty) -> Template {
    Container::create()
        .with_child(TextBlock::create()
            .with_shared_property(button_count_label.clone())
            .with_property(Selector::from("textblock").with_class("fheight")),
        )
}

fn create_counter_button(state: Rc<MainViewState>) -> Template {
    Container::create()
        .with_child(Button::create()
            .with_property(Label::from("Count"))
            .with_event_handler(MouseEventHandler::default().on_click(
                Rc::new(move |_pos: Point| -> bool {
                    state.increment();
                    true
                }),
            )),
        )
}

struct MainView;

impl Widget for MainView {
    fn create() -> Template {
        let state = Rc::new(MainViewState::default());
        let button_count_label = SharedProperty::new(Label::from("0"));

        Template::default()
            .as_parent_type(ParentType::Single)
            .with_state(state.clone())
            .with_child(create_space_row()
                .with_child(Column::create()
                    .with_child(create_counter_button(state))
            )
                .with_child(Column::create()
                    .with_child(create_counter_label(&button_count_label)))
            )
            .with_shared_property(button_count_label)
            .with_debug_name("MainView")
    }
}

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .with_bounds(Bounds::new(0, 0, 400, 100))
        .with_title("Counter")
        .with_root(MainView::create())
        .with_debug_flag(false)
        .build();
    application.run();
}
