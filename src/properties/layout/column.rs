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
    pub fn create() -> ColumnBuilder {
        ColumnBuilder::new()
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
            "Auto" | "auto" => Column::create().width(ColumnWidth::Auto).build(),
            _ => Column::create().width(ColumnWidth::Stretch).build(),
        }
    }
}

impl From<f64> for Column {
    fn from(t: f64) -> Self {
        Column::create().width(ColumnWidth::Width(t)).build()
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

    /// Builds the columns.
    pub fn build(self) -> Columns {
        Columns(ColumnsContainer(self.columns))
    }
}

/// Helper struct used inside of the columns Property.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ColumnsContainer(pub Vec<Column>);

property!(
    /// `Columns` describes a list of grid columns.
    Columns(ColumnsContainer)
);

// --- Trait implementations ---

/// Provides additional operations on grid columns.
pub trait ColumnExtension {
    fn create() -> ColumnsBuilder;

    /// Returns the number of elements in the columns list, also referred to as its 'length'.
    fn len(&self) -> usize;

    /// Returns a reference to an column.
    fn get(&self, column: usize) -> Option<&Column>;

    /// Returns a mutable reference to an column.
    fn get_mut(&mut self, column: usize) -> Option<&mut Column>;

    /// Returns an iterator over the slice.
    fn iter(&self) -> Iter<Column>;

    /// Returns a mutable iterator over the slice.
    fn iter_mut(&mut self) -> IterMut<Column>;
}

impl ColumnExtension for Columns {
    /// Creates a new `ColumnsBuilder` object with default values.
    fn create() -> ColumnsBuilder {
        ColumnsBuilder::new()
    }

    /// Returns the number of elements in the columns list, also referred to as its 'length'.
    fn len(&self) -> usize {
        (self.0).0.len()
    }

    /// Returns a reference to an column.
    fn get(&self, column: usize) -> Option<&Column> {
        (self.0).0.get(column)
    }

    /// Returns a mutable reference to an column.
    fn get_mut(&mut self, column: usize) -> Option<&mut Column> {
        (self.0).0.get_mut(column)
    }

    /// Returns an iterator over the slice.
    fn iter(&self) -> Iter<Column> {
        (self.0).0.iter()
    }

    /// Returns a mutable iterator over the slice.
    fn iter_mut(&mut self) -> IterMut<Column> {
        (self.0).0.iter_mut()
    }
}

