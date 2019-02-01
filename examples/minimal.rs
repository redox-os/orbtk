use orbtk::*;

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .bounds(Bounds::new(100.0, 100.0, 420.0, 730.0))
        .title("OrbTk - minimal example")
        .root(TextBlock::create().text("OrbTk").into())
        .debug_flag(false)
        .build();
    application.run();
}
