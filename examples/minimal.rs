use orbtk::*;

struct MainView;

impl Widget for MainView {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_debug_name("MainView")

    }
}

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .with_bounds(Bounds::new(100.0, 100.0, 420.0, 730.0))
        .with_title("OrbTk - minimal example")
        .with_root(MainView::create())
        .with_debug_flag(true)
        .build();
    application.run();
}
