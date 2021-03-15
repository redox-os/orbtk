use orbtk::{
    {api::prelude::*, proc_macros::*},
    prelude::behaviors::TextBehavior,
    prelude::*,
};

//super::behaviors::{TextAction, TextBehavior, TextResult};

// --- CONSTATNS ---
pub static ID_HEADER: &str = "Text behavior example";
pub static ID_MY_INPUT: &str = "my_input_widget";

widget!(MyInput: TextInputHandler {
    /// MyInput is an example widget that inherits its functionality from
    /// `TextBehavior`.
    ///
    /// To use the text handling functions, you need to define essential
    /// properties that inherit its values from the parent (here:
    /// `TextBehavior`). Interaction on the user input is offered via the
    /// `TextInputHandler`.

    /// Sets or shares the background property.
    background: Brush,

    /// Sets or shares the border brush property.
    border_brush: Brush,

    /// Sets or shares the border radius property.
    border_radius: f64,

    /// Sets or shares the border thickness property.
    border_width: Thickness,

    /// Sets or shares the focused property.
    focused: bool,

    /// Sets or shares the font property.
    font: String,

    /// Sets or shares the font size property.
    font_size: f64,

    /// Sets or shares the foreground property.
    foreground: Brush,

    /// Support line wrapping using Ctrl-Enter key.
    line_wrap: bool,

    /// Sets or shares ta value that describes if the widget should
    /// lose focus on activation (when Enter pressed).
    lose_focus_on_activation: bool,

    /// Sets or shares the padding property.
    padding: Thickness,

    /// Used to request focus from outside. Set to `true` tor request focus.
    request_focus: bool,

    /// Sets or shares the text selection property.
    selection: TextSelection,

    /// If set to `true` all character will be focused when the widget gets focus. Default is `true`
    select_all_on_focus: bool,

    /// Sets or shares the text property.It holds the password.
    text: String,

    /// Sets or shares the water_mark text property.
    water_mark: String
});

impl Template for MyInput {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        // TextBlock widget is our target (a child of TextBehaviour)
        let text_block = TextBlock::new()
            .font_size(id)
            .foreground(id)
            .text(id)
            .water_mark(id)
            .build(ctx);

        // Cursor widget is a child of TextBehavior
        let cursor = Cursor::new()
            .selection(id)
            .build(ctx);

        // TextBehavior widget is the parent
        let text_behavior = TextBehavior::new()
            //.style(STYLE_TEXT_BLOCK)
            .style(STYLE_TEXT_BOX)
            .cursor(cursor.0)
            .focused(id)
            .font(id)
            .font_size(id)
            .h_align("center")
            .line_wrap(id)
            .lose_focus_on_activation(id)
            .request_focus(id)
            .selection(id)
            .select_all_on_focus(id)
            .target(id.0)
            .text(id)
            .text_block(text_block.0)
            .build(ctx);

        self.name(ID_MY_INPUT)
            .child(
                Container::new()
                    .background(id)
                    .border_brush(id)
                    .border_radius(id)
                    .border_width(id)
                    .padding(id)
                    .v_align("center")
                    .child(text_behavior)
                    .child(
                        Stack::new()
                            .orientation("Vertical")
                            .spacing(6)
                            .child(cursor)
                            .child(text_block)
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            // let text_block = MyInput::new()
            //     //.id(ID_MY_INPUT)
            //     .background(colors::LYNCH_COLOR)
            //     //.request_focus(true)
            //     .text("")
            //     .water_mark("Input goes here ...")
            //     .build(ctx);

            Window::new()
                .title("OrbTk - text behaviour example")
                .position((100.0, 100.0))
                .size(420.0, 220.0)
                .resizeable(true)
                .child(
                    Container::new()
                        .margin(8)
                        .child(
                            Stack::new()
                                .orientation("vertical")
                                .spacing(8)
                                //.max_height(120)
                                //.max_width(300)
                                //.clip(true)
                                .child(MyInput::new()
                                       .style("header")
                                       //.max_height(120)
                                       //.max_width(300)
                                       .text(ID_HEADER)
                                       .build(ctx))
                                .child(TextBox::new()
                                       .line_wrap(false)
                                       .min_height(80)
                                       .max_height(140)
                                       .max_width(355)
                                       .water_mark("Singleline text ...")
                                       .build(ctx))
                                .child(TextBox::new()
                                       .line_wrap(true)
                                       .min_height(80)
                                       .max_height(140)
                                       .max_width(355)
                                       .water_mark("Multi line text (Delimiter: Ctrl-Enter) ...")
                                       .build(ctx))
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx)
        })
        .run();
}
