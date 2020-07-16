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
    focused: bool,
}

impl TextBoxState {
    fn action(&mut self, action: TextBoxAction) {
        self.action = Some(action);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent, ctx: &mut Context) {
        if !ctx.widget().get::<bool>("focused") {
            return;
        }

        match key_event.key {
            Key::Left => {
                self.move_cursor_left(ctx);
            }
            Key::Right => {
                self.move_cursor_right(ctx);
            }
            Key::Backspace => {
                self.back_space(ctx);
            }
            Key::Delete => {
                self.delete(ctx);
            }
            Key::Enter => {
                self.activate(ctx);
            }
            Key::A(..) => {
                // if cfg!(mac_os) {
                //     if ctx
                //         .window()
                //         .get::<Global>("global")
                //         .keyboard_state
                //         .is_home_down()
                //     {
                //         self.select_all(ctx);
                //     } else {
                //         self.insert_char(key_event, ctx);
                //     }
                // } else {
                if ctx
                    .window()
                    .get::<Global>("global")
                    .keyboard_state
                    .is_ctrl_down()
                {
                    self.select_all(ctx);
                } else {
                    self.insert_char(key_event, ctx);
                }
                // }
            }
            _ => {
                self.insert_char(key_event, ctx);
            }
        }
    }

    fn request_focus(&self, ctx: &mut Context, p: Mouse) {
        ctx.push_event_by_window(FocusEvent::RequestFocus(ctx.entity));

        // select all text if there is text and the element is not focused yet.
        if ctx.widget().get::<String16>("text").len() > 0 && !(*ctx.widget().get::<bool>("focused"))
        {
            self.select_all(ctx);
            return;
        }

        // change only the caret position if the text is already selected or if the element is focused already
        if *ctx.get_widget(self.cursor).get::<bool>("expanded")
            || *ctx.widget().get::<bool>("focused")
        {
            ctx.widget()
                .get_mut::<TextSelection>("text_selection")
                .start_index = self.get_new_caret_position(ctx, p);
            ctx.widget()
                .get_mut::<TextSelection>("text_selection")
                .length = 0;

            ctx.get_widget(self.cursor).set("expanded", false);
        }
    }

    // Get new position for the caret based on current mouse position
    fn get_new_caret_position(&self, ctx: &mut Context, p: Mouse) -> usize {
        if let Some((index, _x)) = self
            .map_chars_index_to_position(ctx)
            .iter()
            .min_by_key(|(_index, x)| (p.position.x() - x).abs() as u64)
        {
            return *index;
        }

        0
    }

    // Returns a vector with a tuple of each char's starting index (usize) and position (f64)
    fn map_chars_index_to_position(&self, ctx: &mut Context) -> Vec<(usize, f64)> {
        let text: String = ctx.widget().get::<String16>("text").as_string();
        // start x position of the cursor is start position of the text element + padding left
        let start_position: f64 =
            ctx.widget().get::<Point>("position").x() + ctx.widget().get::<Thickness>("padding").left;
        // array which will hold char index and it's x position
        let mut position_index: Vec<(usize, f64)> = Vec::with_capacity(text.len());
        position_index.push((0, start_position));
        // current text font family and size
        let font: String = ctx.widget().clone_or_default::<String>("font");
        let font_size: f64 = ctx.widget().clone_or_default::<f64>("font_size");

        for (index, _) in text.chars().enumerate() {
            let bound_width: f64 = ctx
                .render_context_2_d()
                .measure(&text[..index + 1], font_size, &font)
                .width;
            let next_position: f64 = start_position + bound_width;

            position_index.push((index + 1, next_position));
        }

        position_index
    }

    // Reset selection and offset if text is changed from outside
    fn reset(&self, ctx: &mut Context) {
        ctx.widget().set("text_selection", TextSelection::default());
        ctx.push_event_strategy_by_entity(
            ChangedEvent(ctx.entity),
            ctx.entity,
            EventStrategy::Direct,
        );
    }

    fn check_outside_update(&self, ctx: &mut Context) {
        let len = ctx.widget().get::<String16>("text").len();
        if self.len != len && self.len > len {
            self.reset(ctx);
        }
    }

    fn select_all(&self, ctx: &mut Context) {
        let len = ctx.widget().get::<String16>("text").len();
        ctx.widget()
            .get_mut::<TextSelection>("text_selection")
            .start_index = 0;
        ctx.widget()
            .get_mut::<TextSelection>("text_selection")
            .length = len;
        ctx.get_widget(self.cursor).set("expanded", len > 0);
    }

    fn move_cursor_left(&mut self, ctx: &mut Context) {
        if *ctx.get_widget(self.cursor).get::<bool>("expanded") {
            if let Some(selection) = ctx
                .get_widget(self.cursor)
                .try_get_mut::<TextSelection>("text_selection")
            {
                selection.start_index = 0;
                selection.length = 0;
            }
        }

        if let Some(selection) = ctx
            .get_widget(self.cursor)
            .try_get_mut::<TextSelection>("text_selection")
        {
            selection.start_index = (selection.start_index as i32 - 1).max(0) as usize;
            selection.length = 0;
        }

        ctx.get_widget(self.cursor).set("expanded", false);
    }

    fn move_cursor_right(&mut self, ctx: &mut Context) {
        let text_len = ctx.widget().get::<String16>("text").len();

        if *ctx.get_widget(self.cursor).get::<bool>("expanded") {
            if let Some(selection) = ctx
                .get_widget(self.cursor)
                .try_get_mut::<TextSelection>("text_selection")
            {
                selection.start_index = text_len;
                selection.length = 0;
            }

            ctx.get_widget(self.cursor).set("expanded", false);

            return;
        }

        if let Some(selection) = ctx
            .get_widget(self.cursor)
            .try_get_mut::<TextSelection>("text_selection")
        {
            if selection.start_index < text_len {
                selection.start_index = (selection.start_index + 1).min(text_len);
            }
            selection.length = 0;
        }

        ctx.get_widget(self.cursor).set("expanded", false);
    }

    fn clear_selection(&mut self, ctx: &mut Context) {
        let selection = ctx.widget().clone::<TextSelection>("text_selection");

        if let Some(text) = ctx.widget().try_get_mut::<String16>("text") {
            for i in (selection.start_index..(selection.start_index + selection.length)).rev() {
                text.remove(i);
            }
        }

        ctx.widget()
            .get_mut::<TextSelection>("text_selection")
            .length = 0;

        ctx.get_widget(self.cursor).set("expanded", false);
    }

    fn back_space(&mut self, ctx: &mut Context) {
        let mut changed = false;

        if *ctx.get_widget(self.cursor).get::<bool>("expanded") {
            self.clear_selection(ctx);
            changed = true;
        } else {
            let index = ctx
                .widget()
                .clone::<TextSelection>("text_selection")
                .start_index;
            if index > 0 {
                ctx.widget().get_mut::<String16>("text").remove(index - 1);
                ctx.widget()
                    .get_mut::<TextSelection>("text_selection")
                    .start_index = index - 1;

                changed = true;
            }
        }

        if changed {
            ctx.push_event_strategy_by_entity(
                ChangedEvent(ctx.entity),
                ctx.entity,
                EventStrategy::Direct,
            );
        }
    }

    fn delete(&mut self, ctx: &mut Context) {
        let mut changed = false;

        if *ctx.get_widget(self.cursor).get::<bool>("expanded") {
            self.clear_selection(ctx);
        } else {
            let index = ctx
                .widget()
                .clone::<TextSelection>("text_selection")
                .start_index;
            if index < ctx.widget().get::<String16>("text").len() {
                ctx.widget().get_mut::<String16>("text").remove(index);
                changed = true;

                ctx.widget()
                    .get_mut::<TextSelection>("text_selection")
                    .start_index = index;
            }
        }

        if changed {
            ctx.push_event_strategy_by_entity(
                ChangedEvent(ctx.entity),
                ctx.entity,
                EventStrategy::Direct,
            );
        }
    }

    fn activate(&self, ctx: &mut Context) {
        if *ctx.widget().get::<bool>("lost_focus_on_activation") {
            ctx.push_event_by_window(FocusEvent::RemoveFocus(ctx.entity));
        }

        ctx.push_event_strategy_by_entity(
            ActivateEvent(ctx.entity),
            ctx.entity,
            EventStrategy::Direct,
        )
    }

    fn insert_char(&mut self, key_event: KeyEvent, ctx: &mut Context) {
        if key_event.text.is_empty() {
            return;
        }

        if *ctx.get_widget(self.cursor).get::<bool>("expanded") {
            ctx.widget().set("text", String16::from(key_event.text));
            if let Some(selection) = ctx
                .get_widget(self.cursor)
                .try_get_mut::<TextSelection>("text_selection")
            {
                selection.start_index = 1;
                selection.length = 0
            }
        } else {
            let current_selection = *ctx
                .get_widget(self.cursor)
                .get::<TextSelection>("text_selection");
            ctx.widget()
                .get_mut::<String16>("text")
                .insert_str(current_selection.start_index, key_event.text.as_str());

            if let Some(selection) = ctx
                .get_widget(self.cursor)
                .try_get_mut::<TextSelection>("text_selection")
            {
                selection.start_index =
                    current_selection.start_index + key_event.text.encode_utf16().count();
            }
        }

        ctx.push_event_strategy_by_entity(
            ChangedEvent(ctx.entity),
            ctx.entity,
            EventStrategy::Direct,
        );
    }
}

impl State for TextBoxState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.cursor = ctx
            .entity_of_child(ID_CURSOR)
            .expect("TextBoxState.init: cursor child could not be found.");
        self.len = ctx.widget().get::<String16>("text").len();
        self.focused = *ctx.widget().get::<bool>("focused");

        if self.len == 0 {
            ctx.widget()
                .get_mut::<Selector>("selector")
                .set_state("empty");
            ctx.widget().update(false);
        }
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.check_outside_update(ctx);

        if self.focused != *ctx.widget().get::<bool>("focused") {
            self.focused = *ctx.widget().get::<bool>("focused");
        }

        if let Some(action) = self.action.clone() {
            match action {
                TextBoxAction::Key(event) => {
                    self.handle_key_event(event, ctx);
                }
                TextBoxAction::Mouse(p) => {
                    self.request_focus(ctx, p);
                }
            }

            self.action = None;
            ctx.widget().update(false);
        }

        self.len = ctx.widget().get::<String16>("text").len();

        if self.len == 0 && self.focused {
            ctx.widget()
                .get_mut::<Selector>("selector")
                .set_state("empty_focused");
            ctx.widget().update(false);
        } else if self.len > 0 && self.focused {
            ctx.widget()
                .get_mut::<Selector>("selector")
                .set_state("focused");
            ctx.widget().update(false);
        }
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
            .style(ELEMENT_TEXT_BOX)
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
