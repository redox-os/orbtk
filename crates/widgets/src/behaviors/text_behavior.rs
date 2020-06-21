use super::MouseBehavior;

use crate::{
    prelude::*,
    shell::{Key, KeyEvent}
};

#[derive(Clone)]
enum TextBehaviorAction {
    Key(KeyEvent),
    Mouse(Mouse),
}

#[derive(Default, AsAny)]
pub struct TextBehaviorState {
    action: Option<TextBehaviorAction>,
    cursor: Entity,
    len: usize
}

impl TextBehaviorState {
    fn action(&mut self, action: TextBehaviorAction) {
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
        }
    }

     // Get new position for the caret based on current mouse position
     fn get_new_caret_position(&self, ctx: &mut Context, p: Mouse) -> usize {
        if let Some((index, _x)) = self
            .map_chars_index_to_position(ctx)
            .iter()
            .min_by_key(|(_index, x)| (p.x - x).abs() as u64)
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
            ctx.widget().get::<Point>("position").x + ctx.widget().get::<Thickness>("padding").left;
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

    fn select_all(&self, ctx: &mut Context) {
        let len = ctx.widget().get::<String16>("text").len();
        ctx.widget()
            .get_mut::<TextSelection>("text_selection")
            .start_index = 0;
        ctx.widget()
            .get_mut::<TextSelection>("text_selection")
            .length = len;
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

    fn check_outside_update(&self, ctx: &mut Context) {
        let len = ctx.widget().get::<String16>("text").len();
        if self.len != len && self.len > len {
            self.reset(ctx);
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

impl State for TextBehaviorState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.cursor = Entity::from(ctx
            .widget().clone::<u32>("cursor"));
            //.expect("TextBoxState.init: cursor child could not be found.");
        self.len = ctx.widget().get::<String16>("text").len();
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.check_outside_update(ctx);

        if let Some(action) = self.action.clone() {
            match action {
                TextBehaviorAction::Key(event) => {
                    self.handle_key_event(event, ctx);
                }
                TextBehaviorAction::Mouse(p) => {
                    self.request_focus(ctx, p);
                }
            }

            self.action = None;
            ctx.widget().update_theme_by_state(false);
        }

        self.len = ctx.widget().get::<String16>("text").len();
    }
}

widget!(
    TextBehavior<TextBehaviorState>: ChangedHandler, KeyDownHandler {
        cursor: u32,

        /// Sets or shares the font size property.
        font_size: f64,

        /// Sets or shares the font property.
        font: String,

        /// Sets or shares the focused property.
        focused: bool,

        /// Sets or shares ta value that describes if the text input should lost focus on activation (enter).
        lost_focus_on_activation: bool,

        /// Sets or shares the padding property.
        padding: Thickness,

        /// Sets or shares the text property.
        text: String16,

        /// Sets or shares the text selection property.
        text_selection: TextSelection,

        /// Sets or shares the watermark text property.
        water_mark: String16
    }
);

impl Template for TextBehavior {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("TextBehavior")
        .focused(false)
        .font("Roboto Regular")
        .font_size(fonts::FONT_SIZE_12)
        .lost_focus_on_activation(true)
        .padding(4.0)
        .text("")
        .text_selection(TextSelection::default())
        .water_mark("")
        .on_key_down(move |states, event| -> bool {
            states
                .get_mut::<TextBehaviorState>(id)
                .action(TextBehaviorAction::Key(event));
            false
        })
        .child(
            MouseBehavior::new()
                .visibility(id)
                .enabled(id)
                .on_mouse_down(move |states, m| {
                    states
                        .get_mut::<TextBehaviorState>(id)
                        .action(TextBehaviorAction::Mouse(m));
                    true
                })
                .build(ctx)
            )
    }
}