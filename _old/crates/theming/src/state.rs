use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use ron::Value;

/// Contains a list of properties corresponding to the state key.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct State {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub properties: HashMap<String, Value>,
}

impl State {
    /// Creates a new state.
    pub fn new(key: String) -> Self {
        State {
            key,
            ..Default::default()
        }
    }
}
