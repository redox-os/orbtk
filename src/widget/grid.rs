use dces::prelude::Entity;

use crate::{
    theme::Selector,
    layout::{GridLayout, Layout},
    properties::*,
    render_object::{RenderObject, RectangleRenderObject},
    widget::Template,
};

widget!(
    /// The `Grid` defines a flexible grid area that consists of columns and rows.
    /// 
    /// * CSS element: `grid`
    Grid {
        /// Sets or shares the background property.
        background: Background,

        /// Sets or shares the columns property.
        columns: Columns,

        /// Sets or shares the rows property.
        rows: Rows,

        /// Sets or shares the css selector property. 
        selector: Selector
    }
);

impl Template for Grid {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        self.name("Grid")
            .selector("grid")
            .background("transparent")
            .rows(Rows::default())
            .columns(Columns::default())
    }

    fn render_object(&self) -> Option<Box<dyn RenderObject>> {
        Some(Box::new(RectangleRenderObject))
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(GridLayout::new())
    }
}