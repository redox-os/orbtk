extern crate orbtk;
use orbtk::*;
use std::{rc::Rc};
static CENTER_DEMO_CSS: &'static str = include_str!("center-demo.css");

#[derive(Default)]
struct MainViewState {}

impl MainViewState {}

impl State for MainViewState {
}

struct MainView;

impl Widget for MainView {
  fn create() -> Template {
    let state = Rc::new(MainViewState::default());
    let label = SharedProperty::new(Label::from("prototype"));

    Template::default()
      .as_parent_type(ParentType::Single)
      .with_state(state.clone())
      // Nav bar
      .with_child(Column::create()
        .with_child(build_navbar())
        .with_child(build_main_view())
      )
      .with_shared_property(label)
      .with_debug_name("MainView")
  }
}

// Create a row for buttons and information
fn build_navbar() -> Template {
  let frame = Constraint::default()
    .with_width(400)
    .with_height(50)
    .with_min_width(400);

  Row::create().with_property(Selector::from("row").with_class("full"))
      .with_child(Column::create()
        .with_child(Button::create()
          .with_property(Selector::from("button").with_class("nav"))
          .with_property(Label::from("Home")),
        ),
      )
      .with_child(Column::create()
        .with_layout(FixedSizeLayout::default())
        .with_property(Selector::from("column").with_class("navview"))
        .with_child(Container::create()
          .with_property(Selector::from("container").with_class("navbarview"))
          .with_child(Center::create()
            .with_property(frame)
            .with_property(Selector::from("center").with_class("navtitleview"))
            .with_child(TextBlock::create()
              .with_property(Label::from("TITLE"))
              .with_property(Selector::from("textblock").with_class("navtitle")),
            ))))
      .with_child(Column::create()
        .with_child(Button::create()
          .with_property(Selector::from("button").with_class("nav"))
          .with_property(Label::from("Settings")),
      ),
    )
}

fn build_main_view() -> Template {
  Row::create()
    .with_child(build_boxview("Step 1"))
    .with_child(build_boxview("Step 2"))
    .with_child(build_boxview("Step 3"))
}

fn build_boxview(title: &str) -> Template {
  Column::create()
    .with_child(Container::create()
    .with_property(Selector::from("container").with_class("mainviewbox"))
    .with_child(Center::create()
    .with_property(Selector::from("center").with_class("mainviewbox"))
    .with_child(Button::create()
          .with_property(Selector::from("button").with_class("primary"))
          .with_property(Label::from(title)),
    )))
}

fn main() {
  let mut application = Application::default();
  application
    .create_window()
    .with_bounds(Bounds::new(100, 200, 600, 400))
    .with_title("Fixed and center layouts")
    .with_theme(Theme::create()
        .with_extenstion_css(CENTER_DEMO_CSS)
        .build(),
    )
    .with_root(MainView::create())
    .with_debug_flag(false)
    .build();
  application.run();
}
