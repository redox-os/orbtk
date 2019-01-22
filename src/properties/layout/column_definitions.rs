// todo: docu / tests

// todo: column span

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Column(pub usize);

#[derive(Copy, Clone, PartialEq)]
pub enum ColumnDefinition {
    Auto,
    Stretch,
    Width(f64),
}

#[derive(Default)]
pub struct ColumnDefinitionsBuilder {
    column_definitions: Vec<ColumnDefinition>,
}

impl ColumnDefinitionsBuilder {
    pub fn with(mut self, column_definition: ColumnDefinition) -> Self {
        self.column_definitions.push(column_definition);
        self
    }

    pub fn build(self) -> ColumnDefinitions {
        ColumnDefinitions {
            value: self.column_definitions,
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct ColumnDefinitions {
    pub value: Vec<ColumnDefinition>,
}

impl ColumnDefinitions {
    pub fn create() -> ColumnDefinitionsBuilder {
        ColumnDefinitionsBuilder::default()
    }
}
