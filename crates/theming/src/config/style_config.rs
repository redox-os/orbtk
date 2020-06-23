use std::collections::HashMap;

use ron::Value;
use serde_derive::{Deserialize, Serialize};

/// Defines a style. A style could be base on other styles and contains a list for properties
/// and a list of state properties.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StyleConfig {
    // set default string to base style
    #[serde(default)]
    pub base: String,
    #[serde(default)]
    pub states: HashMap<String, HashMap<String, Value>>,
    #[serde(default)]
    pub properties: HashMap<String, Value>,
}