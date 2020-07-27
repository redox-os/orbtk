use crate::prelude::*;

// Default state of the `Cursor` widget.
#[derive(Default, AsAny)]
pub struct CursorState {
    focused: bool,
    expanded: bool,
}

impl State for CursorState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        let cursor = cursor(ctx.widget());
        self.focused = *cursor.focused();
        self.expanded = *cursor.expanded();
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        let mut cursor = cursor(ctx.widget());

        if self.focused == *cursor.focused() && self.expanded == *cursor.expanded() {
            return;
        }

        self.focused = *cursor.focused();
        self.expanded = *cursor.expanded();

        cursor.selector_mut().clear_state();

        if self.expanded && self.focused {
            cursor.selector_mut().set_state("expanded");
        } else if self.focused {
            cursor.selector_mut().set_state("focused");
        }

        cursor.update(false);
    }
}

widget!(
    /// The `Cursor` widget represents a text cursor used to mark text.
    ///
    /// **style:** `cursor`
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
            .style("cursor")
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
