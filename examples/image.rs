use orbtk::*;

struct MainView;

impl Widget for MainView {
    type Template = Template;

    fn create() -> Self::Template {
        Template::new()
            .debug_name("MainView")
            .child(ImageWidget::create().image("res/orbtk-space.png"))
    }
}

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .bounds((100.0, 100.0, 800.0, 420.0))
        .title("OrbTk - image example")
        .root(MainView::create())
        .build();
    application.run();
}
