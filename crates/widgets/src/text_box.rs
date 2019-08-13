use std::cell::Cell;

use super::behaviors::FocusBehavior;
use crate::{
    prelude::*,
    shell::{Key, KeyEvent},
};

/// The `TextBoxState` handles the text processing of the `TextBox` widget.
#[derive(Default)]
pub struct TextBoxState {
    text: RefCell<String16>,
    focused: Cell<bool>,
    updated: Cell<bool>,
    selection_start: Cell<usize>,
    selection_length: Cell<usize>,
    cursor_x: Cell<f64>,
}

impl TextBoxState {
    // fn click(&self, point: Point) {
    //     println!("Clicked text box point: ({}, {})", point.x, point.y);
    // }

    fn update_selection_start(&self, selection: i32) {
        self.selection_start
            .set(selection.max(0).min(self.text.borrow().len() as i32) as usize);
    }

    fn update_text(&self, key_event: KeyEvent) -> bool {
        if !self.focused.get() {
            return false;
        }

        if !key_event.text.is_empty() {
            (*self.text.borrow_mut()).insert_str(self.selection_start.get(), &key_event.text);
            self.update_selection_start(self.selection_start.get() as i32 + 1);
        } else {
            match key_event.key {
                Key::Left => {
                    self.update_selection_start(self.selection_start.get() as i32 - 1);
                    self.selection_length.set(0);
                }
                Key::Right => {
                    self.update_selection_start(self.selection_start.get() as i32 + 1);
                    self.selection_length.set(0);
                }
                Key::Backspace => {
                    if self.text.borrow().len() > 0 {
                        if self.selection_start.get() > 0 {
                            for _ in 0..(self.selection_length.get() + 1) {
                                (*self.text.borrow_mut()).remove(self.selection_start.get() - 1);
                            }
                            self.update_selection_start(self.selection_start.get() as i32 - 1);
                        }
                    }
                }
                Key::Delete => {
                    let len = self.text.borrow().len();
                    if len > 0 {
                        if self.selection_start.get() < len {
                            for _ in 0..(self.selection_length.get() + 1) {
                                (*self.text.borrow_mut()).remove(self.selection_start.get());
                            }
                            self.update_selection_start(self.selection_start.get() as i32);
                        }
                    }
                }
                _ => {}
            }
        }

        self.updated.set(true);

        true
    }
}

impl State for TextBoxState {
    fn update(&self, context: &mut Context<'_>) {
        let mut widget = context.widget();

        self.focused.set(widget.get::<Focused>().0);

        if let Some(text) = widget.try_get_mut::<Text>() {
            if text.0 != *self.text.borrow() {
                if self.updated.get() {
                    text.0 = self.text.borrow().clone();
                } else {
                    let text_length = self.text.borrow().len();
                    let origin_text_length = String16::from(text.0.to_string().as_str()).len();
                    let delta = text_length as i32 - origin_text_length as i32;

                    *self.text.borrow_mut() = String16::from(text.0.to_string().as_str());

                    // adjust cursor position after label is changed from outside
                    if text_length < origin_text_length {
                        self.update_selection_start(self.selection_start.get() as i32 - delta);
                    } else {
                        self.update_selection_start(self.selection_start.get() as i32 + delta);
                    }
                }

                self.updated.set(false);
            }
        }

        if let Some(selection) = widget.try_get_mut::<TextSelection>() {
            selection.0.start_index = self.selection_start.get();
            selection.0.length = self.selection_length.get();
        }

        if widget.get::<Text>().0.is_empty() {
            add_selector_to_widget("water-mark", &mut widget);
        } else {
            remove_selector_from_widget("water-mark", &mut widget);
        }

        context.update_theme_properties(context.entity);
    }

    fn update_post_layout(&self, context: &mut Context<'_>) {
        let mut cursor_x_delta = 0.0;
        let mut scroll_viewer_width = 0.0;

        {
            let scroll_viewer = context.child_by_id("scroll_viewer");

            if let Some(bounds) = scroll_viewer.unwrap().try_get_mut::<Bounds>() {
                scroll_viewer_width = bounds.width();
            }
        }

        // maybe not use scroll viewer here

        // Adjust offset of text and cursor if cursor position is out of bounds

        {
            let mut cursor = context.child_by_id("cursor").unwrap();

            if let Some(margin) = cursor.try_get_mut::<Margin>() {
                if margin.left() < 0.0 || margin.left() > scroll_viewer_width {
                    cursor_x_delta = self.cursor_x.get() - margin.left();
                    margin.set_left(self.cursor_x.get());
                }
                self.cursor_x.set(margin.left());
            }

            if let Some(bounds) = cursor.try_get_mut::<Bounds>() {
                bounds.set_x(self.cursor_x.get());
            }
        }

        if cursor_x_delta != 0.0 {
            {
                let text_block = context.child_by_id("text_block");

                if let Some(bounds) = text_block.unwrap().try_get_mut::<Bounds>() {
                    bounds.set_x(bounds.x() + cursor_x_delta);
                }
            }

            if let Some(offset) = context.widget().try_get_mut::<Offset>() {
                (offset.0).x += cursor_x_delta;
            }
        }
    }
}

widget!(
    /// The `TextBox` widget represents a single line text input widget.
    ///
    /// * CSS element: `text-box`
    TextBox<TextBoxState>: KeyDownHandler {
        /// Sets or shares the text property.
        text: Text,

        /// Sets or shares the placeholder text property.
        placeholder: WaterMark,

        /// Sets or shares the text selection property.
        selection: TextSelection,

        /// Sets or shares the foreground property.
        foreground: Foreground,

        /// Sets or share the font size property.
        font_size: FontSize,

        /// Sets or shares the font property.
        font: Font,

        /// Sets or shares the background property.
        background: Background,

        /// Sets or shares the border radius property.
        border_radius: BorderRadius,

        /// Sets or shares the border thickness property.
        border_thickness: BorderThickness,

        /// Sets or shares the border brush property.
        border_brush: BorderBrush,

        /// Sets or shares the padding property.
        padding: Padding,

        /// Sets or shares the text offset property.
        offset: Offset,

         /// Sets or shares the focused property.
        focused: Focused,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl Template for TextBox {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        let state = self.clone_state();

        self.name("TextBox")
            .selector("text-box")
            .text("")
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(fonts::FONT_SIZE_12)
            .font("Roboto Regular")
            .selection(TextSelectionValue::default())
            .offset(0.0)
            .padding(4.0)
            .background(colors::LYNCH_COLOR)
            .border_brush("transparent")
            .border_thickness(0.0)
            .border_radius(2.0)
            .size(128.0, 32.0)
            .focused(false)
            .child(
                FocusBehavior::create()
                    .enabled(id)
                    .focused(id)
                    .selector(id)
                    .child(
                        Container::create()
                            .background(id)
                            .border_radius(id)
                            .border_thickness(id)
                            .border_brush(id)
                            .padding(id)
                            .child(
                                Grid::create()
                                    .child(
                                        ScrollViewer::create()
                                            .selector(SelectorValue::default().id("scroll_viewer"))
                                            .offset(id)
                                            .scroll_mode(("None", "None"))
                                            .child(
                                                TextBlock::create()
                                                    .selector(
                                                        SelectorValue::default()
                                                            .clone()
                                                            .id("text_block"),
                                                    )
                                                    .vertical_alignment("Center")
                                                    .foreground(id)
                                                    .text(id)
                                                    .font(id)
                                                    .font_size(id)
                                                    .attach_by_source::<WaterMark>(id)
                                                    .build(context),
                                            )
                                            .build(context),
                                    )
                                    .child(
                                        Cursor::create()
                                            .selector(SelectorValue::from("cursor").id("cursor"))
                                            .margin(0.0)
                                            .horizontal_alignment("Start")
                                            .text(id)
                                            .font(id)
                                            .font_size(id)
                                            .offset(id)
                                            .focused(id)
                                            .selection(id)
                                            .build(context),
                                    )
                                    .build(context),
                            )
                            .build(context),
                    )
                    .build(context),
            )
            .on_key_down(move |event: KeyEvent| -> bool { state.update_text(event) })
    }
}
