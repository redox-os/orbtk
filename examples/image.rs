use orbtk::*;

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .bounds((100.0, 100.0, 800.0, 420.0))
        .title("OrbTk - minimal example")
        .debug_flag(false)
        .build(ImageWidget::create().image("res/orbtk-space.png"));
    application.run();
}