extern crate orbtk;
use orbtk::*;

struct MainView;

impl Widget for MainView {
    fn create() -> Template {
        Column::create().with_child(
            Row::create()
                .with_child(
                    Container::create()
                        .with_child(Button::create().with_property(Label::from("Click me"))),
                )
                .with_child(
                    Container::create()
                        .with_child(TextBox::create().with_property(Label::from("Insert text"))),
                ),
        )
    }
}

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .with_bounds(Rect::new(0, 0, 420, 730))
        .with_title("Orbtk")
        .with_root(MainView::create())
        .with_debug_flag(true)
        .with_theme(Theme::parse(&theme::LIGHT_THEME_CSS))
        .build();
    application.run();
}
