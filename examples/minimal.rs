extern crate orbtk;
use orbtk::*;
use std::rc::Rc;

struct MainView;

impl Widget for MainView {
    fn template(&self) -> Template {
        Template::Single(Rc::new(Container {
            child: Some(Rc::new(TextBlock {
                label: Property::new(Label(String::from("OrbTk"))),
                ..Default::default()
            })),
                ..Default::default()
        }))
    }
}

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .with_bounds(Rect::new(0, 0, 420, 730))
        .with_title("Orbtk")
        .with_root(MainView)
        .build();
    application.run();
}
