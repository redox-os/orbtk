use crate::{api::prelude::*, proc_macros::*};

widget!(
    /// The `Grid` defines a flexible grid area that consists of columns and rows.
    ///
    /// **style:** `grid`
    Grid {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the columns property.
        columns: Blocks,

        /// Sets or shares the rows property.
        rows: Blocks,

        /// Sets or shares the border radius property.
        border_radius: f64

        attached_properties: {
            /// Attach a column position to a widget.
            column: usize,

            /// Attach a column span to a widget.
            column_span: usize,

            /// Attach a row position to a widget.
            row: usize,

            /// Attach a row span to a widget.
            row_span: usize
        }
    }
);

impl Grid {
    /// Sets column and row to the given widget and add it as child.
    pub fn place<W>(self, ctx: &mut BuildContext, child: W, column: usize, row: usize) -> Self
    where
        W: Widget,
    {
        self.child(
            child
                .attach(Grid::column(column))
                .attach(Grid::row(row))
                .build(ctx),
        )
    }
}

impl Template for Grid {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Grid")
            .style("grid")
            .border_radius(0.0)
            .background("transparent")
            .rows(Blocks::default())
            .columns(Blocks::default())
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        RectangleRenderObject.into()
    }

    fn layout(&self) -> Box<dyn Layout> {
        GridLayout::new().into()
    }
}
