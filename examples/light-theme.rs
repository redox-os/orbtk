extern crate orbtk;
use orbtk::*;

use std::cell::Cell;
use std::rc::Rc;

#[derive(Default)]
struct MainViewState {
    counter: Cell<i32>,
}

impl MainViewState {
    pub fn increment(&self) {
        self.counter.set(self.counter.get() + 1)
    }
}

impl State for MainViewState {
    fn update(&self, widget: &mut WidgetContainer) {
        if let Ok(label) = widget.borrow_mut_property::<Label>() {
            label.0 = format!("Button count: {}", self.counter.get());
        }
    }
}

struct MainView {
    state: Rc<MainViewState>,
    counter: Property<Label>,
}

/*
    template!(
        Row {
            children: [
                Container {
                    child: Button {
                        label: "Click me",
                        handler: ButtonHandler {
                            on_mouse_up: || {
                                println!("Button 1 mouse up");
                            }
                        }
                    }
                },
                Container {
                    child: TextBox {
                        label: "Insert",
                        handler: TextBoxHandler {}
                    }
                }
            ]
        }
    )
*/

impl Widget for MainView {
    fn template(&self) -> Template {
        let state = self.state.clone();

        Template::Single(Rc::new(Column {
            children: vec![
                Rc::new(Row {
                    children: vec![
                        Rc::new(Container {
                            child: Some(Rc::new(Button {
                                label: Property::new(Label(String::from("Click me"))),
                                handler: Rc::new(Handler {
                                    on_mouse_up: Some(Rc::new(
                                        move |pos: Point, widget: &mut WidgetContainer| -> bool {
                                            if check_mouse_condition(pos, widget) {
                                                state.increment();
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
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
                Rc::new(Container {
                    child: Some(Rc::new(TextBlock {
                        label: self.counter.clone(),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
            ],

            ..Default::default()
        }))
    }

    fn properties(&self) -> Vec<PropertyResult> {
        vec![self.counter.build()]
    }

    fn state(&self) -> Option<Rc<State>> {
        Some(self.state.clone())
    }
}

fn main() {
    let mut application = Application::new();
    application
        .create_window()
        .with_bounds(Rect::new(0, 0, 420, 730))
        .with_title("Orbtk")
        .with_root(MainView {
            state: Rc::new(MainViewState::default()),
            counter: Property::new(Label(String::from("Button count: 0"))),
        })
        .with_theme(Theme::parse(theme::LIGHT_THEME_CSS))
        .build();
    application.run();
}
