extern crate orbtk;

use std::sync::Arc;

use orbtk::{Application, Rect, Widget, Button, Center, Content};
use orbtk::theme::Theme;

struct MainView {}

impl MainView {
    fn new() -> Arc<MainView> {
        Arc::new(MainView{})
    }
}

impl Widget for MainView {
    fn build(&self) -> Content {
        let center = Center::new();
        center.child(&Button::new());
        Content::Single(center)
    }
}

fn main() {
    let mut application = Application::new(Rect::new(0, 0, 420, 730), "Orbtk");
    application.root(&MainView::new()).run();
}
