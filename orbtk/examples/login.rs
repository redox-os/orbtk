use orbtk::prelude::*;

const USERNAME: &str = "root";
const PASSWORD: &str = "toor";
const USERNAME_INPUT_ID: &str = "username_input";
const PASSWORD_INPUT_ID: &str = "password_input";

enum LoginAction {
    Authenticate,
    ShowPopup,
    ClosePopup,
}

#[derive(Default, AsAny)]
struct LoginFormState {
    authenticated: bool,
    action: Option<LoginAction>,
    username_input: Entity,
    password_input: Entity,
    show_popup: bool,
    popup: Option<Entity>,
}

impl LoginFormState {
    fn authenticate(&mut self) {
        if !self.show_popup {
            self.action = Some(LoginAction::Authenticate);
        }
    }

    fn show_popup(&mut self) {
        if !self.show_popup {
            self.action = Some(LoginAction::ShowPopup);
        }
    }

    fn close_popup(&mut self) {
        if self.show_popup {
            self.action = Some(LoginAction::ClosePopup);
        }
    }

    // creates a popup based on the authenticated field and returns its entity
    fn create_popup(&self, target: Entity, build_context: &mut BuildContext) -> Entity {
        let (msg, text_color) = match self.authenticated {
            true => ("Login success!", "#4CA64C"),
            false => ("Login failed!", "#FF3232"),
        };

        Popup::new()
            .style("popup")
            .target(target)
            .open(true)
            .width(175.0)
            .height(125.0)
            .h_align("center")
            .v_align("center")
            .child(
                Container::new()
                    .border_radius(3.0)
                    .border_width(2.0)
                    .padding(8.0)
                    .child(
                        TextBlock::new()
                            .font_size(18.0)
                            .foreground(text_color)
                            .h_align("center")
                            .v_align("top")
                            .text(msg)
                            .build(build_context),
                    )
                    .child(
                        Button::new()
                            .h_align("center")
                            .v_align("center")
                            .text("OK")
                            // Send a ClosePopup action to LoginFormState when button is clicked
                            .on_click(move |states, _point| -> bool {
                                states.get_mut::<LoginFormState>(target).close_popup();
                                true
                            })
                            .build(build_context),
                    )
                    .build(build_context),
            )
            .build(build_context)
    }
}

impl State for LoginFormState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.authenticated = false;
        self.username_input = ctx
            .entity_of_child(USERNAME_INPUT_ID)
            .expect("Username input could not be found !");
        self.password_input = ctx
            .entity_of_child(PASSWORD_INPUT_ID)
            .expect("Password input could not be found !");
        self.show_popup = false;
        self.popup = None;
    }

    fn update(&mut self, reg: &mut Registry, ctx: &mut Context) {
        if let Some(action) = &self.action {
            match action {
                // read both textbox and passwordbox values to compare then show a popup
                LoginAction::Authenticate => {
                    let username = ctx.get_widget(self.username_input).clone::<String>("text");
                    let password = ctx.get_widget(self.password_input).clone::<String>("text");

                    if USERNAME.eq(username.as_str()) && PASSWORD.eq(password.as_str()) {
                        self.authenticated = true;
                    } else {
                        self.authenticated = false;
                    }

                    self.show_popup();
                    self.update(reg, ctx);
                }
                // creates a popup then attach it to the overlay
                LoginAction::ShowPopup => {
                    let current_entity = ctx.entity();
                    let build_context = &mut ctx.build_context();
                    let popup = self.create_popup(current_entity, build_context);
                    build_context.append_child(current_entity, popup);
                    self.show_popup = true;
                    self.popup = Some(popup);
                }
                // delete popup from widget tree.
                LoginAction::ClosePopup => {
                    if let Some(popup) = self.popup {
                        self.show_popup = false;
                        ctx.remove_child(popup);
                    }
                }
            }

            self.action = None;
        }
    }
}

widget!(LoginForm<LoginFormState>);

impl Template for LoginForm {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("LoginForm").child(
            Grid::new()
                .columns("64, 64")
                .rows("48, 48, 48, 48")
                .v_align("start")
                .h_align("center")
                .child(
                    TextBlock::new()
                        .text("Please login to continue !")
                        .font_size(18.0)
                        .v_align("center")
                        .h_align("start")
                        .attach(Grid::column(0))
                        .attach(Grid::row(0))
                        .attach(Grid::column_span(2))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .text("Username:")
                        .v_align("center")
                        .h_align("start")
                        .attach(Grid::column(0))
                        .attach(Grid::row(1))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .text("Password:")
                        .v_align("center")
                        .h_align("start")
                        .attach(Grid::column(0))
                        .attach(Grid::row(2))
                        .build(ctx),
                )
                .child(
                    TextBox::new()
                        .id(USERNAME_INPUT_ID)
                        .water_mark("Username")
                        .v_align("center")
                        .h_align("start")
                        .attach(Grid::column(1))
                        .attach(Grid::row(1))
                        .max_width(128.0)
                        .build(ctx),
                )
                .child(
                    PasswordBox::new()
                        .id(PASSWORD_INPUT_ID)
                        .water_mark("Password")
                        .v_align("center")
                        .h_align("start")
                        .attach(Grid::column(1))
                        .attach(Grid::row(2))
                        .max_width(128.0)
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .text("Login")
                        .v_align("center")
                        .h_align("end")
                        .attach(Grid::column(1))
                        .attach(Grid::row(3))
                        // send a new Authenticate action to LoginFormState
                        .on_click(move |states, _| -> bool {
                            states.get_mut::<LoginFormState>(id).authenticate();
                            false
                        })
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
                .title("OrbTk - PasswordBox example")
                .position((125.0, 125.0))
                .size(468.0, 730.0)
                .resizeable(true)
                .child(LoginForm::new().build(ctx))
                .build(ctx)
        })
        .run();
}
