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
        border_radius: f64,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

pub struct AttachedProperty<P>
where
    P: Component + Debug,
{
    pub key: String,
    pub property_source: PropertySource<P>,
}

impl<P> AttachedProperty<P>
where
    P: Component + Debug,
{
    fn new(key: impl Into<String>, property_source: impl IntoPropertySource<P>) -> Self {
        AttachedProperty {
            key: key.into(),
            property_source: property_source.into_source(),
        }
    }
}

impl Grid {
    pub fn column(property: impl IntoPropertySource<usize>) -> AttachedProperty<usize> {
        AttachedProperty::new(stringify!(column), property)
    }

    pub fn column_span(property: impl IntoPropertySource<usize>) -> AttachedProperty<usize> {
        AttachedProperty::new(stringify!(columns_span), property)
    }

    pub fn row(property: impl IntoPropertySource<usize>) -> AttachedProperty<usize> {
        AttachedProperty::new(stringify!(row), property)
    }

    pub fn row_span(property: impl IntoPropertySource<usize>) -> AttachedProperty<usize> {
        AttachedProperty::new(stringify!(row_span), property)
    }
}

impl Template for Grid {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Grid")
            .selector("grid")
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
