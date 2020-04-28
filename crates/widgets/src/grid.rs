use crate::prelude::*;

widget!(
    /// The `Grid` defines a flexible grid area that consists of columns and rows.
    ///
    /// **CSS element:** `grid`
    Grid {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the columns property.
        columns: Columns,

        /// Sets or shares the rows property.
        rows: Rows,

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

impl Template for Grid {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Grid")
            .element("grid")
            .border_radius(0.0)
            .background("transparent")
            .rows(Rows::default())
            .columns(Columns::default())
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(RectangleRenderObject)
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(GridLayout::new())
    }
}
