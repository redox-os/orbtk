extern crate orbtk;
use orbtk::*;

use std::sync::Arc;

// use orbtk::{Application, Button, Center, Container, Content, Rect, Row, Thickness, Label, Widget};
// use orbtk::theme::{Style, Selector};

// struct MainView {}

// impl MainView {
//     fn new() -> Arc<MainView> {
//         Arc::new(MainView {})
//     }
// }

// impl Widget for MainView {
//     fn build(&self) -> Content {
//         /* todo: content! marco
//             content!(
//                 Center (
//                     Row (
//                         Container (
//                             padding: 8,
//                             Button (
//                                 text: "Button" 1
//                             )
//                         )
//                         Container (
//                             padding: 8,
//                             Button (
//                                 text: "Button 2"
//                             )
//                         )
//                     )
//                 )
//             )
//         */
//         let container = Container::new();
//         container.selector().set(Selector::new(Some("button")));
//         let root = Container::new();
//         root.padding().set(Thickness::new(32, 32, 32, 32));
//         root.child(&container);
//         Content::Single(root)
//         // let center = Center::new();
//         // // let row = Row::new();
//         // // let left_container = Container::new();
//         // // left_container.padding().set(Thickness::new(8, 8, 8, 8));
//         // // left_container.child(&Button::new());
//         // // row.push(&left_container);
//         // // let right_container = Container::new();
//         // // right_container.padding().set(Thickness::new(8, 8, 8, 8));
//         // // right_container.child(&Button::new());
//         // // row.push(&right_container);
//         // let label = Label::new("Test");
//         // center.child(&label);
//         // Content::Single(center)
//     }
//     fn element(&self) -> &str {
//         "mainView"
//     }
// }

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
    // let mut wm = WidgetManager::new();
    // wm.root(Arc::new(MainView));
    // wm.run();

    let mut application = Application::new();
    application
        .create_window()
        .with_bounds(Rect::new(0, 0, 420, 730))
        .with_title("Orbtk")
        .with_root(MainView)
        .build();
    application.run();

    // let application = Application::new(Rect::new(0, 0, 420, 730), "Orbtk");
    // application.root(&MainView::new()).run();
}
