use orbtk::*;

struct MainView;

impl Widget for MainView {
    fn create() -> Template {
        Template::default()
            .debug_name("MainView")
    }
}

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .bounds(Bounds::new(100.0, 100.0, 420.0, 730.0))
        .title("OrbTk - minimal example")
        .root(MainView::create())
        .debug_flag(true)
        .build();
    application.run();
}
