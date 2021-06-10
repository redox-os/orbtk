use std::collections::HashMap;

use ron::Value;
use serde_derive::{Deserialize, Serialize};

use crate::theming::ThemeState;

/// Defines a style. A style could be base on other styles and contains a list for properties
/// and a list of state properties.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StyleConfig {
    // set default string to base style
    #[serde(default)]
    pub base: String,
    #[serde(default)]
    pub states: Vec<ThemeState>,
    #[serde(default)]
    pub properties: HashMap<String, Value>,
}
