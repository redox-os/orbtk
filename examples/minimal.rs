use orbtk::prelude::*;

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .bounds((100.0, 100.0, 420.0, 730.0))
        .title("OrbTk - minimal example")
        .debug_flag(false)
        .build(TextBlock::create().text("OrbTk"));
    application.run();
}

