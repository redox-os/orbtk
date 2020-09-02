use crate::{api::prelude::*, proc_macros::*};

// Default state of the `Cursor` widget.
#[derive(Default, AsAny)]
pub struct CursorState {
    focused: bool,
    expanded: bool,
}

impl State for CursorState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        let cursor = ctx.widget();
        self.focused = *Cursor::focused_ref(&cursor);
        self.expanded = *Cursor::expanded_ref(&cursor);
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        let focused = *Cursor::focused_ref(&ctx.widget());
        let expanded = *Cursor::expanded_ref(&ctx.widget());

        if self.focused == focused && self.expanded == expanded {
            return;
        }

        self.focused = focused;
        self.expanded = expanded;

        Cursor::selector_mut(&mut ctx.widget()).clear_state();

        if self.expanded && self.focused {
            Cursor::selector_mut(&mut ctx.widget()).set_state("expanded");
        } else if self.focused {
            Cursor::selector_mut(&mut ctx.widget()).set_state("focused");
        }

        ctx.widget().update(false);
    }
}

widget!(
    /// The `Cursor` widget represents a text cursor used to mark text.
    ///
    /// **style:** `cursor`
    Cursor<CursorState> {
        /// Sets or shares the text selection property.
        selection: TextSelection,

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
        CursorRenderObject.into()
    }
}
