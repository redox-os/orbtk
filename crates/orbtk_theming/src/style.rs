use std::collections::HashMap;

use ron::Value;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Style {
    pub properties: HashMap<String, Value>,
    pub states: HashMap<String, HashMap<String, Value>>,
}
