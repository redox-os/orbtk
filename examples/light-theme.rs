extern crate orbtk;
use orbtk::*;

use std::rc::Rc;

struct MainView;

impl Widget for MainView {
    fn template(&self) -> Template {
        Template::Single(Rc::new(Row {
            children: vec![
                Rc::new(Container {
                    child: Some(Rc::new(Button {
                        label: Property::new(Label(String::from("Click me"))),
                        state: Rc::new(ButtonState {
                            on_mouse_up: Some(Rc::new(|| {
                                println!("Button 1 mouse up");
                            })),
                            ..Default::default()
                        }),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
                Rc::new(Container {
                    child: Some(Rc::new(TextBox {
                        label: Property::new(Label(String::from("Insert"))),
                        state: Rc::new(TextBoxState {
                            ..Default::default()
                        }),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }))
    }
}

fn main() {
    let mut application = Application::new();
    application
        .create_window()
        .with_bounds(Rect::new(0, 0, 420, 730))
        .with_title("Orbtk")
        .with_root(MainView)
        .with_theme(Theme::parse(theme::LIGHT_THEME_CSS))
        .build();
    application.run();
}
