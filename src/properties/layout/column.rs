use crate::properties::Constraint;
use std::slice::{Iter, IterMut};

// todo: docu / tests

// todo: column span

#[derive(Default, Copy, Clone, PartialEq)]
pub struct GridColumn(pub usize);

#[derive(Default, Copy, Clone, PartialEq)]
pub struct ColumnSpan(pub usize);

#[derive(Default)]
pub struct ColumnBuilder {
    width: ColumnWidth,
    min_width: f64,
    max_width: f64,
}

impl ColumnBuilder {
    pub fn with_width(mut self, width: ColumnWidth) -> Self {
        self.width = width;
        self
    }

    pub fn with_min_width(mut self, min_width: f64) -> Self {
        self.min_width = min_width;
        self
    }

    pub fn with_max_width(mut self, max_width: f64) -> Self {
        self.max_width = max_width;
        self
    }

    pub fn build(self) -> Column {
        Column {
            width: self.width,
            min_width: self.min_width,
            max_width: self.max_width,
            current_width: 0.0,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Column {
    pub width: ColumnWidth,
    pub min_width: f64,
    pub max_width: f64,
    current_width: f64,
}

impl Column {
    pub fn create() -> ColumnBuilder {
        ColumnBuilder::default()
    }

    pub fn current_width(&self) -> f64 {
        self.current_width
    }

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

#[derive(Copy, Clone, PartialEq)]
pub enum ColumnWidth {
    Auto,
    Stretch,
    Width(f64),
}

impl Default for ColumnWidth {
    fn default() -> Self {
        ColumnWidth::Stretch
    }
}

#[derive(Default)]
pub struct ColumnsBuilder {
    column_definitions: Vec<Column>,
}

impl ColumnsBuilder {
    pub fn with(mut self, column_definition: Column) -> Self {
        self.column_definitions.push(column_definition);
        self
    }

    pub fn build(self) -> Columns {
        Columns {
            value: self.column_definitions,
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct Columns {
    value: Vec<Column>,
}

impl Columns {
    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn get(&self, column: usize) -> Option<&Column> {
        self.value.get(column)
    }

    pub fn get_mut(&mut self, column: usize) -> Option<&mut Column> {
        self.value.get_mut(column)
    }

    pub fn iter(&self) -> Iter<Column> {
        self.value.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Column> {
        self.value.iter_mut()
    }
}

impl Columns {
    pub fn create() -> ColumnsBuilder {
        ColumnsBuilder::default()
    }
}
