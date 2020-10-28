use super::behaviors::{TextAction, TextBehavior};

use crate::{api::prelude::*, prelude::*, proc_macros::*, theme_default::prelude::*};

// --- KEYS --
pub static STYLE_TEXT_BOX: &str = "text_box";
static ID_CURSOR: &str = "id_cursor";
// --- KEYS --

widget!(
    /// The `TextBox` widget represents a single line text input widget.
    ///
    /// * style: `text_box`
    TextBox: ActivateHandler,
    KeyDownHandler,
    TextInputHandler {
        /// Sets or shares the text property.
        text: String,

        /// Sets or shares the water_mark text property.
        water_mark: String,

        /// Sets or shares the text selection property.
        selection: TextSelection,

        /// Sets or shares the foreground property.
        foreground: Brush,

        /// Sets or shares the font size property.
        font_size: f64,

        /// Sets or shares the font property.
        font: String,

        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the padding property.
        padding: Thickness,

        /// Sets or shares the focused property.
        focused: bool,

        /// Sets or shares ta value that describes if the TextBox should lose focus on activation (enter).
        lose_focus_on_activation: bool,

        /// Used to request focus from outside. Set to `true` tor request focus.
        request_focus: bool,

        /// If set to `true` all character will be focused when the widget gets focus. Default is `true`
        select_all_on_focus: bool,

        /// Indicates if the widget is hovered by the mouse cursor.
        hover: bool
    }
);

impl Template for TextBox {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let text_block = TextBlock::new()
            .v_align("center")
            .h_align("start")
            .foreground(id)
            .text(id)
            .water_mark(id)
            .font(id)
            .font_size(id)
            .localizable(false)
            .build(ctx);

        let cursor = Cursor::new().id(ID_CURSOR).selection(id).build(ctx);

        let text_behavior = TextBehavior::new()
            .cursor(cursor.0)
            .target(id.0)
            .text_block(text_block.0)
            .focused(id)
            .font(id)
            .font_size(id)
            .lose_focus_on_activation(id)
            .select_all_on_focus(id)
            .request_focus(id)
            .text(id)
            .selection(id)
            .build(ctx);

        self.name("TextBox")
            .style(STYLE_TEXT_BOX)
            .text("")
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(fonts::FONT_SIZE_12)
            .font("Roboto-Regular")
            .selection(TextSelection::default())
            .padding(4.0)
            .background(colors::LYNCH_COLOR)
            .border_brush("transparent")
            .border_width(0.0)
            .border_radius(2.0)
            .min_width(128.0)
            .height(32.0)
            .focused(false)
            .lose_focus_on_activation(true)
            .select_all_on_focus(true)
            .child(text_behavior)
            .child(
                Container::new()
                    .background(id)
                    .border_radius(id)
                    .border_width(id)
                    .border_brush(id)
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
            .on_changed("text", move |ctx, _| {
                ctx.send_message(TextAction::ForceUpdate(false), text_behavior);
            })
    }
}
