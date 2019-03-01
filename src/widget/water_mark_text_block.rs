use std::rc::Rc;

use crate::{
    layout::FixedSizeLayout,
    properties::{
        FontProperty, FontSizeProperty, ForegroundProperty, Text, TextProperty, Visibility,
        WaterMark, WaterMarkProperty,
    },
    render_object::TextRenderObject,
    widget::{
        add_selector_to_widget, remove_selector_from_widget, Context, State, Template, Widget,
    },
    styling::{colors, fonts}
};

/// The `WaterMarkTextBlockState` handles the text processing of the `WaterMarkTextBlock` widget.
#[derive(Default)]
pub struct WaterMarkTextBlockState;

impl Into<Rc<dyn State>> for WaterMarkTextBlockState {
    fn into(self) -> Rc<dyn State> {
        Rc::new(self)
    }
}

impl State for WaterMarkTextBlockState {
    fn update(&self, context: &mut Context<'_>) {
        let mut widget = context.widget();

        let mut is_text_empty = false;
        let mut is_water_mark_empty = false;

        if let Ok(text) = widget.borrow_property::<Text>() {
            is_text_empty = text.0.is_empty();
        }

        if is_text_empty {
            add_selector_to_widget("watermark", &mut widget);
        } else {
            remove_selector_from_widget("watermark", &mut widget);
        }

        if let Ok(text) = widget.borrow_property::<WaterMark>() {
            is_water_mark_empty = text.0.is_empty();
        }

        if let Ok(visibility) = widget.borrow_mut_property::<Visibility>() {
            if is_water_mark_empty && is_text_empty {
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Visible;
            }
        }
    }
}

widget!(
    /// The `WaterMarkTextBlock` widget is used to display a placeholder watermark if the `Text` is empty.
    WaterMarkTextBlock
    (
        WaterMarkProperty,
        ForegroundProperty,
        TextProperty,
        FontSizeProperty,
        FontProperty
    )
);

impl Widget for WaterMarkTextBlock {
    fn create() -> Self {
        WaterMarkTextBlock::new()
            .layout(FixedSizeLayout::new())
            .render_object(TextRenderObject)
            .water_mark("Placeholder")
            .selector("watermark")
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(fonts::FONT_SIZE_12)
            .font(fonts::font_into_box(fonts::ROBOTO_REGULAR_FONT))
            .text("")
            .state(Rc::new(WaterMarkTextBlockState::default()))
            .debug_name("WaterMarkTextBlock")
    }
}


