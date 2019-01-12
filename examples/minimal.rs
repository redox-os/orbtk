use orbtk::*;

struct MainView;

impl Widget for MainView {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_debug_name("MainView")
            .with_child(
                Container::create()
                    .as_parent_type(ParentType::Single)
                    .with_child(Button::create().with_property(Label::from("OrbTk"))),
            )
    }
}

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .with_bounds(Bounds::new(0, 0, 420, 730))
        .with_title("OrbTk - Minimal example")
        .with_root(MainView::create())
        .with_debug_flag(true)
        .build();
    application.run();
}
