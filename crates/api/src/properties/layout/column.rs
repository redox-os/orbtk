use std::slice::{Iter, IterMut};

/// Used to build a column, specifying additional details.
#[derive(Default)]
pub struct ColumnBuilder {
    width: ColumnWidth,
    min_width: f64,
    max_width: f64,
}

impl ColumnBuilder {
    /// Creates a new `ColumnBuilder` with default values.
    pub fn new() -> Self {
        ColumnBuilder::default()
    }

    /// Inserts a new width.
    pub fn width(mut self, width: ColumnWidth) -> Self {
        self.width = width;
        self
    }

    /// Inserts a new min width.
    pub fn min_width(mut self, min_width: f64) -> Self {
        self.min_width = min_width;
        self
    }

    /// Inserts a new max width.
    pub fn max_width(mut self, max_width: f64) -> Self {
        self.max_width = max_width;
        self
    }

    /// Builds the column.
    pub fn build(self) -> Column {
        Column {
            width: self.width,
            min_width: self.min_width,
            max_width: self.max_width,
            current_width: 0.0,
        }
    }
}

/// Used to define the column of the `Grid`.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Column {
    pub width: ColumnWidth,
    pub min_width: f64,
    pub max_width: f64,
    current_width: f64,
}

impl Column {
    /// Creates a new `ColumnBuilder` object with default values.
    #[inline]
    pub fn new() -> ColumnBuilder {
        ColumnBuilder::new()
    }

    /// Creates a new `ColumnBuilder` object with default values.
    #[inline(always)]
    #[deprecated]
    pub fn create() -> ColumnBuilder {
        Column::new()
    }

    /// Gets the column width.
    pub fn width(&self) -> ColumnWidth {
        self.width
    }

    /// Gets the current width.
    pub fn current_width(&self) -> f64 {
        self.current_width
    }

    /// Sets the current width.
    pub fn set_current_width(&mut self, width: f64) {
        self.current_width = if self.min_width == 0.0 && self.max_width == 0.0 && width > 0.0 {
            width
        } else if width < self.min_width && self.min_width > 0.0 {
            self.min_width
        } else if width > self.max_width && self.max_width > 0.0 {
            self.max_width
        } else {
            width
        };
    }
}

impl From<&str> for Column {
    fn from(t: &str) -> Self {
        match t {
            "Auto" | "auto" => Column::new().width(ColumnWidth::Auto).build(),
            _ => Column::new().width(ColumnWidth::Stretch).build(),
        }
    }
}

impl From<f64> for Column {
    fn from(t: f64) -> Self {
        Column::new().width(ColumnWidth::Width(t)).build()
    }
}

/// Used to define the width of a grid column.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ColumnWidth {
    /// Column is measured by the largest child.
    Auto,

    /// Column expands to the rest available width.
    Stretch,

    /// Defines a fixed size for the column.
    Width(f64),
}

impl Default for ColumnWidth {
    fn default() -> Self {
        ColumnWidth::Stretch
    }
}

/// Used to build a columns, specifying additional details.
#[derive(Default)]
pub struct ColumnsBuilder {
    columns: Vec<Column>,
}

/// Used to build a columns, specifying additional details.
impl ColumnsBuilder {
    /// Creates a new `ColumnsBuilder` with default values.
    pub fn new() -> Self {
        ColumnsBuilder::default()
    }

    /// Inserts a new column.
    pub fn column<C: Into<Column>>(mut self, column: C) -> Self {
        self.columns.push(column.into());
        self
    }

    /// Inserts a list of columns.
    pub fn columns<R: Into<Column> + Clone>(mut self, columns: &[R]) -> Self {
        for column in columns.to_vec() {
            self.columns.push(column.into());
        }
        self
    }

    /// Inserts the given column as often as given.
    pub fn repeat<R: Into<Column> + Copy>(mut self, column: R, count: usize) -> Self {
        for _ in 0..count {
            self.columns.push(column.into())
        }
        self
    }

    /// Builds the columns.
    pub fn build(self) -> Columns {
        Columns(self.columns)
    }
}

impl From<ColumnsBuilder> for Columns {
    fn from(builder: ColumnsBuilder) -> Self {
        builder.build()
    }
}

/// Helper struct used inside of the columns Property.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Columns(pub Vec<Column>);

impl Columns {
    /// Creates a new `ColumnsBuilder` object with default values.
    #[inline(always)]
    pub fn new() -> ColumnsBuilder {
        ColumnsBuilder::new()
    }

    /// Creates a new `ColumnsBuilder` object with default values.
    #[inline(always)]
    #[deprecated]
    pub fn create() -> ColumnsBuilder {
        Columns::new()
    }

    /// Returns the number of elements in the columns list, also referred to as its 'length'.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Is the column empty?
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a reference to an column.
    pub fn get(&self, column: usize) -> Option<&Column> {
        self.0.get(column)
    }

    /// Returns a mutable reference to an column.
    pub fn get_mut(&mut self, column: usize) -> Option<&mut Column> {
        self.0.get_mut(column)
    }

    /// Returns an iterator over the slice.
    pub fn iter(&self) -> Iter<Column> {
        self.0.iter()
    }

    /// Returns a mutable iterator over the slice.
    pub fn iter_mut(&mut self) -> IterMut<Column> {
        self.0.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_width() {
        let width = ColumnWidth::Width(64.0);

        let builder = ColumnBuilder::new();
        let column = builder.width(width).build();

        assert_eq!(column.width, width);
    }

    #[test]
    fn test_min_width() {
        let min_width = 64.0;

        let builder = ColumnBuilder::new();
        let column = builder.min_width(min_width).build();

        assert_eq!(column.min_width, min_width);
    }

    #[test]
    fn test_max_width() {
        let max_width = 64.0;

        let builder = ColumnBuilder::new();
        let column = builder.max_width(max_width).build();

        assert_eq!(column.max_width, max_width);
    }

    #[test]
    fn test_set_current_width() {
        let out_one_width = 10.0;
        let out_two_width = 66.0;
        let in_width = 33.0;
        let min_width = 14.0;
        let max_width = 64.0;

        let builder = ColumnBuilder::new();
        let mut column = builder.min_width(min_width).max_width(max_width).build();

        column.set_current_width(out_one_width);
        assert_eq!(column.current_width(), min_width);

        column.set_current_width(out_two_width);
        assert_eq!(column.current_width(), max_width);

        column.set_current_width(in_width);
        assert_eq!(column.current_width(), in_width);
    }

    #[test]
    fn test_column() {
        let builder = ColumnsBuilder::new();
        let columns = builder.build();

        assert_eq!(columns.len(), 0);

        let builder = ColumnsBuilder::new();
        let columns = builder
            .column(Column::new().build())
            .column(Column::new().build())
            .build();

        assert_eq!(columns.len(), 2);
    }

    #[test]
    fn test_column_width_into() {
        let column: Column = "Auto".into();
        assert_eq!(column.width(), ColumnWidth::Auto);

        let column: Column = "auto".into();
        assert_eq!(column.width(), ColumnWidth::Auto);

        let column: Column = "Stretch".into();
        assert_eq!(column.width(), ColumnWidth::Stretch);

        let column: Column = "stretch".into();
        assert_eq!(column.width(), ColumnWidth::Stretch);

        let column: Column = "*".into();
        assert_eq!(column.width(), ColumnWidth::Stretch);

        let column: Column = "other".into();
        assert_eq!(column.width(), ColumnWidth::Stretch);

        let column: Column = 64.0.into();
        assert_eq!(column.width(), ColumnWidth::Width(64.0));
    }
}
