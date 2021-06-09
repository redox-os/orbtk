use std::collections::HashMap;

use ron::de::from_str;
use serde_derive::Deserialize;

/// Internal struct used by the `RonLocalization` to parse as language file.
#[derive(Debug, Clone, Deserialize)]
pub struct Dictionary {
    pub words: HashMap<String, String>,
}

impl From<&str> for Dictionary {
    fn from(s: &str) -> Self {
        from_str(s).unwrap()
    }
}
