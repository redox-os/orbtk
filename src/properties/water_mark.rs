/// The `WaterMark` struct represents a string used as placeholder text.
#[derive(Default, Clone)]
pub struct WaterMark(pub String);

impl From<&str> for WaterMark {
    fn from(s: &str) -> WaterMark {
        WaterMark(s.to_string())
    }
}

impl From<String> for WaterMark {
    fn from(s: String) -> WaterMark {
        WaterMark(s)
    }
}
