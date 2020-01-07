use orbtk::prelude::*;
fn main() {
  Application::from_name("Ironix")
        .window(|ctx| {
            Window::create()
                .title("Buggy")
                .size(400.0, 400.0)
                .child(
                    Grid::create()
                        .columns(
                            Columns::create()
                                .column(100.0)
                                .column(100.0)
                                .build()
                        )
                        .child(
                            TextBlock::create().text("test")
                                .attach(Grid::column(0))
                                .build(ctx)
                        )
                        .child(
                            TextBlock::create().text("test")
                                .attach(Grid::column(1))
                                .build(ctx)
                        )
                        .build(ctx)
                )
                .build(ctx)
        })
        .run();

}