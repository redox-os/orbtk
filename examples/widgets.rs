extern crate orbtk;

use std::sync::Arc;

use orbtk::{Application, Button, Center, Content, Rect, Widget, Row};
use orbtk::theme::Theme;

struct MainView {}

impl MainView {
    fn new() -> Arc<MainView> {
        Arc::new(MainView {})
    }
}

impl Widget for MainView {
    fn build(&self) -> Content {
        let center = Center::new();
        let row = Row::new();
        row.push(&Button::new());
        row.push(&Button::new());
        center.child(&row);
        Content::Single(center)
    }
    fn element(&self) -> &str {
        "mainView"
    }
}

fn main() {
    let mut application = Application::new(Rect::new(0, 0, 420, 730), "Orbtk");
    application.root(&MainView::new()).print_tree().run();
}
