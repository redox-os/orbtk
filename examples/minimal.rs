use orbtk::*;

struct MainView;

impl Widget for MainView {
    type Template = Template;

    fn create() -> Template {
        Template::new()
            .child(
                Container::create().padding(6.0).child(
                    Stack::create()
                        .child(
                            Button::create()
                                .text("Button with long long text")
                                .font_icon(FLOPPY_FONT_ICON)
                                .horizontal_alignment("Left"),
                        )
                        .child(
                            ToggleButton::create()
                                .margin((0.0, 8.0, 0.0, 0.0))
                                .text("Toggle")
                                .horizontal_alignment("Left"),
                        )
                        .child(
                            CheckBox::create()
                                .margin((0.0, 8.0, 0.0, 0.0))
                                .text("CheckBox")
                                .horizontal_alignment("Left"),
                        )
                        .child(
                            Switch::create()
                                .margin((0.0, 8.0, 0.0, 0.0))
                                .horizontal_alignment("Left"),
                        )
                        .child(
                            TextBox::create()
                                .margin((0.0, 8.0, 0.0, 0.0))
                                .horizontal_alignment("Left")
                                .text("")
                                .water_mark("Insert text")
                        ),
                ),
            )
            .debug_name("MainView")
    }
}

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .bounds(Bounds::new(100.0, 100.0, 420.0, 730.0))
        .title("OrbTk - minimal example")
        .root(MainView::create())
        .debug_flag(false)
        .build();
    application.run();
}
