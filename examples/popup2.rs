use orbtk::{
    prelude::*,
    theme_default::{THEME_DEFAULT, THEME_DEFAULT_COLORS_DARK, THEME_DEFAULT_FONTS},
    theming::config::ThemeConfig,
};

static DARK_EXT: &str = include_str!("../assets/popup/default_dark.ron");

fn theme() -> Theme {
    register_default_fonts(Theme::from_config(
        ThemeConfig::from(DARK_EXT)
            .extend(ThemeConfig::from(THEME_DEFAULT))
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_DARK))
            .extend(ThemeConfig::from(THEME_DEFAULT_FONTS)),
    ))
}

static ID_GRID: &str = "GRID";
static ID_BUTTON: &str = "BUTTON";
static ID_COMBO_BOX: &str = "COMBO BOX";
static ID_TARGET: &str = "TARGET";

#[derive(Copy, Clone)]
enum PopUpAction {
    Toggle,
    UpdateRelativePosition,
}

#[derive(Default, AsAny)]
struct MainViewState {
    action: Option<PopUpAction>,
    popup: Option<Entity>,
}

impl MainViewState {
    fn toggle_popup(&mut self) {
        self.action = Some(PopUpAction::Toggle);
    }

    fn update_relative_position(&mut self) {
        self.action = Some(PopUpAction::UpdateRelativePosition);
    }
}

impl State for MainViewState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        let target_entity = ctx.entity_of_child(ID_TARGET).unwrap();

        let popup = create_popup(target_entity, "Popup text", &mut ctx.build_context());
        ctx.build_context()
            .append_child_to_overlay(popup)
            .expect("Failed to add popup to overlay");
        self.popup = Some(popup);
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                PopUpAction::Toggle => {
                    if let Some(popup) = self.popup {
                        let open = ctx.get_widget(popup).clone::<bool>("open");
                        println!("Current open: {}", open);
                        println!(
                            "Current visibility: {:#?}",
                            ctx.get_widget(popup).clone::<Visibility>("visibility")
                        );
                        if open {
                            ctx.get_widget(popup).set("open", false);
                            change_button_title("Click me to show popup", ctx);
                        } else {
                            ctx.get_widget(popup).set("open", true);
                            change_button_title("Click me to hide popup", ctx);
                        }
                        println!("Popup toggled !");
                    }
                }
                PopUpAction::UpdateRelativePosition => {
                    if let Some(popup) = self.popup {
                        let cmb = ctx.entity_of_child(ID_COMBO_BOX).unwrap();
                        let selected_index: i32 = ctx.get_widget(cmb).clone("selected_index");
                        let relative_position: RelativePosition =
                            ctx.get_widget(popup).clone_or_default("relative_position");
                        match selected_index {
                            0 => ctx
                                .get_widget(self.popup.unwrap())
                                .set("relative_position", relative_position.into_bottom()),
                            1 => ctx
                                .get_widget(self.popup.unwrap())
                                .set("relative_position", relative_position.into_top()),
                            2 => ctx
                                .get_widget(self.popup.unwrap())
                                .set("relative_position", relative_position.into_left()),
                            3 => ctx
                                .get_widget(self.popup.unwrap())
                                .set("relative_position", relative_position.into_right()),
                            _ => panic!(),
                        }
                        println!("Relative position updated");
                    }
                }
            }
            self.action = None;
        }
    }
}

fn create_popup(target: Entity, text: &str, ctx: &mut BuildContext) -> Entity {
    Popup::new()
        // Entity as target
        .target(target.0)
        // Point as target
        // .target(Point::new(200.0,200.0))
        // Specify the popup position relative to the target (the button in this case)
        // This is also the default value if no one is specified
        .relative_position(RelativePosition::Bottom(5.0))
        .open(true)
        .style("popup_form")
        .width(150.0)
        .height(150.0)
        .child(
            Container::new()
                .child(
                    TextBlock::new()
                        .style("popup_text_block")
                        .h_align("center")
                        //.v_align("top")
                        .text(text)
                        .build(ctx),
                )
                .build(ctx),
        )
        .build(ctx)
}

fn change_button_title(title: &str, ctx: &mut Context) {
    Button::text_set(&mut ctx.child(ID_BUTTON), String::from(title));
}

widget!(MainView<MainViewState>);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").margin(16.0).child(
            Grid::new()
                .rows(Rows::create().push(50).push(200).push(200).push(200))
                .columns(Columns::create().push(200).push(200).push(200))
                .id(ID_GRID)
                .child(
                    ComboBox::new()
                        .id(ID_COMBO_BOX)
                        .attach(Grid::row(0))
                        .attach(Grid::column(0))
                        .style("combo_box_form")
                        .h_align("center")
                        //.v_align("center")
                        //.width(100.0)
                        .on_changed("selected_item", move |states, _entity| {
                            states
                                .get_mut::<MainViewState>(id)
                                .update_relative_position()
                        })
                        .items_builder(|ictx, index| match index {
                            0 => TextBlock::new()
                                .text("Bottom")
                                .v_align("center")
                                .build(ictx),
                            1 => TextBlock::new().text("Top").v_align("center").build(ictx),
                            2 => TextBlock::new().text("Left").v_align("center").build(ictx),
                            3 => TextBlock::new().text("Right").v_align("center").build(ictx),
                            _ => panic!(),
                        })
                        .count(4)
                        .selected_index(0)
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .id(ID_BUTTON)
                        .attach(Grid::row(0))
                        .attach(Grid::column(1))
                        .style("button")
                        //.h_align("center")
                        .text("Click me to hide popup")
                        .on_click(move |ctx, _| -> bool {
                            ctx.get_mut::<MainViewState>(id).toggle_popup();
                            true
                        })
                        .build(ctx),
                )
                .child(
                    Container::new()
                        .id(ID_TARGET)
                        .attach(Grid::row(2))
                        .attach(Grid::column(1))
                        .style("container_form")
                        .child(
                            TextBlock::new()
                                .style("target_text_block")
                                .text("Target")
                                .v_align("center")
                                .h_align("center")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

fn main() {
    Application::new()
        .theme(theme())
        .window(|ctx| {
            Window::new()
                .title("OrbTk - Popup example")
                .position((100.0, 100.0))
                .size(750, 750.0)
                .resizeable(true)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}
