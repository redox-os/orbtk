use orbtk::*;

struct MainView;

impl Widget for MainView {
    type Template = Template;

    fn create() -> Template {
        Template::new()
            .parent_type(ParentType::Single)
            .debug_name("MainView")
            .child(TextBlock::create().text("Wait for next merge"))
    }
}

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .bounds(Bounds::new(100.0, 100.0, 800.0, 600.0))
        .title("OrbTk - canvas example")
        .root(MainView::create())
        .build();
    application.run();
}
