use orbtk::prelude::*;

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - minimal example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(
                    ItemsWidget::create()
                        .selector("blub")
                        .padding((4.0, 4.0, 4.0, 2.0))
                        .margin((0.0, 8.0, 0.0, 8.0))
                        .items_builder(move |bc, index| {
                            Button::create()
                                .margin((0.0, 0.0, 0.0, 2.0))
                                .text("button")
                                .build(bc)
                        })
                        .count(3)
                        .build(ctx),
                )
                .build(ctx)
        })
        .run();
}

// use orbtk::prelude::*;

// fn main() {
//     // use this only if you want to run it as web application.
//     orbtk::initialize();

//     Application::new()
//         .window(|ctx| {
//             Window::create()
//                 .title("OrbTk - minimal example")
//                 .position((100.0, 100.0))
//                 .size(420.0, 730.0)
//                 .child(
//                     Stack::create()
//                         .orientation("vertical")
//                         .child(
//                             Button::create()
//                                 .margin((10.0, 10.0, 0.0, 0.0))
//                                 .horizontal_alignment("start")
//                                 .text("Button 1")
//                                 .build(ctx),
//                         )
//                         .child(
//                             Button::create()
//                                 .horizontal_alignment("end")
//                                 .text("Button 2")
//                                 .build(ctx),
//                         )
//                         .child(
//                             Button::create()
//                                 .text("Button 2")
//                                 .build(ctx),
//                         )
//                         .build(ctx),
//                 )
//                 .build(ctx)
//         })
//         .run();
// }

// use orbtk::prelude::*;

// fn main() {
//     // use this only if you want to run it as web application.
//     orbtk::initialize();

//     Application::new()
//         .window(|ctx| {
//             Window::create()
//                 .title("OrbTk - minimal example")
//                 .position((100.0, 100.0))
//                 .size(420.0, 730.0)
//                 .child(TextBlock::create().text("OrbTk").margin(4.0).build(ctx))
//                 .build(ctx)
//         })
//         .run();
// }
