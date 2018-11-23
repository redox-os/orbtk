extern crate orbtk;
use orbtk::*;

struct MainView;

impl Widget for MainView {
    fn template() -> Template {
        Container::template()
            .as_parent_type(ParentType::Single)
            .with_child(TextBlock::template())
    }
}

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .with_bounds(Rect::new(0, 0, 420, 730))
        .with_title("Orbtk")
        .with_root(MainView::template())
        .with_debug_flag(true)
        .build();
    application.run();
}
