use orbtk::*;

struct MainView;

impl Widget for MainView {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_debug_name("MainView")
            .with_child(
                ImageWidget::create()
                    .with_shape(ImageElement::create("res/orbtk-space.png").build()),
            )
    }
}

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .with_bounds(Bounds::new(0, 0, 800, 420))
        .with_title("OrbTk - Image example")
        .with_root(MainView::create())
        .build();
    application.run();
}
