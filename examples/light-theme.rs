extern crate orbtk;
use orbtk::*;

use std::rc::Rc;

struct MainView;

impl Widget for MainView {
    fn template(&self) -> Template {
        Template::Single(Rc::new(Column {
            children: vec![
                Rc::new(Row {
                    children: vec![
                        Rc::new(Container {
                            child: Some(Rc::new(Button {
                                label: Property::new(Label(String::from("Click me"))),
                                state: Rc::new(State {
                                    on_mouse_up: Some(Rc::new(
                                        |pos: Point, widget: &mut WidgetContainer| -> bool {
                                            if check_mouse_condition(pos, widget) {
                                                println!("Button 1 mouse up");
                                                return true;
                                            }

                                            false
                                        },
                                    )),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                        Rc::new(Container {
                            child: Some(Rc::new(TextBox {
                                label: Property::new(Label(String::from("Insert Insert"))),
                                // state: Rc::new(TextBoxState {
                                //     ..Default::default()
                                // }),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
                Rc::new(Container {
                    child: Some(Rc::new(TextBlock {
                        // label: self.state.button_counter.clone(),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
            ],

            ..Default::default()
        }))
    }

    // fn state(&self) -> Option<Rc<State>> {
    //     Some(self.state.clone())
    // }
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
