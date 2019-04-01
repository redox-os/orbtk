use std::slice::{Iter, IterMut};

/// Used to build a row, specifying additional details.
#[derive(Default)]
pub struct RowBuilder {
    height: RowHeight,
    min_height: f64,
    max_height: f64,
}

impl RowBuilder {
    /// Creates a new `RowBuilder` with default values.
    pub fn new() -> Self {
        RowBuilder::default()
    }

    /// Inserts a new height.
    pub fn height(mut self, height: RowHeight) -> Self {
        self.height = height;
        self
    }

    /// Inserts a new min height.
    pub fn min_height(mut self, min_height: f64) -> Self {
        self.min_height = min_height;
        self
    }

    /// Inserts a new max height.
    pub fn max_height(mut self, max_height: f64) -> Self {
        self.max_height = max_height;
        self
    }

    /// Builds the row.
    pub fn build(self) -> Row {
        Row {
            height: self.height,
            min_height: self.min_height,
            max_height: self.max_height,
            current_height: 0.0,
        }
    }
}

/// Used to define the row of the `Grid`.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Row {
    pub height: RowHeight,
    pub min_height: f64,
    pub max_height: f64,
    current_height: f64,
}

impl Row {
    /// Creates a new `RowBuilder` object with default values.
    pub fn create() -> RowBuilder {
        RowBuilder::new()
    }

    /// Gets the row height.
    pub fn height(&self) -> RowHeight {
        self.height
    }

    /// Gets the current height.
    pub fn current_height(&self) -> f64 {
        self.current_height
    }

    /// Sets the current height.
    pub fn set_current_height(&mut self, height: f64) {
        self.current_height = if self.min_height == 0.0 && self.max_height == 0.0 && height > 0.0 {
            height
        } else if height < self.min_height && self.min_height > 0.0 {
            self.min_height
        } else if height > self.max_height && self.max_height > 0.0 {
            self.max_height
        } else {
            height
        };
    }
}

impl From<&str> for Row {
    fn from(t: &str) -> Self {
        match t {
            "Auto" | "auto" => Row::create().height(RowHeight::Auto).build(),
            _ => Row::create().height(RowHeight::Stretch).build(),
        }
    }
}

impl From<f64> for Row {
    fn from(t: f64) -> Self {
        Row::create().height(RowHeight::Height(t)).build()
    }
}

/// Used to define the height of a grid row.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RowHeight {
    /// Row is measured by the highest child.
    Auto,

    /// Column expands to the rest available height.
    Stretch,

    /// Defines a fixed size for the row.
    Height(f64),
}

impl Default for RowHeight {
    fn default() -> Self {
        RowHeight::Stretch
    }
}

#[derive(Default)]
pub struct RowsBuilder {
    row_definitions: Vec<Row>,
}

/// Used to build a rows, specifying additional details.
impl RowsBuilder {
    /// Creates a new `RowsBuilder` with default values.
    pub fn new() -> Self {
        RowsBuilder::default()
    }

    /// Inserts a new row.
    pub fn row<R: Into<Row>>(mut self, row: R) -> Self {
        self.row_definitions.push(row.into());
        self
    }

    /// Inserts a list of rows.
    pub fn rows<R: Into<Row> + Clone>(mut self, rows: &[R]) -> Self {
        for row in rows.to_vec() {
            self.row_definitions.push(row.into());
        }
        self
    }

    /// Inserts the given row as often as given.
    pub fn repeat<R: Into<Row> + Copy>(mut self, row: R, count: usize) -> Self {
        for _ in 0..count {
            self.row_definitions.push(row.into())
        }
        self
    }

    /// Builds the rows.
    pub fn build(self) -> Rows {
        Rows(RowsContainer(self.row_definitions))
    }
}

/// Helper struct used inside of the row Property.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RowsContainer(Vec<Row>);

property!(
    /// `Rows` describes a list of grid rows.
    Rows(RowsContainer)
);

// --- Trait implementations ---

/// Provides additional operations on grid rows.
pub trait RowExtension {
    /// Returns a new Rows Builder.
    fn create() -> RowsBuilder;

    /// Returns the number of elements in the rows list, also referred to as its 'length'.
    fn len(&self) -> usize;

    /// Returns a reference to an row.
    fn get(&self, row: usize) -> Option<&Row>;

    /// Returns a mutable reference to an row.
    fn get_mut(&mut self, row: usize) -> Option<&mut Row>;

    /// Returns an iterator over the slice.
    fn iter(&self) -> Iter<Row>;

    /// Returns a mutable iterator over the slice.
    fn iter_mut(&mut self) -> IterMut<Row>;
}

impl RowExtension for Rows {
    /// Creates a new `RowsBuilder` object with default values.
    fn create() -> RowsBuilder {
        RowsBuilder::new()
    }

    /// Returns the number of elements in the rows list, also referred to as its 'length'.
    fn len(&self) -> usize {
        (self.0).0.len()
    }

    /// Returns a reference to an row.
    fn get(&self, row: usize) -> Option<&Row> {
        (self.0).0.get(row)
    }

    /// Returns a mutable reference to an row.
    fn get_mut(&mut self, row: usize) -> Option<&mut Row> {
        (self.0).0.get_mut(row)
    }

    /// Returns an iterator over the slice.
    fn iter(&self) -> Iter<Row> {
        (self.0).0.iter()
    }

    /// Returns a mutable iterator over the slice.
    fn iter_mut(&mut self) -> IterMut<Row> {
        (self.0).0.iter_mut()
    }
}

// --- Conversions ---

