use crate::{
    api::prelude::*,
    proc_macros::*,
    shell::prelude::{Key, KeyEvent},
    theme::fonts,
};

use super::MouseBehavior;

#[derive(Clone)]
enum TextAction {
    Key(KeyEvent),
    Mouse(Mouse),
    Drop(String, Point),
}

/// The `TextBehaviorState` handles the text processing of the `TextBehavior` widget.
#[derive(Default, AsAny)]
pub struct TextBehaviorState {
    action: Option<TextAction>,
    len: usize,
    cursor: Entity,
    target: Entity,
    focused: bool,
}

impl TextBehaviorState {
    fn action(&mut self, action: TextAction) {
        self.action = Some(action);
    }

    fn is_ctlr_home_pressed(&self, ctx: &mut Context) -> bool {
        if cfg!(target_os = "macos")
            && ctx
                .window()
                .get::<Global>("global")
                .keyboard_state
                .is_home_down()
        {
            return true;
        } else if ctx
            .window()
            .get::<Global>("global")
            .keyboard_state
            .is_ctrl_down()
        {
            return true;
        }

        false
    }

    fn copy(&self, registry: &mut Registry, ctx: &mut Context) {
        let text_selection: TextSelection = ctx.get_widget(self.target).clone("text_selection");

        if text_selection.length == 0 {
            return;
        }

        if let Some(text_part) = ctx
            .get_widget(self.target)
            .clone::<String16>("text")
            .get_string(
                text_selection.start_index,
                text_selection.start_index + text_selection.length,
            )
        {
            registry.get_mut::<Clipboard>("clipboard").set(text_part);
        }
    }

    fn paste(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.clear_selection(ctx);
        let index = ctx
            .get_widget(self.target)
            .get::<TextSelection>("text_selection")
            .start_index;

        let mut text = ctx.widget().clone::<String16>("text");
        let mut len = 0;
        if let Some(value) = registry.get::<Clipboard>("clipboard").get() {
            len = value.len();
            text.insert_str(index, value.as_str());
        }

        ctx.get_widget(self.target).set("text", text);
        ctx.get_widget(self.target)
            .get_mut::<TextSelection>("text_selection")
            .start_index = index + len;
    }

    fn request_focus(&self, ctx: &mut Context, p: Mouse) {
        ctx.push_event_by_window(FocusEvent::RequestFocus(self.target));

        // select all text if there is text and the element is not focused yet.
        if !ctx.widget().get::<String16>("text").is_empty()
            && !(*ctx.widget().get::<bool>("focused"))
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
        let text: String16 = ctx.widget().clone("text");
        // start x position of the cursor is start position of the text element + padding left
        let start_position: f64 = ctx.widget().get::<Point>("position").x()
            + ctx.get_widget(self.target).get::<Thickness>("padding").left;
        // array which will hold char index and it's x position
        let mut position_index: Vec<(usize, f64)> = Vec::with_capacity(text.len());
        position_index.push((0, start_position));
        // current text font family and size
        let font: String = ctx.widget().clone_or_default::<String>("font");
        let font_size: f64 = ctx.widget().clone_or_default::<f64>("font_size");

        for i in 0..text.len() {
            let bound_width: f64 = ctx
                .render_context_2_d()
                .measure(
                    &text.get_string(0, i + 1).unwrap().as_str(),
                    font_size,
                    &font,
                )
                .width;
            let next_position: f64 = start_position + bound_width;

            position_index.push((i + 1, next_position));
        }

        // for (index, _) in text.chars().u.enumerate() {}

        position_index
    }

    // Reset selection and offset if text is changed from outside
    fn reset(&self, ctx: &mut Context) {
        ctx.widget().set("text_selection", TextSelection::default());
    }

    fn check_outside_update(&self, ctx: &mut Context) {
        let len = ctx.widget().get::<String16>("text").len();
        if self.len != len && self.len > len {
            self.reset(ctx);
        }
    }

    fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        registry: &mut Registry,
        ctx: &mut Context,
    ) {
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
            Key::C(..) => {
                if self.is_ctlr_home_pressed(ctx) {
                    self.copy(registry, ctx);
                } else {
                    self.insert_char(key_event, ctx);
                }
            }
            Key::V(..) => {
                if self.is_ctlr_home_pressed(ctx) {
                    self.paste(registry, ctx);
                } else {
                    self.insert_char(key_event, ctx);
                }
            }
            Key::A(..) => {
                if self.is_ctlr_home_pressed(ctx) {
                    self.select_all(ctx);
                } else {
                    self.insert_char(key_event, ctx);
                }
            }
            _ => {
                self.insert_char(key_event, ctx);
            }
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
        let mut text = ctx.widget().clone::<String16>("text");

        for i in (selection.start_index..(selection.start_index + selection.length)).rev() {
            text.remove(i);
        }

        ctx.get_widget(self.target).set("text", text);

        ctx.widget()
            .get_mut::<TextSelection>("text_selection")
            .length = 0;

        ctx.get_widget(self.cursor).set("expanded", false);
    }

    fn back_space(&mut self, ctx: &mut Context) {
        if *ctx.get_widget(self.cursor).get::<bool>("expanded") {
            self.clear_selection(ctx);
        } else {
            let index = ctx
                .widget()
                .clone::<TextSelection>("text_selection")
                .start_index;
            if index > 0 {
                let mut text = ctx.widget().clone::<String16>("text");
                text.remove(index - 1);
                ctx.get_widget(self.target).set("text", text);
                ctx.widget()
                    .get_mut::<TextSelection>("text_selection")
                    .start_index = index - 1;
            }
        }
    }

    fn delete(&mut self, ctx: &mut Context) {
        if *ctx.get_widget(self.cursor).get::<bool>("expanded") {
            self.clear_selection(ctx);
        } else {
            let index = ctx
                .widget()
                .clone::<TextSelection>("text_selection")
                .start_index;
            if index < ctx.widget().get::<String16>("text").len() {
                let mut text = ctx.widget().clone::<String16>("text");
                text.remove(index);
                ctx.get_widget(self.target).set("text", text);

                ctx.widget()
                    .get_mut::<TextSelection>("text_selection")
                    .start_index = index;
            }
        }
    }

    fn activate(&self, ctx: &mut Context) {
        if *ctx.widget().get::<bool>("lost_focus_on_activation") {
            ctx.push_event_by_window(FocusEvent::RemoveFocus(ctx.entity));
        }

        ctx.push_event_strategy_by_entity(
            ActivateEvent(self.target),
            self.target,
            EventStrategy::Direct,
        )
    }

    fn insert_char(&mut self, key_event: KeyEvent, ctx: &mut Context) {
        if key_event.text.is_empty() {
            return;
        }

        if *ctx.get_widget(self.cursor).get::<bool>("expanded") {
            ctx.get_widget(self.target)
                .set("text", String16::from(key_event.text));
            if let Some(selection) = ctx
                .get_widget(self.cursor)
                .try_get_mut::<TextSelection>("text_selection")
            {
                selection.start_index = 1;
                selection.length = 0
            }
            ctx.get_widget(self.cursor).set("expanded", false);
        } else {
            let current_selection = *ctx
                .get_widget(self.cursor)
                .get::<TextSelection>("text_selection");

            let mut text = ctx.widget().clone::<String16>("text");
            text.insert_str(current_selection.start_index, key_event.text.as_str());
            ctx.get_widget(self.target).set("text", text);

            if let Some(selection) = ctx
                .get_widget(self.cursor)
                .try_get_mut::<TextSelection>("text_selection")
            {
                selection.start_index =
                    current_selection.start_index + key_event.text.encode_utf16().count();
            }
        }
    }
}

impl State for TextBehaviorState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.cursor = Entity::from(
            ctx.widget()
                .try_clone::<u32>("cursor")
                .expect("TextBehaviorState.init: cursor could not be found."),
        );
        self.target = Entity::from(
            ctx.widget()
                .try_clone::<u32>("target")
                .expect("TextBehaviorState.init: target could not be found."),
        );
        self.len = ctx.widget().get::<String16>("text").len();
        self.focused = *ctx.widget().get::<bool>("focused");

        if self.len == 0 {
            ctx.get_widget(self.target)
                .get_mut::<Selector>("selector")
                .set_state("empty");
            ctx.get_widget(self.target).update(false);
        }
    }

    fn update(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.check_outside_update(ctx);

        let focused = *ctx.widget().get::<bool>("focused");
        let empty = ctx.widget().get::<String16>("text").is_empty();

        if !focused
            && empty
            && !ctx
                .get_widget(self.target)
                .get::<Selector>("selector")
                .has_state("empty")
        {
            ctx.get_widget(self.target)
                .get_mut::<Selector>("selector")
                .set_state("empty");
            ctx.get_widget(self.target).update(false);
        }

        if !focused && *ctx.widget().get::<bool>("request_focus") {
            ctx.widget().set("request_focus", false);
            ctx.push_event_by_window(FocusEvent::RequestFocus(ctx.entity));
            self.select_all(ctx);
        }

        if self.focused != *ctx.widget().get::<bool>("focused") {
            self.focused = *ctx.widget().get::<bool>("focused");
        }

        if let Some(action) = self.action.clone() {
            match action {
                TextAction::Key(event) => {
                    self.handle_key_event(event, registry, ctx);
                }
                TextAction::Mouse(p) => {
                    self.request_focus(ctx, p);
                }
                TextAction::Drop(text, position) => {
                    println!("text drop file {:?}", position);
                    if check_mouse_condition(position, &ctx.get_widget(self.target)) {
                        println!("Drop {}", text);
                    }
                }
            }

            self.action = None;
            ctx.get_widget(self.target).update(false);
        }

        self.len = ctx.widget().get::<String16>("text").len();

        if self.len == 0 && self.focused {
            ctx.get_widget(self.target)
                .get_mut::<Selector>("selector")
                .set_state("empty_focused");
            ctx.get_widget(self.target).update(false);
        } else if self.len > 0 && self.focused {
            ctx.get_widget(self.target)
                .get_mut::<Selector>("selector")
                .set_state("focused");
            ctx.get_widget(self.target).update(false);
        }
    }
}

widget!(
    /// The TextBehavior widget shares the same logic of handling text input between
    /// tex-related widgets.
    ///
    /// Attaching to a widget makes it able to handle text input like:
    /// * input characters by keyboard
    /// * select all text with Ctrl+A key combination
    /// * delete selected text with Backspace or Delete
    /// * move cursor by the left or right arrow keys or clicking with mouse
    /// * delete characters by pressing the Backspace or the Delete key
    /// * run on_activate() callback on pressing the Enter key
    ///
    /// TextBehavior needs the following prerequisites to able to work:
    /// * a `cursor`: the [`Entity`] of a [`Cursor`] widget
    /// * a target: the [`Entity`] of the target widget
    ///
    /// * and must inherit the following properties from its target:
    ///     * focused
    ///     * font
    ///     * font_size
    ///     * lost_focus_on_activation
    ///     * request_focus
    ///     * text
    ///     * text_selection
    ///
    /// # Example
    ///
    /// ```
    /// use orbtk::prelude::*
    ///
    /// widget!(MyInput {
    ///     // essential properties TextBehavior needs to inherit
    ///     focused: bool,
    ///     font: String,
    ///     font_size: f64,
    ///     lost_focus_on_activation: bool,
    ///     request_focus: bool,
    ///     text_selection: TextSelection
    /// });
    ///
    /// impl Template for MyInput {
    ///     fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
    ///         // Cursor depends on a TextBlock
    ///         let text_block = TextBlock::new()
    ///             .text(id)
    ///             .water_mark(id)
    ///             .font(id)
    ///             .font_size(id)
    ///             .build(ctx);
    ///
    ///         let cursor = Cursor::new()
    ///            // use .0 because Entity wraps an u32
    ///            .text_block(text_block.0)
    ///            .focused(id)
    ///            .text_selection(id)
    ///            .build(ctx);
    ///
    ///        let text_behavior = TextBehavior::new()
    ///            .cursor(cursor.0)
    ///            .focused(id)
    ///            .font(id)
    ///            .font_size(id)
    ///            .lost_focus_on_activation(id)
    ///            .target(id.0)
    ///            .request_focus(id)
    ///            .text(id)
    ///            .text_selection(id)
    ///            .build(ctx);
    ///
    ///        self.child(cursor)
    ///            .child(text_behavior)
    /// }
    /// ```
    ///
    /// [`Entity`]: https://docs.rs/dces/0.2.0/dces/entity/struct.Entity.html
    /// [`Cursor`]: ../struct.Cursor.html
    TextBehavior<TextBehaviorState>: ActivateHandler, KeyDownHandler, DropHandler {
        /// Sets or shares the entity of the Cursor widget property.
        cursor: u32,

        /// Sets or shares the focused property.
        focused: bool,

        /// Sets or shares the font property.
        font: String,

        /// Sets or shares the font size property.
        font_size: f64,

        /// Sets or shares ta value that describes if the widget should lost focus on activation (when Enter pressed).
        lost_focus_on_activation: bool,

        /// Sets or shares the entity of the target widget.
        target: u32,

        /// Sets or shares the request_focus property. Used to request focus from outside.Set to `true` to request focus.
        request_focus: bool,

        /// Sets or shares the text property.
        text: String16,

        /// Sets or shares the text selection property.
        text_selection: TextSelection
    }
);

impl Template for TextBehavior {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("TextBehavior")
            .font_size(fonts::FONT_SIZE_12)
            .font("Roboto-Regular")
            .text("")
            .text_selection(TextSelection::default())
            .focused(false)
            .lost_focus_on_activation(true)
            .child(
                MouseBehavior::new()
                    .visibility(id)
                    .enabled(id)
                    .on_mouse_down(move |states, m| {
                        states
                            .get_mut::<TextBehaviorState>(id)
                            .action(TextAction::Mouse(m));
                        true
                    })
                    .build(ctx),
            )
            .on_key_down(move |states, event| -> bool {
                states
                    .get_mut::<TextBehaviorState>(id)
                    .action(TextAction::Key(event));
                false
            })
            .on_drop_file(move |states, file_name, position| {
                states
                    .get_mut::<TextBehaviorState>(id)
                    .action(TextAction::Drop(file_name, position));
                true
            })
            .on_drop_text(move |states, file_name, position| {
                states
                    .get_mut::<TextBehaviorState>(id)
                    .action(TextAction::Drop(file_name, position));
                true
            })
    }
}
