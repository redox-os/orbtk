use super::behaviors::MouseBehavior;
use crate::{
    prelude::*,
    shell::{Key, KeyEvent},
};

use crate::shell::CONSOLE;

// --- KEYS --

pub static ELEMENT_TEXT_BOX: &'static str = "text_box";

static ID_CURSOR: &'static str = "id_cursor";
// static ID_SCROLL_VIEWER: &'static str = "id_scroll_viewer";
static ID_TEXT_BLOCK: &'static str = "id_text_block";
static ID_TEXT_BLOCK_ROOT: &'static str = "id_text_block_root";

// --- KEYS --

#[derive(Clone)]
enum TextBoxAction {
    Key(KeyEvent),
    Mouse(Point),
}

/// The `TextBoxState` handles the text processing of the `TextBox` widget.
#[derive(Default, AsAny)]
pub struct TextBoxState {
    action: Option<TextBoxAction>,
    cursor_x: f64,
    len: usize,
    cursor: Entity,
    text_block_root: Entity,
    // scroll_viewer: Entity,
    text_block: Entity,
}

impl TextBoxState {
    fn action(&mut self, action: TextBoxAction) {
        self.action = Some(action);
    }

    fn handle_key_event(&self, key_event: KeyEvent, ctx: &mut Context<'_>) {
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

    fn request_focus(&self, ctx: &mut Context<'_>) {
        ctx.push_event_by_window(FocusEvent::RequestFocus(ctx.entity));
    }

    // Reset selection and offset if text is changed from outside
    fn reset(&self, ctx: &mut Context<'_>) {
        ctx.widget().set("text_selection", TextSelection::default());
        ctx.widget().set("scroll_offset", Point::default());
        ctx.push_event_strategy_by_entity(
            ChangedEvent(ctx.entity),
            ctx.entity,
            EventStrategy::Direct,
        );
    }

    fn check_outside_update(&self, ctx: &mut Context<'_>) {
        let len = ctx.widget().get::<String16>("text").len();
        if self.len != len && self.len > len {
            self.reset(ctx);
        }
    }

    // Adjust offset of text and cursor if cursor position is out of bounds
    fn adjust_cursor(&mut self, ctx: &mut Context) {
        let cursor_x = ctx
            .get_widget(self.cursor)
            .get::<Thickness>("margin")
            .left();
        let view_port_width = ctx
            .get_widget(self.text_block_root)
            .get::<Rectangle>("bounds")
            .width();

        CONSOLE.log(format!("cursor {}", cursor_x));

        if cursor_x >= 0.0 && cursor_x < view_port_width {
            return;
        }

        let delta = if cursor_x < 0.0 {
            cursor_x
        } else {
            cursor_x - view_port_width
        };

        if let Some(bounds) = ctx
            .get_widget(self.text_block)
            .try_get_mut::<Rectangle>("bounds")
        {
            bounds.set_x(bounds.x() - delta);
        }

        if let Some(bounds) = ctx
            .get_widget(self.cursor)
            .try_get_mut::<Rectangle>("bounds")
        {
            bounds.set_x(bounds.x() - delta);
        }

        if let Some(margin) = ctx
            .get_widget(self.cursor)
            .try_get_mut::<Thickness>("margin")
        {
            margin.set_left(margin.left() - delta);
        }

        CONSOLE.log(format!("delta {}", delta));

        // ctx.widget().get_mut::<Point>("scroll_offset").x -= delta;
        // ctx.get_widget(self.cursor).get_mut::<Thickness>("margin").set_left(cursor_x - delta);

        // if let Some(bounds) = ctx.get_widget(self.text_block).try_get_mut::<Rectangle>("bounds") {
        //     bounds.set_x(bounds.x() - delta);
        // }

        // let mut cursor_x_delta = 0.0;
        // let mut scroll_viewer_width = 0.0;

        // {
        //     if let Some(bounds) = ctx
        //         .get_widget(self.scroll_viewer)
        //         .try_get_mut::<Rectangle>("bounds")
        //     {
        //         scroll_viewer_width = bounds.width();
        //     }
        // }

        // {
        //     let mut cursor = ctx.get_widget(self.cursor);

        //     if let Some(margin) = cursor.try_get_mut::<Thickness>("margin") {
        //         CONSOLE.log(format!("ml: {}", margin.left()));
        //         if margin.left() < 0.0 || margin.left() > scroll_viewer_width {
        //             cursor_x_delta = self.cursor_x - margin.left();
        //             margin.set_left(self.cursor_x);
        //         }
        //         self.cursor_x = margin.left();
        //     }

        //     if let Some(bounds) = cursor.try_get_mut::<Rectangle>("bounds") {
        //         bounds.set_x(self.cursor_x);
        //     }
        // }

        // if cursor_x_delta != 0.0 {
        //     {
        //         if let Some(bounds) = ctx
        //             .get_widget(self.text_block)
        //             .try_get_mut::<Rectangle>("bounds")
        //         {
        //             bounds.set_x(bounds.x() + cursor_x_delta);
        //         }
        //     }

        //     if let Some(scroll_offset) = ctx.widget().try_get_mut::<Point>("scroll_offset") {
        //         scroll_offset.x += cursor_x_delta;
        //     }
        // }
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

    fn move_cursor_left(&self, ctx: &mut Context) {
        if *ctx.get_widget(self.cursor).get::<bool>("expanded") {
            self.reset(ctx);
        }

        if let Some(selection) = ctx
            .get_widget(self.cursor)
            .try_get_mut::<TextSelection>("text_selection")
        {
            selection.start_index = (selection.start_index as i32 - 1).max(0) as usize;
            selection.length = 0;
        }
    }

    fn move_cursor_right(&self, ctx: &mut Context) {
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
            selection.start_index = (selection.start_index + 1).min(text_len);
            selection.length = 0;
        }
    }

    fn clear_selection(&self, ctx: &mut Context) {
        let selection = ctx.widget().clone::<TextSelection>("text_selection");

        if let Some(text) = ctx.widget().try_get_mut::<String16>("text") {
            for i in (selection.start_index..(selection.start_index + selection.length)).rev() {
                println!("{}", i);
                text.remove(i);
            }
        }

        ctx.widget()
            .get_mut::<TextSelection>("text_selection")
            .length = 0;
    }

    fn back_space(&self, ctx: &mut Context) {
        if *ctx.get_widget(self.cursor).get::<bool>("expanded") {
            self.clear_selection(ctx);
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
            }
        }

        ctx.push_event_strategy_by_entity(
            ChangedEvent(ctx.entity),
            ctx.entity,
            EventStrategy::Direct,
        );
    }

    fn delete(&self, ctx: &mut Context) {
        self.clear_selection(ctx);

        ctx.push_event_strategy_by_entity(
            ChangedEvent(ctx.entity),
            ctx.entity,
            EventStrategy::Direct,
        );
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

    fn insert_char(&self, key_event: KeyEvent, ctx: &mut Context) {
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
    fn init(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        self.cursor = ctx
            .entity_of_child(ID_CURSOR)
            .expect("TextBoxState.init: cursor child could not be found.");
        self.text_block_root = ctx
            .entity_of_child(ID_TEXT_BLOCK_ROOT)
            .expect("TextBoxState.init: text block root could not be found.");
        // self.scroll_viewer = ctx
        //     .entity_of_child(ID_SCROLL_VIEWER)
        //     .expect("TextBoxState.init: scroll_viewer child could not be found.");
        self.text_block = ctx
            .entity_of_child(ID_TEXT_BLOCK)
            .expect("TextBoxState.init: text_block child could not be found.");
        self.len = ctx.widget().get::<String16>("text").len();
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        self.check_outside_update(ctx);

        if let Some(action) = self.action.clone() {
            match action {
                TextBoxAction::Key(event) => {
                    self.handle_key_event(event, ctx);
                }
                TextBoxAction::Mouse(_p) => {
                    self.request_focus(ctx);
                }
            }
        }

        self.action = None;
        ctx.widget().update_theme_by_state(false);
        self.len = ctx.widget().get::<String16>("text").len();
    }

    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        self.adjust_cursor(ctx);
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

        /// Sets or share the font size property.
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

        // /// Sets or shares the text offset property.
        // scroll_offset: Point,

        // /// Sets or shares the (wheel, scroll) delta property.
        // delta: Point,

        /// Sets or shares the focused property.
        focused: bool,

        /// Sets or shares ta value that describes if the TextBox should lost focus on activation (enter).
        lost_focus_on_activation: bool
    }
);

impl Template for TextBox {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("TextBox")
            .element(ELEMENT_TEXT_BOX)
            .text("")
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(fonts::FONT_SIZE_12)
            .font("Roboto Regular")
            .text_selection(TextSelection::default())
            // .scroll_offset(0.0)
            .padding(4.0)
            .background(colors::LYNCH_COLOR)
            .border_brush("transparent")
            .border_width(0.0)
            .border_radius(2.0)
            .min_width(128.0)
            .height(32.0)
            .focused(false)
            // .delta(0.0)
            .lost_focus_on_activation(true)
            .child(
                MouseBehavior::create()
                    .visibility(id)
                    .enabled(id)
                    .on_mouse_down(move |states, p| {
                        states
                            .get_mut::<TextBoxState>(id)
                            .action(TextBoxAction::Mouse(p));
                        true
                    })
                    .child(
                        Container::create()
                            .background(id)
                            .border_radius(id)
                            .border_width(id)
                            .border_brush(id)
                            .padding(id)
                            .child(
                                Grid::create()
                                    .id(ID_TEXT_BLOCK_ROOT)
                                    .clip(true)
                                    .child(
                                        //     ScrollViewer::create()
                                        //         .id(ID_SCROLL_VIEWER)
                                        //         .scroll_offset(id)
                                        //         .scroll_viewer_mode(("custom", "disabled"))
                                        //         .delta(id)
                                        //         .child(
                                        TextBlock::create()
                                            .id(ID_TEXT_BLOCK)
                                            .vertical_alignment("center")
                                            .foreground(id)
                                            .text(id)
                                            .water_mark(id)
                                            .font(id)
                                            .font_size(id)
                                            .build(ctx),
                                        // )
                                        // .build(ctx),
                                    )
                                    .child(
                                        Cursor::create()
                                            .id(ID_CURSOR)
                                            .margin(0.0)
                                            .horizontal_alignment("start")
                                            .text(id)
                                            .font(id)
                                            .font_size(id)
                                            // .scroll_offset(id)
                                            .focused(id)
                                            .text_selection(id)
                                            .build(ctx),
                                    )
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
