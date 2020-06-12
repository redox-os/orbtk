use crate::prelude::*;

// Default state of the `Cursor` widget.
#[derive(Default, AsAny)]
pub struct CursorState;

impl State for CursorState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        let selection_length = ctx.widget().get::<TextSelection>("text_selection").length;
        let expanded = *ctx.widget().get::<bool>("expanded");

        if selection_length > 0 && !expanded {
            ctx.widget().set("expanded", true);
            ctx.widget().update_theme_by_state(false);
        } else if expanded {
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
        /// Sets or shares the text selection property.
        text_selection: TextSelection,

        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the id of the text block reference.
        text_block: u32,

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
            .background("transparent")
            .h_align("start")
            .focused(false)
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(RectangleRenderObject)
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(TextSelectionLayout::default())
    }
}
