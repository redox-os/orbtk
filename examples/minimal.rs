extern crate orbtk;
use orbtk::*;

extern crate orbrender;
use orbrender::structs::Size;
use orbrender::window::WindowBuilder;

struct MainView;

impl Widget for MainView {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_debug_name("MainView")
            .with_child(
                Container::create()
            )
    }
}

fn main() {
     orbtk::initialize();

    let mut application = Application::default();
    application
        .main_window(
            WindowBuilder::default()
                .with_size(Size::new(420.0, 730.0))
                .with_title("OrbTk - Minimal example")
                .build(),
        )
        .with_root(MainView::create())
        .with_debug_flag(true)
        .finish();

    application.run();
}