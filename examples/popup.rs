use orbtk::prelude::*;

static STACK_ID: &'static str = "STACK";
static BTN_ID: &'static str = "BUTTON";

#[derive(Copy, Clone)]
enum PopUpAction {
   Show,
   Hide
}

#[derive(Default, AsAny)]
struct MainViewState {
    action: Option<PopUpAction>,
    show_popup: bool,
    popup: Option<Entity>
}

impl MainViewState {
    fn show_popup(&mut self) {
        if self.show_popup {
            self.action = Some(PopUpAction::Hide);
        } else {
            self.action = Some(PopUpAction::Show);
        }
        self.show_popup = !self.show_popup;
    }
}

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                PopUpAction::Show => {
                    let stack = ctx.entity_of_child(STACK_ID).unwrap();
                    let current_entity = ctx.entity;
                    let build_context = &mut ctx.build_context();
                    
                    let popup = create_popup(current_entity, "Popup text", build_context);
                    build_context.append_child(stack, popup);
                    self.popup = Some(popup);
                    change_button_title("Click me to hide popup", ctx);
                    println!("Popup created: {:?}", self.popup);
                }
                PopUpAction::Hide => {
                    if let Some(popup) = self.popup {
                        ctx.remove_child(popup);
                        change_button_title("Click me to show popup", ctx);
                        println!("Popup removed !");
                    }
                }
            }
            self.action = None;
        }
    }
}

fn create_popup(target: Entity, text: &str, build_context: &mut BuildContext) -> Entity {
    Popup::new()
        .target(target)
        .open(true)
        .width(250.0)
        .height(250.0)
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
                .build(build_context)
            )
            .build(build_context)
        )
        .build(build_context)
}

fn change_button_title(title: &str, ctx: &mut Context) {
    let btn = ctx.entity_of_child(BTN_ID).unwrap();
    ctx.get_widget(btn).set::<String16>("text", String16::from(title));
}

widget!(MainView<MainViewState>);

impl Template for MainView {
    fn template(self, id: Entity, bc: &mut BuildContext) -> Self {
        self.name("MainView")
        .margin(16.0)
        .child(
            Stack::new()
            .id(STACK_ID)
            .h_align("center")
            .spacing(16.0)
            .child(
                Button::new()
                .id(BTN_ID)
                .v_align("top")
                .h_align("center")
                .text("Click me to show popup")
                .width(250.0)
                .on_click(move |states, _| -> bool {
                    states.get_mut::<MainViewState>(id).show_popup();
                    true
                })
                .build(bc))
               
            .build(bc)
        )
    }
}

fn main() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk - Popup example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}