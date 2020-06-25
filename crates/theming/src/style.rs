use std::collections::HashMap;

use ron::Value;

#[derive(Debug, Clone, Default)]
pub struct Style {
    pub properties: HashMap<String, Value>,
    pub states: HashMap<String, HashMap<String, Value>>,
}
