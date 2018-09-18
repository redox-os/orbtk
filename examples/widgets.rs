extern crate orbtk;
use orbtk::*;

use std::sync::Arc;

struct MainView;

impl Widget for MainView {
    fn template(&self) -> Template {
        Template::Single(Arc::new(Button))
    }

    fn components(&self) -> Vec<ComponentBox> {
        vec![]
    }
}

fn main() {
    println!("Widget example");

    let mut application = Application::new();
    application
        .create_window()
        .with_bounds(Rect::new(0, 0, 420, 730))
        .with_title("Orbtk")
        .with_root(MainView)
        .build();
    application.run();
}
