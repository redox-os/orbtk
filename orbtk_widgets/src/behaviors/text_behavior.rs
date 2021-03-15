use crate::{
    api::prelude::*,
    proc_macros::*,
    render::TextMetrics,
    shell::prelude::{Key, KeyEvent},
    themes::theme_orbtk::orbtk_fonts,
    Cursor, TextBlock,
};

// --- KEYS ---
pub static NOT_EMPTY_STATE: &str = "not_empty";
pub static NOT_EMPTY_FOCUSED_STATE: &str = "not_empty_focused";
pub static FOCUSED_STATE: &str = "focused";

// --- CONSTANTS ---
pub static CONDITIONAL_LINE_BRAKE: &str = "\n";

/// Actions of TextBehaviorState
#[derive(Clone, Debug)]
pub enum TextAction {
    /// Send message with drop event
    Drop(String, Point),
    /// Send message with focus has changed event.
    FocusedChanged,
    /// Used to force an update on visual state and offset.
    ForceUpdate(bool),
    /// Send message with key down event.
    KeyDown(KeyEvent),
    /// Send message with line_wrap has changed event.
    LineWrapChanged,
    /// Send message with mouse down event.
    MouseDown(Mouse),
    /// Send message with mouse move event.
    MouseMove(Point),
    /// Send message with mouse up event.
    MouseUp,
    /// Send message with selection has changed event.
    SelectionChanged,
    /// Send message to insert new text.
    TextInput(String),
}

/// Message that the behavior can sent to its target.
pub enum TextResult {
    /// Text was manipulated. Contains the new text string.
    TextManipulated(String),
}

// helper enum
#[derive(Debug, PartialEq)]
enum CursorDirection {
    Left,
    Right,
    None,
}

impl Default for CursorDirection {
    fn default() -> Self {
        CursorDirection::None
    }
}

/// The `TextBehaviorState` handles the text processing of the
/// `TextBehavior` widget.
#[derive(Default, AsAny)]
pub struct TextBehaviorState {
    cursor: Entity,
    direction: CursorDirection,
    event_adapter: EventAdapter,
    pressed: bool,
    self_update: bool,
    target: Entity,
    text_block: Entity,
    update_selection: bool,
    window: Entity, //mouse_up_count: usize,
}

impl TextBehaviorState {
    // -- Text operations --

    // handle backspace key
    fn back_space(&mut self, ctx: &mut Context) {
        if self.clear_selection(ctx) {
            return;
        }

        let mut selection = self.selection(ctx);

        if selection.start() == 0 {
            return;
        }

        selection.set(selection.start() - 1);

        let mut text = String16::from(ctx.get_widget(self.target).clone::<String>("text"));

        let removed_width = self
            .measure(ctx, selection.start(), selection.start() + 1)
            .width;

        let mut offset = *Cursor::offset_ref(&ctx.get_widget(self.cursor));
        offset = (offset + removed_width).min(0.);

        Cursor::offset_set(&mut ctx.get_widget(self.cursor), offset);
        TextBlock::offset_set(&mut ctx.get_widget(self.text_block), offset);

        text.remove(selection.start());

        self.set_text(ctx, text.to_string());
        self.set_selection(ctx, selection);

        if self.len(ctx) == 0 {
            self.update_focused_state(ctx);
        }
    }

    // copy selected text to registry
    fn copy(&self, registry: &mut Registry, ctx: &mut Context) {
        let selection = self.selection(ctx);

        let (start, end) = self.selection_start_end(selection);

        if selection.is_empty() {
            return;
        }

        if let Some(copy_text) = String16::from(ctx.get_widget(self.target).clone::<String>("text"))
            .get_string(start, end)
        {
            registry.get_mut::<Clipboard>("clipboard").set(copy_text);
        }
    }

    // clear all chars from the selection.
    fn clear_selection(&mut self, ctx: &mut Context) -> bool {
        let mut selection = self.selection(ctx);

        if selection.is_empty() {
            return false;
        }

        let mut text = String16::from(ctx.get_widget(self.target).clone::<String>("text"));

        let (start, end) = self.selection_start_end(selection);

        for i in (start..end).rev() {
            text.remove(i);
        }

        let removed_width = self.measure(ctx, start, end).width;

        let mut offset = *Cursor::offset_ref(&ctx.get_widget(self.cursor));
        offset = (offset + removed_width).min(0.);

        Cursor::offset_set(&mut ctx.get_widget(self.cursor), offset);
        TextBlock::offset_set(&mut ctx.get_widget(self.text_block), offset);

        selection.set(start);

        self.set_text(ctx, text.to_string());
        self.set_selection(ctx, selection);

        if self.len(ctx) == 0 {
            self.update_focused_state(ctx);
        }

        true
    }

    // copy to registry and delete the selection
    fn cut(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.copy(registry, ctx);
        self.clear_selection(ctx);
    }

    // clear text and remove from selection
    fn delete(&mut self, ctx: &mut Context) {
        if self.clear_selection(ctx) {
            return;
        }

        let selection = self.selection(ctx);
        let len = self.len(ctx);

        if len == 0 || selection.end() > self.len(ctx) || selection.start() >= self.len(ctx) {
            return;
        }

        let mut text = String16::from(ctx.get_widget(self.target).clone::<String>("text"));

        text.remove(selection.start());

        self.set_text(ctx, text.to_string());
    }

    fn insert_conditional_line_brake(&mut self, ctx: &mut Context) {
        if !self.focused(ctx) {
            return;
        }

        let mut update_focus_state = self.len(ctx) == 0;

        update_focus_state = update_focus_state || self.clear_selection(ctx);

        let mut selection = self.selection(ctx);

        let mut text = ctx.get_widget(self.target).clone::<String>("text");
        text.push_str(CONDITIONAL_LINE_BRAKE);
        //text.insert_str(selection.start(), insert_conditional_line_break.as_str());

        selection.set(selection.start() + CONDITIONAL_LINE_BRAKE.len());
        self.set_selection(ctx, selection);

        self.update_selection = true;

        self.set_text(ctx, text);

        // used to trigger bounds adjustments
        self.direction = CursorDirection::Right;

        if update_focus_state {
            self.update_focused_state(ctx);
        }
    }

    // insert text from given selection
    fn insert_text(&mut self, insert_text: String, ctx: &mut Context) {
        if insert_text.is_empty() || !self.focused(ctx) {
            return;
        }

        let mut update_focus_state = self.len(ctx) == 0;

        update_focus_state = update_focus_state || self.clear_selection(ctx);

        let mut selection = self.selection(ctx);

        let mut text = String16::from(ctx.get_widget(self.target).clone::<String>("text"));
        text.insert_str(selection.start(), insert_text.as_str());

        selection.set(selection.start() + insert_text.chars().count());
        self.set_selection(ctx, selection);

        self.update_selection = true;

        self.set_text(ctx, text.to_string());

        // used to trigger bounds adjustments
        self.direction = CursorDirection::Right;

        if update_focus_state {
            self.update_focused_state(ctx);
        }
    }

    // paste in text from clipboard
    fn paste(&mut self, registry: &mut Registry, ctx: &mut Context) {
        if let Some(value) = registry.get::<Clipboard>("clipboard").get() {
            self.insert_text(value, ctx);
        }
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

        if self.direction == CursorDirection::None {
            return;
        }

        // adjust position
        // WIP: adapt handling of `conditional_line_break`
        // if self.line_wrap(ctx) {
        //     let offset = *Cursor::offset_ref(&ctx.get_widget(self.cursor));
        //     let width = Cursor::bounds_ref(&ctx.get_widget(self.cursor)).width();
        //     let delta = width - offset;
        // }

        let offset = *Cursor::offset_ref(&ctx.get_widget(self.cursor));
        let width = Cursor::bounds_ref(&ctx.get_widget(self.cursor)).width();
        let delta = width - offset;

        if self.direction == CursorDirection::Right && cursor_start_measure.width > delta {
            let offset_delta = delta - cursor_start_measure.width;
            Cursor::offset_set(&mut ctx.get_widget(self.cursor), offset + offset_delta);
            TextBlock::offset_set(&mut ctx.get_widget(self.text_block), offset + offset_delta);
        }

        if self.direction == CursorDirection::Left && cursor_start_measure.width + offset < 0. {
            let offset_delta = cursor_start_measure.width + offset;
            Cursor::offset_set(&mut ctx.get_widget(self.cursor), offset - offset_delta);
            TextBlock::offset_set(&mut ctx.get_widget(self.text_block), offset - offset_delta);
        }

        self.direction = CursorDirection::None;
    }

    fn select_all(&mut self, ctx: &mut Context) {
        if TextBlock::text_ref(&ctx.get_widget(self.text_block)).is_empty()
            || !*TextBehavior::focused_ref(&ctx.widget())
        {
            return;
        }

        let mut selection = self.selection(ctx);
        selection.set_start(0);
        selection.set_end(self.len(ctx));

        self.set_selection(ctx, selection);
    }

    fn expand_selection_left(&mut self, ctx: &mut Context) {
        self.direction = CursorDirection::Left;
        let mut selection = self.selection(ctx);
        if selection.start() as i32 > 0 {
            selection.set_start(selection.start() - 1);
        }
        self.set_selection(ctx, selection);
    }

    fn expand_selection_right(&mut self, ctx: &mut Context) {
        self.direction = CursorDirection::Right;
        let mut selection = self.selection(ctx);
        if selection.start() < self.len(ctx) {
            selection.set_start(selection.start() + 1);
        }
        self.set_selection(ctx, selection);
    }

    fn move_selection_left(&mut self, ctx: &mut Context) {
        self.direction = CursorDirection::Left;
        let selection = move_selection_left(self.selection(ctx));
        self.set_selection(ctx, selection);
    }

    fn move_selection_right(&mut self, ctx: &mut Context) {
        self.direction = CursorDirection::Right;
        let selection = move_selection_right(self.selection(ctx), self.len(ctx));
        self.set_selection(ctx, selection);
    }

    // -- Selection --

    // -- Event handling --

    fn activate(&self, ctx: &mut Context) {
        if *ctx.widget().get::<bool>("lose_focus_on_activation") {
            self.event_adapter
                .push_event_direct(self.window, FocusEvent::RemoveFocus(self.target));
        }

        self.event_adapter
            .push_event_direct(self.target, ActivateEvent(self.target));
    }

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
                if self.is_ctrl_enter_down(ctx) {
                    // just intercept, if `line_warp` is selected (true)
                    if self.line_wrap(ctx) {
                        println!("Line wrap: insert conditional_line_brake.");
                        self.insert_conditional_line_brake(ctx);
                    }
                } else {
                    self.activate(ctx);
                }
            }
            Key::X(..) => {
                if self.is_ctrl_home_down(ctx) {
                    self.cut(registry, ctx);
                }
            }
            Key::C(..) => {
                if self.is_ctrl_home_down(ctx) {
                    println!("is_ctrl_home_down: true.");
                    self.copy(registry, ctx);
                }
            }
            Key::V(..) => {
                if self.is_ctrl_home_down(ctx) {
                    self.paste(registry, ctx);
                }
            }
            Key::A(..) => {
                if self.is_ctrl_home_down(ctx) {
                    self.select_all(ctx);
                }
            }
            Key::Escape => self.collapse_selection(ctx),
            _ => {}
        }
    }

    // handles mouse down event
    fn mouse_down(&mut self, ctx: &mut Context, mouse: Mouse) {
        self.pressed = true;
        if !*TextBehavior::focused_ref(&ctx.widget()) {
            self.request_focus();
            return;
        }

        let selection_start = self.get_new_selection_position(ctx, mouse.position);
        let mut selection = self.selection(ctx);
        selection.set(selection_start);

        self.set_selection(ctx, selection);
    }

    // handles mouse move
    fn mouse_move(&mut self, ctx: &mut Context, position: Point) {
        if !self.pressed || !*TextBehavior::focused_ref(&ctx.widget()) {
            return;
        }

        let mut selection = self.selection(ctx);
        let new_start = self.get_new_selection_position(ctx, position);

        if selection.start() == new_start {
            return;
        }

        if selection.start() < new_start {
            self.direction = CursorDirection::Right;
        } else {
            self.direction = CursorDirection::Left;
        }

        selection.set_start(new_start);

        if selection.start() > 0 || selection.start() < self.len(ctx) {
            ctx.send_message(TextAction::MouseMove(position), ctx.entity());
            ctx.widget().set("dirty", true);
        }

        self.set_selection(ctx, selection);
    }

    fn collapse_selection(&mut self, ctx: &mut Context) {
        let mut selection = self.selection(ctx);
        selection.set_end(selection.start());

        self.set_selection(ctx, selection);
    }

    fn mouse_up(&mut self, _ctx: &mut Context) {
        self.pressed = false;

        // todo need timer
        // if !self.focused(ctx) {
        //     return;
        // }

        // if self.mouse_up_count == 1 {
        //     self.mouse_up_count = 0;
        //     self.select_all(ctx);
        //     return;
        // }

        // self.mouse_up_count += 1;
    }

    // handles focus changed event
    fn focused_changed(&mut self, ctx: &mut Context) {
        self.adjust_selection(ctx);

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
                    .remove_state(NOT_EMPTY_STATE);
            } else {
                ctx.get_widget(self.target)
                    .get_mut::<Selector>("selector")
                    .push_state(NOT_EMPTY_STATE);
            }

            // update the visual state of the target
            ctx.get_widget(self.target).update(false);
        }
    }

    // -- Event handling --

    // -- Helpers --

    // get the focused state
    fn adjust_selection(&mut self, ctx: &mut Context) {
        let mut selection = self.selection(ctx);
        let len = self.len(ctx);

        if selection.start() <= len || selection.end() <= len {
            return;
        }

        selection.set(len);

        self.set_selection(ctx, selection);

        if *TextBehavior::focused_ref(&ctx.widget()) {
            self.update_focused_state(ctx);
        }
    }

    // get a reference to the focused property
    fn focused(&self, ctx: &mut Context) -> bool {
        *TextBehavior::focused_ref(&ctx.widget())
    }

    // force update of the focused state
    fn force_update(&mut self, ctx: &mut Context, force: bool) {
        let self_update = self.self_update;
        self.self_update = false;

        if self_update && !force {
            return;
        }

        self.adjust_selection(ctx);

        if self.len(ctx) == 0 {
            Cursor::offset_set(&mut ctx.get_widget(self.cursor), 0.);
            TextBlock::offset_set(&mut ctx.get_widget(self.text_block), 0.);

            ctx.get_widget(self.target)
                .get_mut::<Selector>("selector")
                .remove_state(NOT_EMPTY_STATE);
        } else {
            ctx.get_widget(self.target)
                .get_mut::<Selector>("selector")
                .push_state(NOT_EMPTY_STATE);
        }
    }

    // get new position for the selection based on current mouse position
    fn get_new_selection_position(&self, ctx: &mut Context, position: Point) -> usize {
        if let Some((index, _x)) = self
            .map_chars_index_to_position(ctx)
            .iter()
            .min_by_key(|(_index, x)| (position.x() - x).abs() as u64)
        {
            return *index;
        }

        0
    }

    // check if control is pressed or on macos home key
    fn is_ctrl_enter_down(&self, ctx: &mut Context) -> bool {
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

    // check if control is pressed or on macos home key
    fn is_ctrl_home_down(&self, ctx: &mut Context) -> bool {
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

    // returns a vector with a tuple of each char's starting index (usize) and position (f64)
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

    // gets the len of the text
    fn len(&self, ctx: &mut Context) -> usize {
        TextBlock::text_ref(&ctx.get_widget(self.text_block))
            .chars()
            .count()
    }

    // get a reference to the line_wrap property
    fn line_wrap(&self, ctx: &mut Context) -> bool {
        *TextBehavior::line_wrap_ref(&ctx.widget())
    }

    // toggles line wapping for given input text
    fn toggle_line_wrap(&mut self, ctx: &mut Context) {
        if self.line_wrap(ctx) {
            println!("Line wrap toggled to `false`");
            TextBehavior::line_wrap_set(&mut ctx.widget(), false);
        } else {
            println!("Line wrap toggled to `true`");
            TextBehavior::line_wrap_set(&mut ctx.widget(), true);
        }
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

    fn request_focus(&self) {
        self.event_adapter
            .push_event_direct(self.window, FocusEvent::RequestFocus(self.target));
    }

    fn selection(&self, ctx: &mut Context) -> TextSelection {
        *TextBehavior::selection_ref(&ctx.widget())
    }

    fn selection_start_end(&self, selection: TextSelection) -> (usize, usize) {
        if selection.start() > selection.end() {
            return (selection.end(), selection.start());
        }
        (selection.start(), selection.end())
    }

    // sets new text
    fn set_text(&mut self, ctx: &mut Context, text: String) {
        ctx.get_widget(self.target).set("text", text.clone());
        ctx.send_message(TextResult::TextManipulated(text), self.target);
        self.self_update = true;
    }

    // sets and mark selected text
    fn set_selection(&mut self, ctx: &mut Context, selection: TextSelection) {
        TextBehavior::selection_set(&mut ctx.widget(), selection);
        self.update_selection = true;
    }

    fn update_focused_state(&self, ctx: &mut Context) {
        ctx.get_widget(self.target)
            .get_mut::<Selector>("selector")
            .remove_state(FOCUSED_STATE);

        if !self.focused(ctx) {
            return;
        }

        if self.len(ctx) == 0 {
            ctx.get_widget(self.target)
                .get_mut::<Selector>("selector")
                .push_state(FOCUSED_STATE);
            ctx.get_widget(self.target)
                .get_mut::<Selector>("selector")
                .remove_state(NOT_EMPTY_FOCUSED_STATE);
        } else {
            ctx.get_widget(self.target)
                .get_mut::<Selector>("selector")
                .push_state(NOT_EMPTY_FOCUSED_STATE);
            ctx.get_widget(self.target)
                .get_mut::<Selector>("selector")
                .remove_state(FOCUSED_STATE);
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

        self.event_adapter = ctx.event_adapter();
        self.window = ctx.entity_of_window();

        // hide cursor
        Cursor::visibility_set(&mut ctx.get_widget(self.cursor), Visibility::Collapsed);

        // set initial empty state
        if TextBlock::text_ref(&ctx.get_widget(self.text_block)).is_empty() {
            ctx.get_widget(self.target)
                .get_mut::<Selector>("selector")
                .remove_state(NOT_EMPTY_STATE);
            ctx.get_widget(self.target).update(false);
        }
    }

    fn messages(
        &mut self,
        mut messages: MessageReader,
        registry: &mut Registry,
        ctx: &mut Context,
    ) {
        for action in messages.read::<TextAction>() {
            match action {
                TextAction::Drop(text, position) => {
                    if check_mouse_condition(position, &ctx.get_widget(self.target)) {
                        self.insert_text(text, ctx);
                    }
                }
                TextAction::FocusedChanged => self.focused_changed(ctx),
                TextAction::ForceUpdate(force) => self.force_update(ctx, force),
                TextAction::KeyDown(event) => self.key_down(registry, ctx, event),
                TextAction::LineWrapChanged => self.toggle_line_wrap(ctx),
                TextAction::MouseDown(p) => self.mouse_down(ctx, p),
                TextAction::MouseMove(position) => self.mouse_move(ctx, position),
                TextAction::MouseUp => self.mouse_up(ctx),
                TextAction::SelectionChanged => self.update_selection = true,
                TextAction::TextInput(text) => self.insert_text(text, ctx),
            }
        }
    }

    fn update_post_layout(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        if self.update_selection {
            self.update_cursor(ctx);

            self.update_selection = false;
        }
    }
}

widget!(
    /// The `TextBehavior` widget implements generic text handling
    /// functionality, that can be inherited to any text-related
    /// widgets (like TextBox, PasswordBox). Following associated
    /// functions are implemented to control the behavior inside the
    /// text property:
    ///
    /// * insert characters via keyboard press
    /// * select all text with Ctrl+A key combination
    /// * delete selected text with Backspace or Delete
    /// * move cursor left or right using arrow keys
    /// * move cursor via mouse click to given position
    /// * use Backspace- or Delete-Key to remove preceding characters
    /// * press Enter-Key to run on_activate() callback
    /// * use Ctrl-Enter-Key to line wap the inserted text
    ///
    /// Please take care to define and `attach` the next **listed
    /// entities** to your text processing widgets. If not,
    /// functionality is not inherited and can't be processed by
    /// `TextBehavior`:
    ///
    /// * a `cursor`: the [`Entity`] of a [`Cursor`] widget
    /// * a `target`: the [`Entity`] of the target widget
    /// * a `text_block`: the [`Entity`] of the [`TextBlock`] widget
    ///
    /// Each of this building blog entities needs define the listed
    /// properties to inherit them from its respective target:
    ///
    ///     * focused
    ///     * font
    ///     * font_size
    ///     * line_wrap
    ///     * lose_focus_on_activation
    ///     * request_focus
    ///     * text
    ///     * selection
    ///
    /// # Example
    ///
    /// The following code example creates a new widget `MyInput`. Intantiate it
    /// in you App as needed.
    ///
    /// ```rust
    /// use orbtk::prelude::*;
    ///
    /// // --- CONSTANTS ---
    /// pub static ID_MY_INPUT: &str = "MY_INPUT_WIDGET";
    ///
    /// widget!(MyInput {
    ///     /// MyInput is an example widget that inherits its functionality from
    ///     /// `TextBehavior`.
    ///     ///
    ///     /// To use the text handling functions, you need to define essential
    ///     /// properties that inherit its values from the parent (here:
    ///     /// `TextBehavior`). Interaction on the user input is offered via the
    ///     /// `TextInputHandler`.
    ///     focused: bool,
    ///     font: String,
    ///     font_size: f64,
    ///     line_wrap: bool,
    ///     lose_focus_on_activation: bool,
    ///     request_focus: bool,
    ///     selection: TextSelection
    /// });
    ///
    /// impl Template for MyInput {
    ///     fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
    ///         // TextBlock widget is our target (a child of TextBehaviour)
    ///         let text_block = TextBlock::new()
    ///             .font(id)
    ///             .font_size(id)
    ///             .line_wrap(id)
    ///             .text(id)
    ///             .water_mark(id)
    ///             .build(ctx);
    ///
    ///         // Cursor widget is a child of TextBehavior
    ///         let cursor = Cursor::new()
    ///            // will inherit from its target `TextBlock`
    ///            .focused(id)
    ///            .selection(id)
    ///            .text_block(text_block.0) // .0 is needed to unwrap its u32
    ///            .build(ctx);
    ///
    ///        // TextBehavior widget is the parent
    ///        let text_behavior = TextBehavior::new()
    ///            .cursor(cursor.0) // .0 is needed to unwrap its u32
    ///            .focused(id)
    ///            .font(id)
    ///            .font_size(id)
    ///            .line_wrap(id)
    ///            .lose_focus_on_activation(id)
    ///            .request_focus(id)
    ///            .selection(id)
    ///            .target(id.0)
    ///            .text(id)
    ///            .build(ctx);
    ///
    ///        // creates the widget subtree
    ///        self.id(ID_MY_INPUT)
    ///            .child(cursor)
    ///            .child(text_behavior)
    /// }
    /// ```
    ///
    /// Here is a sample calling App code that makes use of `MyInput`.
    ///
    /// ```rust
    /// use orbtk::{
    ///     {api::prelude::*, proc_macros::*, theme_default::prelude::*},
    ///     prelude::behaviors::TextBehavior,
    ///     prelude::*,
    /// };
    ///
    /// // --- CONSTANTS ---
    /// pub static ID_HEADER: &str = "Text behavior widget";
    ///
    /// fn main() {
    ///    // use this only if you want to run it as web application.
    ///    orbtk::initialize();
    ///
    ///    Application::new()
    ///        .window(|ctx| {
    ///            let text_block = MyInput::new()
    ///                .background(colors::LYNCH_COLOR)
    ///                .text("Our TextBehavior widget text.")
    ///                .build(ctx);
    ///
    ///            Window::new()
    ///                .title("OrbTk - text behaviour sample")
    ///                .position((100.0, 100.0))
    ///                .size(420.0, 160.0)
    ///                .child(
    ///                    Container::new()
    ///                        .margin(16)
    ///                        .child(
    ///                            Stack::new()
    ///                                .orientation("vertical")
    ///                                .spacing(8)
    ///                                .clip(true)
    ///                                .child(
    ///                                    TextBlock::new()
    ///                                       .style("header")
    ///                                       .text(ID_HEADER)
    ///                                       .build(ctx),
    ///                                )
    ///                                .child(text_block)
    ///                                .build(ctx),
    ///                        )
    ///                        .build(ctx),
    ///                )
    ///                .build(ctx)
    ///        })
    ///        .run();
    ///
    /// [`Entity`]: https://docs.rs/dces/0.2.0/dces/entity/struct.Entity.html
    /// [`Cursor`]: ../struct.Cursor.html
    TextBehavior<TextBehaviorState>: ActivateHandler, KeyDownHandler, TextInputHandler, DropHandler, MouseHandler {
        /// The `Cursor` reference will handle the text selection.
        cursor: u32,

        /// Sets or shares the focused property.
        focused: bool,

        /// Sets or shares the font property.
        font: String,

        /// Sets or shares the font size property.
        font_size: f64,

        /// Support line wrapping using Ctrl-Enter key.
        /// Default: `true`
        line_wrap: bool,

        /// Sets or shares a `lose_focus_on_activation`
        /// boolean. Describes what happens if the widget will lose
        /// focus on activation (e.g Enter-Key is pressed).
        lose_focus_on_activation: bool,

        /// Sets or shares a `request_focus` property. If
        /// `request_focus` value is `true`, the widget will request
        /// focus from outside.
        request_focus: bool,

        /// Sets or shares the text selection property.
        selection: TextSelection,

        /// All character will be focused when the widget gets
        /// focus. Default: `true`
        select_all_on_focus: bool,

        /// The `target ` reference the parent widget (e.g. `TextBox` or `PasswordBox`).
        target: u32,

        /// The `TextBlock` reference is used to display the text.
        text_block: u32,

        /// Sets or shares the text property.
        text: String

    }
);

impl Template for TextBehavior {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("TextBehavior")
            .font_size(orbtk_fonts::FONT_SIZE_12)
            .font("Roboto-Regular")
            .text("")
            .selection(TextSelection::default())
            .focused(false)
            .line_wrap(true)
            .lose_focus_on_activation(true)
            .select_all_on_focus(false)
            .on_key_down(move |ctx, event| -> bool {
                ctx.send_message(TextAction::KeyDown(event), id);
                false
            })
            .on_text_input(move |ctx, text| {
                ctx.send_message(TextAction::TextInput(text.to_string()), id);
                false
            })
            .on_drop_file(move |ctx, file_name, position| {
                ctx.send_message(TextAction::Drop(file_name, position), id);
                false
            })
            .on_drop_text(move |ctx, file_name, position| {
                ctx.send_message(TextAction::Drop(file_name, position), id);
                false
            })
            .on_mouse_down(move |ctx, m| {
                ctx.send_message(TextAction::MouseDown(m), id);
                true
            })
            .on_mouse_up(move |ctx, _| {
                ctx.send_message(TextAction::MouseUp, id);
            })
            .on_mouse_move(move |ctx, p| {
                ctx.send_message(TextAction::MouseMove(p), id);
                true
            })
            .on_changed("focused", move |ctx, _| {
                ctx.send_message(TextAction::FocusedChanged, id);
            })
            .on_changed("selection", move |ctx, _| {
                ctx.send_message(TextAction::SelectionChanged, id);
            })
    }
}

// --- Helpers --

fn move_selection_left(mut selection: TextSelection) -> TextSelection {
    match selection.start().cmp(&selection.end()) {
        std::cmp::Ordering::Less => selection.set_end(selection.start()),
        std::cmp::Ordering::Equal => {
            if selection.start() as i32 > 0 {
                selection.set(selection.start() - 1);
            }
        }
        std::cmp::Ordering::Greater => selection.set_start(selection.end()),
    }

    selection
}

fn move_selection_right(mut selection: TextSelection, len: usize) -> TextSelection {
    match selection.start().cmp(&selection.end()) {
        std::cmp::Ordering::Less => selection.set_start(selection.end()),
        std::cmp::Ordering::Equal => {
            if selection.start() < len {
                selection.set(selection.start() + 1);
            }
        }
        std::cmp::Ordering::Greater => selection.set_end(selection.start()),
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

    #[test]
    fn test_insert_conditional_line_break() {
        let mut text = String::from("First line");
        let second_line = String::from("Second line");

        text.push_str(CONDITIONAL_LINE_BRAKE);
        text.push_str(second_line.as_str());

        assert_eq!("First line\nSecond line", text.as_str());
    }
}
