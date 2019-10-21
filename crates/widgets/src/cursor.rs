use crate::prelude::*;

// Default state of the `Cursor` widget.
#[derive(Default)]
pub struct CursorState;

impl State for CursorState {
    fn update_post_layout(&self, context: &mut Context<'_>) {
        let mut widget = context.widget();

        let selection_length = widget.get::<TextSelection>().0.length;

        if selection_length > 0 {
            add_selector_to_widget("expanded", &mut widget);
        } else {
            remove_selector_from_widget("expanded", &mut widget)
        }

        if widget.get::<Focused>().0 {
            widget.set(Visibility::from("visible"));
        } else {
            widget.set(Visibility::from("collapsed"));
        } 
    }
}

widget!(
    /// The `Cursor` widget represents a text cursor used to mark text.
    /// 
    /// **CSS element:** `cursor`
    Cursor<CursorState> {
        /// Sets or shares the text property.
        text: Text,

        /// Sets or shares the text selection property.
        selection: TextSelection,

        /// Sets or shares the background property.
        background: Background,

        /// Sets or share the font size property.
        font_size: FontSize,

        /// Sets or shares the font property.
        font: Font,

        /// Sets or shares the text offset property.
        scroll_offset: ScrollOffset,

        /// Sets or shares the focused property.
        focused: Focused,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl Template for Cursor {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Cursor")
            .width(1.0)
            .selector("cursor")
            .text("")
            .scroll_offset(0.0)
            .background("transparent")
            .font_size(fonts::FONT_SIZE_12)
            .font("Roboto Regular")
            .focused(false)
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(RectangleRenderObject)
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(TextSelectionLayout::default())
    }
}
