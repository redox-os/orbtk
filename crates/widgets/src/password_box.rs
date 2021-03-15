use super::behaviors::{TextAction, TextBehavior, TextResult};

use crate::prelude::*;
use crate::{api::prelude::*, proc_macros::*, theme_default::prelude::*};

#[derive(Default, AsAny)]
struct PasswordBoxState {
    echo: char,
    text_behavior: Entity,
    text_block: Entity,
}

impl PasswordBoxState {
    fn mask(&mut self, ctx: &mut Context, text: String) {
        let mut new_prompt = String::new();

        for _ in text.chars() {
            new_prompt.push(self.echo);
        }

        TextBlock::text_set(&mut ctx.get_widget(self.text_block), new_prompt);
        ctx.send_message(TextAction::ForceUpdate(true), self.text_behavior);
    }
}

impl State for PasswordBoxState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.echo = ctx.widget().clone("echo");
    }

    fn messages(
        &mut self,
        mut messages: MessageReader,
        _registry: &mut Registry,
        ctx: &mut Context,
    ) {
        for message in messages.read::<TextResult>() {
            match message {
                TextResult::TextManipulated(text) => self.mask(ctx, text),
            }
        }
    }
}

widget!(
    /// The PasswordBox is a specialised [`TextBox`] masking its input.
    ///
    /// It is for use cases when users needs to enter sensitive data
    /// (like passwords, credit card PIN-codes, etc) that should not be readable directly on the display.
    /// PasswordBox masks its input with the `echo` char (the default value is an asterisk).
    ///
    /// The value typed in the `PasswordBox` can be obtained through the `text` property.
    /// You can process this value in your application and set the authentication logic as appropriate.
    /// It is a good practice to clear the content of the `text` property after the value is used.
    ///
    /// Notes:
    /// * If the input is empty, it will render the content of the `water_mark` property.
    /// * Changing the `echo` property after the `PasswordBox` is created has no effect.
    /// * The password is stored in plain text currently
    ///
    /// For an example how to use the PasswordBox, check the [`example`].
    ///
    /// [`TextBox`]: ./struct.TextBox.html
    /// [`example`]: https://github.com/redox-os/orbtk/tree/develop/examples/login.rs
    PasswordBox<PasswordBoxState>: KeyDownHandler, TextInputHandler {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the echo character which used to mask the input
        echo: char,

        /// Sets or shares the focused property.
        focused: bool,

        /// Sets or shares the font property.
        font: String,

        /// Sets or shares the font size property.
        font_size: f64,

        /// Sets or shares the foreground property.
        foreground: Brush,

        /// Indicates if the widget is hovered by the mouse cursor.
        hover: bool,

        /// Sets or shares ta value that describes if the PasswordBox
        /// should lose focus on activation (when Enter pressed).
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
    }
);

impl Template for PasswordBox {
    fn template(mut self, id: Entity, ctx: &mut BuildContext) -> Self {
        let text_block = TextBlock::new()
            .font(id)
            .font_size(id)
            .foreground(id)
            .h_align("start")
            .localizable(false)
            .v_align("center")
            .water_mark(id)
            .build(ctx);

        self.state_mut().text_block = text_block;

        let cursor = Cursor::new().selection(id).build(ctx);

        let text_behavior = TextBehavior::new()
            .cursor(cursor.0)
            .focused(id)
            .font(id)
            .font_size(id)
            .lose_focus_on_activation(id)
            .request_focus(id)
            .selection(id)
            .select_all_on_focus(id)
            .target(id.0)
            .text(id)
            .text_block(text_block.0)
            .build(ctx);

        self.state_mut().text_behavior = text_behavior;

        self.name("PasswordBox")
            .style(STYLE_TEXT_BOX)
            .background(colors::LYNCH_COLOR)
            .border_brush("transparent")
            .border_width(0.0)
            .border_radius(2.0)
            .echo('*')
            .focused(false)
            .font_size(fonts::FONT_SIZE_12)
            .font("Roboto-Regular")
            .foreground(colors::LINK_WATER_COLOR)
            .height(32.0)
            .lose_focus_on_activation(true)
            .min_width(128.0)
            .padding(4.0)
            .selection(TextSelection::default())
            .select_all_on_focus(true)
            .text("")
            .water_mark("Password")
            .child(text_behavior)
            .child(
                Container::new()
                    .background(id)
                    .border_brush(id)
                    .border_radius(id)
                    .border_width(id)
                    .padding(id)
                    .child(
                        Grid::new()
                            .clip(true)
                            .child(cursor)
                            .child(text_block)
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
