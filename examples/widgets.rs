// use orbtk::*;

// use std::{cell::Cell, rc::Rc};

// #[derive(Default)]
// struct MainViewState {
//     counter: Cell<i32>,
// }

// impl MainViewState {
//     pub fn increment(&self) {
//         self.counter.set(self.counter.get() + 1)
//     }
// }

// impl State for MainViewState {
//     fn update(&self, context: &mut Context<'_>) {
//         if let Ok(button_count_label) = context.widget().borrow_mut_property::<Label>() {
//             button_count_label.0 = format!("Button count: {}", self.counter.get());
//         }
//     }
// }

// fn create_header(text: &str) -> Template {
//     TextBlock::create()
//         .property(Label::from(text))
//         .property(Selector::from("textblock").class("h1"))
// }

// fn create_space_row() -> Template {
//     Row::create().property(Selector::from("row").class("space"))
// }

// struct MainView;

// impl Widget for MainView {
//     fn create() -> Template {
//         let state = Rc::new(MainViewState::default());
//         let button_count_label = SharedProperty::new(Label::from("Button count: 0"));

//         Template::default()
//             .state(state.clone())
//             .child(
//                 create_space_row()
//                     .child(
//                         Column::create()
//                             .child(Container::create().child(create_header("Buttons")))
//                             .child(
//                                 Container::create().child(
//                                     Button::create()
//                                         .property(Label::from("Button"))
//                                         .property(FontIcon::from(
//                                             styling::vector_graphics::material_font_icons::CHECK_FONT_ICON,
//                                         ))
//                                         .event_handler(MouseEventHandler::default().on_click(
//                                             Rc::new(move |_pos: Point| -> bool {
//                                                 state.increment();
//                                                 true
//                                             }),
//                                         )),
//                                 ),
//                             )
//                             .child(
//                                 Container::create().child(
//                                     Button::create()
//                                         .property(
//                                             Selector::from("button").class("primary"),
//                                         )
//                                         .property(Label::from("Primary")),
//                                 ),
//                             )
//                             .child(Container::create().child(
//                                 ToggleButton::create().property(Label::from("ToggleButton")),
//                             ))
//                             .child(Container::create().child(
//                                 CheckBox::create().property(Label::from("CheckBox")),
//                             ))
//                             .child(Container::create().child(Switch::create())),
//                     )
//                     .child(
//                         Column::create()
//                             .child(Container::create().child(create_header("Text")))
//                             .child(
//                                 Container::create().child(
//                                     TextBlock::create()
//                                         .shared_property(button_count_label.clone())
//                                         .property(
//                                             Selector::from("textblock").class("fheight"),
//                                         ),
//                                 ),
//                             )
//                             .child(Container::create().child(
//                                 TextBox::create().property(WaterMark::from("TextBox...")),
//                             )), // .child(Container::create().child(
//                                 //     TextBox::create().property(WaterMark::from("TextBox...")),
//                                 // ))
//                     ),
//             )
//             .shared_property(button_count_label)
//             .debug_name("MainView")
//     }
// }

// fn main() {
//     let mut application = Application::default();

//     application
//         .create_window()
//         .bounds(Bounds::new(100, 100, 420, 730))
//         .title("OrbTk - widgets example")
//         .root(MainView::create())
//         .resizable(true)
//         .debug_flag(false)
//         .build();
//     application.run();
// }

fn main() {}
