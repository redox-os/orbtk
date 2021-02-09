use crate::{api::prelude::*, proc_macros::*};

widget!(
    /// The `Cursor` widget represents a text cursor that is used to mark text.
    ///
    /// **style:** `cursor`
    ///
    /// # Example
    ///
    /// Create a cursor and share its text selection.
    ///
    /// ```rust
    /// Cursor::new().selection(id).build(ctx)
    /// ```
    Cursor {
        /// Defines the selection of that cursor that is shared with the `TextBehavior`.
        selection: TextSelection,

        /// Defines the background of the whole selection range.
        background: Brush,

        /// Defines the brush of the text selection indicator.
        border_brush: Brush,

        /// Defines the with of the text selection indicator.
        border_width: Thickness,

        // Defines the opacity of the background of the selection range.
        background_opacity: f32,

        /// Defines the current width of the selection.
        selection_width: f64,

        /// Defines the start position of the current selection.
        selection_x: f64,

        /// Defines the x position of the cursor.
        cursor_x: f64,

        /// Defines the of the cursor.
        offset: f64
    }
);

impl Template for Cursor {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Cursor")
            .style("cursor")
            .background_opacity(0.3)
            .background("transparent")
            .h_align("stretch")
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        CursorRenderObject.into()
    }
}
