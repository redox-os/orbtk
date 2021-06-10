use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use ron::Value;

/// Contains a list of properties corresponding to the state key.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ThemeState {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub properties: HashMap<String, Value>,
}

impl ThemeState {
    /// Creates a new state.
    pub fn new(key: String) -> Self {
        ThemeState {
            key,
            ..Default::default()
        }
    }
}
