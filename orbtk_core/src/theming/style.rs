use std::collections::HashMap;

use super::ThemeState;

use ron::Value;

/// A style is used internaly by `Theme`. It contains a map of default
/// properties beside a list of states.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Style {
    /// Represents the map of default properties.
    pub properties: HashMap<String, Value>,

    /// Represents the list of states.
    pub states: Vec<ThemeState>,
}

impl Style {
    /// Creates a new style.
    pub fn new() -> Self {
        Style::default()
    }
}
