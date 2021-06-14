/// Mark the selection inside a text object.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct TextSelection {
    start: usize,
    end: usize,
}

impl TextSelection {
    /// Creates a new text selection with an given start and end.
    pub fn new(start: usize, end: usize) -> Self {
        TextSelection { start, end }
    }

    /// Gets the start index.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Sets the start index.
    pub fn set_start(&mut self, start: usize) {
        self.start = start;
    }

    /// Gets the end index.
    pub fn end(&self) -> usize {
        self.end
    }

    /// Sets the end index.
    pub fn set_end(&mut self, end: usize) {
        self.end = end;
    }

    /// Sets start and end to the same value.
    pub fn set(&mut self, value: usize) {
        self.start = value;
        self.end = value;
    }

    /// Gets the length of the selection.
    pub fn len(&self) -> usize {
        (self.start as i32 - self.end as i32).abs() as usize
    }

    /// Check if the selection is empty (start == end).
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl From<(usize, usize)> for TextSelection {
    fn from(t: (usize, usize)) -> Self {
        TextSelection::new(t.0, t.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into() {
        let offset: TextSelection = (14, 16).into();
        assert_eq!(offset.start(), 14);
        assert_eq!(offset.end(), 16);
    }

    #[test]
    fn test_len() {
        let offset: TextSelection = (14, 16).into();
        assert_eq!(offset.len(), 2);

        let offset: TextSelection = (16, 14).into();
        assert_eq!(offset.len(), 2);
    }

    #[test]
    fn test_set() {
        let mut offset: TextSelection = (14, 16).into();
        offset.set(5);

        assert_eq!(offset.start(), 5);
        assert_eq!(offset.end(), 5);
    }
}
