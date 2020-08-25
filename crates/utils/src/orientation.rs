/// Is used to control the orientation of the `Stack`.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Orientation {
    /// Vertical orientation.
    Vertical,

    /// Horizontal orientation.
    Horizontal,
}

// --- Conversions ---

impl From<&str> for Orientation {
    fn from(t: &str) -> Self {
        match t {
            "Horizontal" | "horizontal" => Orientation::Horizontal,
            _ => Orientation::Vertical,
        }
    }
}

impl Default for Orientation {
    fn default() -> Orientation {
        Orientation::Vertical
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into() {
        let orientation: Orientation = "Vertical".into();
        assert_eq!(orientation, Orientation::Vertical);

        let orientation: Orientation = "vertical".into();
        assert_eq!(orientation, Orientation::Vertical);

        let orientation: Orientation = "Horizontal".into();
        assert_eq!(orientation, Orientation::Horizontal);

        let orientation: Orientation = "horizontal".into();
        assert_eq!(orientation, Orientation::Horizontal);

        let orientation: Orientation = "other".into();
        assert_eq!(orientation, Orientation::Vertical);
    }
}
