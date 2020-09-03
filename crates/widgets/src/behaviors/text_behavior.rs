use crate::{
    api::prelude::*,
    proc_macros::*,
    render::TextMetrics,
    shell::prelude::{Key, KeyEvent},
    theme::fonts,
    Cursor, TextBlock,
};

// --- KEYS --
pub static EMPTY_STATE: &str = "empty";
pub static EMPTY_FOCUSED_STATE: &str = "empty_focused";
pub static FOCUSED_STATE: &str = "focused";
// --- KEYS --

use super::MouseBehavior;

// actions of TextBehaviorState
#[derive(Clone)]
enum TextAction {
    KeyDown(KeyEvent),
    MouseDown(Mouse),
    Drop(String, Point),
    FocusedChanged,
    SelectionChanged,
}

// helper enum
#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    None,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::None
    }
}

/// The `TextBehaviorState` handles the text processing of the `TextBehavior` widget.
#[derive(Default, AsAny)]
pub struct TextBehaviorState {
    action: Option<TextAction>,
    cursor: Entity,
    target: Entity,
    text_block: Entity,
    direction: Direction,
}

impl TextBehaviorState {
    fn action(&mut self, action: TextAction) {
        self.action = Some(action);
    }

    fn request_focus(&self, ctx: &mut Context) {
        ctx.push_event_by_window(FocusEvent::RequestFocus(ctx.entity));
    }

    // -- Text operations --

    fn cut(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.copy(registry, ctx);
        self.clear_selection(ctx);
    }

    fn copy(&self, registry: &mut Registry, ctx: &mut Context) {
        let selection = self.selection(ctx);

        let (start, end) = self.selection_start_end(selection);

        if selection.is_empty() {
            return;
        }

        if let Some(copy_text) =
            String16::from(TextBehavior::text_clone(&ctx.widget())).get_string(start, end)
        {
            registry.get_mut::<Clipboard>("clipboard").set(copy_text);
        }
    }

    fn paste(&mut self, registry: &mut Registry, ctx: &mut Context) {
        if let Some(value) = registry.get::<Clipboard>("clipboard").get() {
            self.insert_text(value, ctx);
        }
    }

    fn insert_text(&mut self, insert_text: String, ctx: &mut Context) {
        if insert_text.is_empty() {
            return;
        }

        let text_was_empty = self.len(ctx) == 0;

        let _ = self.clear_selection(ctx);

        let mut selection = self.selection(ctx);

        let mut text = TextBehavior::text_clone(&ctx.widget());
        text.insert_str(selection.start(), insert_text.as_str());

        selection.set(selection.start() + insert_text.chars().count());
        TextBehavior::selection_set(&mut ctx.widget(), selection);

        ctx.get_widget(self.target).set("text", text);

        // used to trigger bounds adjustments
        self.direction = Direction::Right;

        if text_was_empty {
            self.update_focused_state(ctx);
        }
    }

    // handle back space
    fn back_space(&mut self, ctx: &mut Context) {
        if self.clear_selection(ctx) {
            return;
        }

        let mut selection = self.selection(ctx);

        if selection.start() == 0 {
            return;
        }

        selection.set(selection.start() - 1);

        let mut text = String16::from(TextBehavior::text_clone(&ctx.widget()));

        text.remove(selection.start());

        ctx.get_widget(self.target).set("text", text.to_string());
        TextBehavior::selection_set(&mut ctx.widget(), selection);

        // used to trigger bounds adjustments
        self.direction = Direction::Left;

        if self.len(ctx) == 0 {
            self.update_focused_state(ctx);
        }
    }

    // handle delete
    fn delete(&mut self, ctx: &mut Context) {
        if self.clear_selection(ctx) {
            return;
        }

        let selection = self.selection(ctx);

        if selection.end() > self.len(ctx) {
            return;
        }

        let mut text = String16::from(TextBehavior::text_clone(&ctx.widget()));

        text.remove(selection.start() + 1);

        ctx.get_widget(self.target).set("text", text.to_string());
    }

    // clear all chars from the selection.
    fn clear_selection(&mut self, ctx: &mut Context) -> bool {
        let mut selection = self.selection(ctx);

        if selection.is_empty() {
            return false;
        }

        let mut text = String16::from(TextBehavior::text_clone(&ctx.widget()));

        let (start, end) = self.selection_start_end(selection);

        for i in (start..end).rev() {
            text.remove(i);
        }

        selection.set(start);

        ctx.get_widget(self.target).set("text", text.to_string());
        TextBehavior::selection_set(&mut ctx.widget(), selection);

        true
    }

    // -- Text operations --

    // -- Selection --

    fn update_cursor(&mut self, ctx: &mut Context) {
        let selection = self.selection(ctx);
        let (start, end) = self.selection_start_end(selection);

        let cursor_start_measure = self.measure(ctx, 0, selection.start());
        Cursor::cursor_x_set(&mut ctx.get_widget(self.cursor), cursor_start_measure.width);

        let start_measure = self.measure(ctx, 0, start);
        Cursor::selection_x_set(&mut ctx.get_widget(self.cursor), start_measure.width);
        let length_measure = self.measure(ctx, start, end);
        Cursor::selection_width_set(&mut ctx.get_widget(self.cursor), length_measure.width);

        // adjust position
        let offset = *Cursor::offset_ref(&ctx.get_widget(self.cursor));
        let width = Cursor::bounds_ref(&ctx.get_widget(self.cursor)).width();
        let delta = width - offset;

        if self.direction == Direction::Right && cursor_start_measure.width > delta {
            let offset_delta = delta - cursor_start_measure.width;
            Cursor::offset_set(&mut ctx.get_widget(self.cursor), offset + offset_delta);
            TextBlock::offset_set(&mut ctx.get_widget(self.text_block), offset + offset_delta);
        }

        if self.direction == Direction::Left && cursor_start_measure.width + offset < 0. {
            let offset_delta = cursor_start_measure.width + offset;
            Cursor::offset_set(&mut ctx.get_widget(self.cursor), offset - offset_delta);
            TextBlock::offset_set(&mut ctx.get_widget(self.text_block), offset - offset_delta);
        }

        self.direction = Direction::None;
    }

    fn select_all(&self, ctx: &mut Context) {
        if TextBehavior::text_ref(&ctx.widget()).is_empty()
            || !*TextBehavior::focused_ref(&ctx.widget())
        {
            return;
        }

        let mut selection = self.selection(ctx);
        selection.set_start(0);
        selection.set_end(self.len(ctx));

        TextBehavior::selection_set(&mut ctx.widget(), selection);
    }

    fn expand_selection_left(&mut self, ctx: &mut Context) {
        self.direction = Direction::Left;
        let mut selection = self.selection(ctx);
        if selection.start() as i32 > 0 {
            selection.set_start(selection.start() - 1);
        }
        TextBehavior::selection_set(&mut ctx.widget(), selection);
    }

    fn expand_selection_right(&mut self, ctx: &mut Context) {
        self.direction = Direction::Right;
        let mut selection = self.selection(ctx);
        if selection.start() < self.len(ctx) {
            selection.set_start(selection.start() + 1);
        }
        TextBehavior::selection_set(&mut ctx.widget(), selection);
    }

    fn move_selection_left(&mut self, ctx: &mut Context) {
        self.direction = Direction::Left;
        let selection = move_selection_left(self.selection(ctx));
        TextBehavior::selection_set(&mut ctx.widget(), selection);
    }

    fn move_selection_right(&mut self, ctx: &mut Context) {
        self.direction = Direction::Right;
        let selection = move_selection_right(self.selection(ctx), self.len(ctx));
        TextBehavior::selection_set(&mut ctx.widget(), selection);
    }

    // -- Selection --

    fn activate(&self, ctx: &mut Context) {
        if *ctx.widget().get::<bool>("lost_focus_on_activation") {
            ctx.push_event_by_window(FocusEvent::RemoveFocus(ctx.entity));
        }

        ctx.push_event_strategy_by_entity(
            ActivateEvent(self.target),
            self.target,
            EventStrategy::Direct,
        );
    }

    // -- Event handling --

    // handles the key down event
    fn key_down(&mut self, registry: &mut Registry, ctx: &mut Context, key_event: KeyEvent) {
        if !self.focused(ctx) {
            return;
        }

        match key_event.key {
            Key::Left => {
                if self.is_shift_down(ctx) {
                    self.expand_selection_left(ctx);
                } else {
                    self.move_selection_left(ctx);
                }
            }

            Key::Right => {
                if self.is_shift_down(ctx) {
                    self.expand_selection_right(ctx);
                } else {
                    self.move_selection_right(ctx);
                }
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
            Key::X(..) => {
                if self.is_ctlr_home_down(ctx) {
                    self.cut(registry, ctx);
                } else {
                    self.insert_text(key_event.text, ctx);
                }
            }
            Key::C(..) => {
                if self.is_ctlr_home_down(ctx) {
                    self.copy(registry, ctx);
                } else {
                    self.insert_text(key_event.text, ctx);
                }
            }
            Key::V(..) => {
                if self.is_ctlr_home_down(ctx) {
                    self.paste(registry, ctx);
                } else {
                    self.insert_text(key_event.text, ctx);
                }
            }
            Key::A(..) => {
                if self.is_ctlr_home_down(ctx) {
                    self.select_all(ctx);
                } else {
                    self.insert_text(key_event.text, ctx);
                }
            }
            _ => {
                self.insert_text(key_event.text, ctx);
            }
        }
    }

    // handles mouse down event
    fn mouse_down(&self, ctx: &mut Context, p: Mouse) {
        if !*TextBehavior::focused_ref(&ctx.widget()) {
            self.request_focus(ctx);
            return;
        }

        let selection_start = self.get_new_selection_position(ctx, p);
        let mut selection = self.selection(ctx);
        selection.set(selection_start);

        TextBehavior::selection_set(&mut ctx.widget(), selection);
    }

    // handles focus changed event
    fn focused_changed(&self, ctx: &mut Context) {
        if *TextBehavior::select_all_on_focus_ref(&ctx.widget()) {
            self.select_all(ctx);
        }

        if self.focused(ctx) {
            Cursor::visibility_set(&mut ctx.get_widget(self.cursor), Visibility::Visible);
            self.update_focused_state(ctx);
        } else {
            Cursor::visibility_set(&mut ctx.get_widget(self.cursor), Visibility::Collapsed);

            if self.len(ctx) == 0 {
                ctx.get_widget(self.target)
                    .get_mut::<Selector>("selector")
                    .set_state(EMPTY_STATE);
            } else {
                ctx.get_widget(self.target)
                    .get_mut::<Selector>("selector")
                    .clear_state();
            }

            // update the visual state of the target
            ctx.get_widget(self.target).update(false);
        }
    }

    // -- Event handling --

    // -- Helpers --

    // gets the len of the text
    fn len(&self, ctx: &mut Context) -> usize {
        TextBehavior::text_ref(&ctx.widget()).chars().count()
    }

    // gets the focused state
    fn focused(&self, ctx: &mut Context) -> bool {
        *TextBehavior::focused_ref(&ctx.widget())
    }

    fn selection(&self, ctx: &mut Context) -> TextSelection {
        *TextBehavior::selection_ref(&ctx.widget())
    }

    // check if control is pressed or on macos home key
    fn is_ctlr_home_down(&self, ctx: &mut Context) -> bool {
        // todo move window to api
        if cfg!(target_os = "macos")
            && ctx
                .window()
                .get::<KeyboardState>("keyboard_state")
                .is_home_down()
        {
            return true;
        }

        if !cfg!(target_os = "macos")
            && ctx
                .window()
                .get::<KeyboardState>("keyboard_state")
                .is_ctrl_down()
        {
            return true;
        }

        false
    }

    // check if the shift key is down
    fn is_shift_down(&self, ctx: &mut Context) -> bool {
        // todo move window to api
        if ctx
            .window()
            .get::<KeyboardState>("keyboard_state")
            .is_shift_down()
        {
            return true;
        }

        false
    }

    // Get new position for the selection based on current mouse position
    fn get_new_selection_position(&self, ctx: &mut Context, p: Mouse) -> usize {
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
        let len = self.len(ctx);

        // start x position of the cursor is start position of the text element + padding left
        let start_position: f64 = ctx.widget().get::<Point>("position").x()
            + ctx.get_widget(self.target).get::<Thickness>("padding").left
            + *TextBlock::offset_ref(&ctx.get_widget(self.text_block));

        // array which will hold char index and it's x position
        let mut position_index: Vec<(usize, f64)> = Vec::with_capacity(len);
        position_index.push((0, start_position));

        for i in 0..len {
            let bound_width: f64 = self.measure(ctx, 0, i + 1).width;

            let next_position: f64 = start_position + bound_width;

            position_index.push((i + 1, next_position));
        }

        position_index
    }

    // measure text part
    fn measure(&self, ctx: &mut Context, start: usize, end: usize) -> TextMetrics {
        let font = TextBehavior::font_clone(&ctx.widget());
        let font_size = *TextBehavior::font_size_ref(&ctx.widget());

        if let Some(text_part) =
            String16::from(TextBlock::text_ref(&ctx.get_widget(self.text_block)).as_str())
                .get_string(start, end)
        {
            return ctx
                .render_context_2_d()
                .measure(text_part.as_str(), font_size, font);
        }

        TextMetrics::default()
    }

    fn selection_start_end(&self, selection: TextSelection) -> (usize, usize) {
        if selection.start() > selection.end() {
            return (selection.end(), selection.start());
        }
        (selection.start(), selection.end())
    }

    fn update_focused_state(&self, ctx: &mut Context) {
        if self.len(ctx) == 0 {
            ctx.get_widget(self.target)
                .get_mut::<Selector>("selector")
                .set_state(EMPTY_FOCUSED_STATE);
        } else {
            ctx.get_widget(self.target)
                .get_mut::<Selector>("selector")
                .set_state(FOCUSED_STATE);
        }

        // update the visual state of the target
        ctx.get_widget(self.target).update(false);
    }

    // -- Helpers --
}

impl State for TextBehaviorState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.cursor = Entity::from(*TextBehavior::cursor_ref(&ctx.widget()));

        self.target = Entity::from(*TextBehavior::target_ref(&ctx.widget()));

        self.text_block = Entity::from(*TextBehavior::text_block_ref(&ctx.widget()));

        // hide cursor
        Cursor::visibility_set(&mut ctx.get_widget(self.cursor), Visibility::Collapsed);

        // set initial empty state
        if TextBehavior::text_ref(&ctx.widget()).len() == 0 {
            ctx.get_widget(self.target)
                .get_mut::<Selector>("selector")
                .set_state("empty");
            ctx.get_widget(self.target).update(false);
        }
    }

    fn update(&mut self, registry: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action.clone() {
            match action {
                TextAction::KeyDown(event) => self.key_down(registry, ctx, event),
                TextAction::MouseDown(p) => self.mouse_down(ctx, p),
                TextAction::Drop(text, position) => {
                    if check_mouse_condition(position, &ctx.get_widget(self.target)) {
                        self.insert_text(text, ctx);
                    }
                }
                TextAction::FocusedChanged => self.focused_changed(ctx),
                TextAction::SelectionChanged => return,
            }

            self.action = None;
        }
    }

    fn update_post_layout(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action.clone() {
            match action {
                TextAction::SelectionChanged => self.update_cursor(ctx),
                _ => {}
            }

            self.action = None;
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
    /// * a `target`: the [`Entity`] of the target widget
    /// * a `text_block`: the [`Entity`] of the [`TextBlock`] widget
    ///
    /// * and must inherit the following properties from its target:
    ///     * focused
    ///     * font
    ///     * font_size
    ///     * lost_focus_on_activation
    ///     * request_focus
    ///     * text
    ///     * selection
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
    ///     selection: TextSelection
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
    ///            .selection(id)
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
    ///            .selection(id)
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
        /// Reference the target (parent) widget e.g. `TextBox` or `PasswordBox`.
        target: u32,

        /// Reference text selection `Cursor`.
        cursor: u32,

        /// Reference `TextBlock` that is used to display the text.
        text_block: u32,

        /// Sets or shares the focused property.
        focused: bool,

        /// Sets or shares the font property.
        font: String,

        /// Sets or shares the font size property.
        font_size: f64,

        /// Sets or shares ta value that describes if the widget should lost focus on activation (when Enter pressed).
        lost_focus_on_activation: bool,

        /// Sets or shares the request_focus property. Used to request focus from outside.Set to `true` to request focus.
        request_focus: bool,

        /// Sets or shares the text property.
        text: String,

        /// Sets or shares the text selection property.
        selection: TextSelection,

        /// If set to `true` all character will be focused when the widget gets focus. Default is `true`
        select_all_on_focus: bool
    }
);

impl Template for TextBehavior {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("TextBehavior")
            .font_size(fonts::FONT_SIZE_12)
            .font("Roboto-Regular")
            .text("")
            .selection(TextSelection::default())
            .focused(false)
            .lost_focus_on_activation(true)
            .select_all_on_focus(false)
            .child(
                MouseBehavior::new()
                    .visibility(id)
                    .enabled(id)
                    .on_mouse_down(move |states, m| {
                        states
                            .get_mut::<TextBehaviorState>(id)
                            .action(TextAction::MouseDown(m));
                        true
                    })
                    .build(ctx),
            )
            .on_key_down(move |states, event| -> bool {
                states
                    .get_mut::<TextBehaviorState>(id)
                    .action(TextAction::KeyDown(event));
                false
            })
            .on_drop_file(move |states, file_name, position| {
                states
                    .get_mut::<TextBehaviorState>(id)
                    .action(TextAction::Drop(file_name, position));
                false
            })
            .on_drop_text(move |states, file_name, position| {
                states
                    .get_mut::<TextBehaviorState>(id)
                    .action(TextAction::Drop(file_name, position));
                false
            })
            .on_changed("focused", |states, id| {
                states
                    .get_mut::<TextBehaviorState>(id)
                    .action(TextAction::FocusedChanged);
            })
            .on_changed("selection", |states, id| {
                states
                    .get_mut::<TextBehaviorState>(id)
                    .action(TextAction::SelectionChanged);
            })
    }
}

// --- Helpers --

fn move_selection_left(mut selection: TextSelection) -> TextSelection {
    if selection.start() == selection.end() {
        if selection.start() as i32 > 0 {
            selection.set(selection.start() - 1);
        }
    } else if selection.start() < selection.end() {
        selection.set_end(selection.start());
    } else {
        selection.set_start(selection.end());
    }

    selection
}

fn move_selection_right(mut selection: TextSelection, len: usize) -> TextSelection {
    if selection.start() == selection.end() {
        if selection.start() < len {
            selection.set(selection.start() + 1);
        }
    } else if selection.start() < selection.end() {
        selection.set_start(selection.end());
    } else {
        selection.set_end(selection.start());
    }

    selection
}

// --- Helpers --

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_selection_left() {
        //  check left bounds
        let selection = TextSelection::new(0, 0);
        let result = move_selection_left(selection);
        assert_eq!(result.start(), 0);
        assert_eq!(result.end(), 0);

        // start == end
        let selection = TextSelection::new(1, 1);
        let result = move_selection_left(selection);
        assert_eq!(result.start(), 0);
        assert_eq!(result.end(), 0);

        // start < end
        let selection = TextSelection::new(4, 6);
        let result = move_selection_left(selection);
        assert_eq!(result.start(), 4);
        assert_eq!(result.end(), 4);

        // start > end
        let selection = TextSelection::new(6, 4);
        let result = move_selection_left(selection);
        assert_eq!(result.start(), 4);
        assert_eq!(result.end(), 4);
    }

    #[test]
    fn test_move_selection_right() {
        //  check left bounds
        let selection = TextSelection::new(4, 4);
        let len = 5;
        let result = move_selection_right(selection, len);
        assert_eq!(result.start(), 5);
        assert_eq!(result.end(), 5);

        // start == end
        let selection = TextSelection::new(3, 3);
        let len = 5;
        let result = move_selection_right(selection, len);
        assert_eq!(result.start(), 4);
        assert_eq!(result.end(), 4);

        // start < end
        let selection = TextSelection::new(4, 6);
        let result = move_selection_right(selection, len);
        assert_eq!(result.start(), 6);
        assert_eq!(result.end(), 6);

        // start > end
        let selection = TextSelection::new(6, 4);
        let result = move_selection_right(selection, len);
        assert_eq!(result.start(), 6);
        assert_eq!(result.end(), 6);
    }
}
