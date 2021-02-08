/// Represents a selection mode.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SelectionMode {
    None,
    Single,
    Multiple,
}

impl Default for SelectionMode {
    fn default() -> Self {
        SelectionMode::Single
    }
}

impl From<&str> for SelectionMode {
    fn from(t: &str) -> Self {
        match t {
            "Single" | "single" => SelectionMode::Single,
            "Multiple" | "multiple" => SelectionMode::Multiple,
            _ => SelectionMode::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_into() {
        let selection_mode: SelectionMode = "Single".into();
        assert_eq!(selection_mode, SelectionMode::Single);

        let selection_mode: SelectionMode = "single".into();
        assert_eq!(selection_mode, SelectionMode::Single);

        let selection_mode: SelectionMode = "Multiple".into();
        assert_eq!(selection_mode, SelectionMode::Multiple);

        let selection_mode: SelectionMode = "multiple".into();
        assert_eq!(selection_mode, SelectionMode::Multiple);

        let selection_mode: SelectionMode = "None".into();
        assert_eq!(selection_mode, SelectionMode::None);

        let selection_mode: SelectionMode = "none".into();
        assert_eq!(selection_mode, SelectionMode::None);

        let selection_mode: SelectionMode = "other".into();
        assert_eq!(selection_mode, SelectionMode::None);
    }
}
