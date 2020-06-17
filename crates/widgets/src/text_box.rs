use super::behaviors::MouseBehavior;
use crate::{
    prelude::*,
    shell::{Key, KeyEvent},
};

// --- KEYS --

pub static ELEMENT_TEXT_BOX: &'static str = "text_box";

static ID_CURSOR: &'static str = "id_cursor";

// --- KEYS --

#[derive(Clone)]
enum TextBoxAction {
    Key(KeyEvent),
    Mouse(Mouse),
}

/// The `TextBoxState` handles the text processing of the `TextBox` widget.
#[derive(Default, AsAny)]
pub struct TextBoxState {
    action: Option<TextBoxAction>,
    len: usize,
    cursor: Entity,
}

impl TextBoxState {
    fn action(&mut self, action: TextBoxAction) {
        self.action = Some(action);
    }
}

impl State for TextBoxState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
    }
}

widget!(
    /// The `TextBox` widget represents a single line text input widget.
    ///
    /// * CSS element: `text_box`
    TextBox<TextBoxState>: ActivateHandler, ChangedHandler, KeyDownHandler {
        /// Sets or shares the text property.
        text: String16,

        /// Sets or shares the water_mark text property.
        water_mark: String16,

        /// Sets or shares the text selection property.
        text_selection: TextSelection,

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

        /// Sets or shares ta value that describes if the TextBox should lost focus on activation (enter).
        lost_focus_on_activation: bool
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
            .build(ctx);

        self.name("TextBox")
            .element(ELEMENT_TEXT_BOX)
            .text("")
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(fonts::FONT_SIZE_12)
            .font("Roboto Regular")
            .text_selection(TextSelection::default())
            .padding(4.0)
            .background(colors::LYNCH_COLOR)
            .border_brush("transparent")
            .border_width(0.0)
            .border_radius(2.0)
            .min_width(128.0)
            .height(32.0)
            .focused(false)
            .lost_focus_on_activation(true)
            .child(
                MouseBehavior::new()
                    .visibility(id)
                    .enabled(id)
                    .on_mouse_down(move |states, m| {
                        states
                            .get_mut::<TextBoxState>(id)
                            .action(TextBoxAction::Mouse(m));
                        true
                    })
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
                                    // It is important that cursor is the first child
                                    // should be refactored in the future.
                                    .child(
                                        Cursor::new()
                                            .id(ID_CURSOR)
                                            .h_align("start")
                                            .text_block(text_block.0)
                                            .focused(id)
                                            .text_selection(id)
                                            .build(ctx),
                                    )
                                    .child(text_block)
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .on_key_down(move |states, event| -> bool {
                states
                    .get_mut::<TextBoxState>(id)
                    .action(TextBoxAction::Key(event));
                false
            })
    }
}
