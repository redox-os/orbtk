/// Used to filter stuff such as the `on_changed` callback.
#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    // Everting will be filtered. No element will be available.
    Complete,

    // Nothing will be filtered, all elements will be available.
    Nothing,

    /// Define a list of filtered element.
    List(Vec<String>),
}

impl From<&str> for Filter {
    fn from(s: &str) -> Self {
        match s {
            "nothing" | "Nothing" => Filter::Nothing,
            _ => Filter::Complete,
        }
    }
}

impl From<String> for Filter {
    fn from(s: String) -> Self {
        Filter::from(s.as_str())
    }
}

impl From<Vec<String>> for Filter {
    fn from(v: Vec<String>) -> Self {
        Filter::List(v)
    }
}

impl From<Vec<&str>> for Filter {
    fn from(v: Vec<&str>) -> Self {
        let vec: Vec<String> = v.iter().map(|s| s.to_string()).collect();
        Filter::from(vec)
    }
}

impl Default for Filter {
    fn default() -> Self {
        Filter::Complete
    }
}
