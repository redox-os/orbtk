use crate::prelude::*;

// Default state of the `Cursor` widget.
#[derive(Default, AsAny)]
pub struct CursorState;

impl State for CursorState {
    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        let selection_length = ctx.widget().get::<TextSelection>("text_selection").length;

        if selection_length > 0 {
            ctx.widget().set("expanded", true);
            ctx.widget().update_theme_by_state(false);
        } else {
            ctx.widget().set("expanded", false);
            ctx.widget().update_theme_by_state(false);
        }
    }
}

widget!(
    /// The `Cursor` widget represents a text cursor used to mark text.
    ///
    /// **CSS element:** `cursor`
    Cursor<CursorState> {
        /// Sets or shares the text property.
        text: String16,

        /// Sets or shares the text selection property.
        text_selection: TextSelection,

        /// Sets or shares the background property.
        background: Brush,

        /// Sets or share the font size property.
        font_size: f64,

        /// Sets or shares the font property.
        font: String,

        /// Sets or shares the text offset property.
        scroll_offset: Point,

        /// Sets or shares the focused property.
        focused: bool,

        /// Sets or shares the expanded property.
        expanded: bool
    }
);

impl Template for Cursor {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Cursor")
            .width(1.0)
            .element("cursor")
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
