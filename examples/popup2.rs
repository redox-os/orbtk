use orbtk::prelude::*;

static STACK_ID: &'static str = "STACK";
static BTN_ID: &'static str = "BUTTON";
static CMB_ID: &'static str = "COMBO BOX";
static TARGET_ID: &'static str = "TARGET";

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
        let target_entity = ctx.entity_of_child(TARGET_ID).unwrap();

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
                        let cmb = ctx.entity_of_child(CMB_ID).unwrap();
                        let selected_index: i32 = ctx.get_widget(cmb).clone("selected_index");
                        let relative_position: RelativePosition =
                            ctx.get_widget(popup).clone_or_default("relative_position");
                        match selected_index {
                            0 => ctx
                                .get_widget(self.popup.unwrap())
                                .set("relative_position", relative_position.to_bottom()),
                            1 => ctx
                                .get_widget(self.popup.unwrap())
                                .set("relative_position", relative_position.to_top()),
                            2 => ctx
                                .get_widget(self.popup.unwrap())
                                .set("relative_position", relative_position.to_left()),
                            3 => ctx
                                .get_widget(self.popup.unwrap())
                                .set("relative_position", relative_position.to_right()),
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

fn create_popup(target: Entity, text: &str, build_context: &mut BuildContext) -> Entity {
    Popup::new()
        // Entity as target
        .target(target.0)
        // Point as target
        //.target(Point::new(200.0,200.0))
        //Specify the popup position relative to the target (the button in this case)
        //This is also the default value if no one is specified
        .relative_position(RelativePosition::Bottom(1.0))
        .open(true)
        .width(150.0)
        .height(150.0)
        .child(
            Container::new()
                .background("#FFFFFF")
                .border_radius(3.0)
                .border_width(2.0)
                .border_brush("#000000")
                .padding(8.0)
                .child(
                    TextBlock::new()
                        .h_align("center")
                        .v_align("top")
                        .foreground("#000000")
                        .text(text)
                        .build(build_context),
                )
                .build(build_context),
        )
        .build(build_context)
}

fn change_button_title(title: &str, ctx: &mut Context) {
    let btn = ctx.entity_of_child(BTN_ID).unwrap();
    ctx.get_widget(btn)
        .set::<String16>("text", String16::from(title));
}

widget!(MainView<MainViewState>);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").margin(16.0).child(
            Grid::new()
                .rows(Rows::create().push(50).push(200).push(200).push(200))
                .columns(Columns::create().push(200).push(200).push(200))
                .id(STACK_ID)
                .child(
                    ComboBox::new()
                        .id(CMB_ID)
                        .attach(Grid::row(0))
                        .attach(Grid::column(0))
                        .h_align("center")
                        .width(250.0)
                        .on_changed("selected_item",move |states, _entity| {
                            states.get_mut::<MainViewState>(id).update_relative_position()
                        })
                        .items_builder(|bc, index| match index {
                            0 => TextBlock::new().text("Bottom").build(bc),
                            1 => TextBlock::new().text("Top").build(bc),
                            2 => TextBlock::new().text("Left").build(bc),
                            3 => TextBlock::new().text("Right").build(bc),
                            _ => panic!(),
                        })
                        .count(4)
                        .selected_index(0)
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .id(BTN_ID)
                        .attach(Grid::row(0))
                        .attach(Grid::column(1))
                        .h_align("center")
                        .text("Click me to hide popup")
                        .on_click(move |states, _| -> bool {
                            states.get_mut::<MainViewState>(id).toggle_popup();
                            true
                        })
                        .build(ctx),
                )
                .child(
                    Container::new()
                        .id(TARGET_ID)
                        .attach(Grid::row(2))
                        .attach(Grid::column(1))
                        .background("#0000FF")
                        .v_align("stretch")
                        .h_align("stretch")
                        .child(
                            TextBlock::new()
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
