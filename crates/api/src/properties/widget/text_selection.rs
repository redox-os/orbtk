/// Is used to mark the selection of a text.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct TextSelection {
    pub start_index: usize,
    pub length: usize,
}

impl From<(usize, usize)> for TextSelection {
    fn from(t: (usize, usize)) -> Self {
        TextSelection {
            start_index: t.0,
            length: t.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into() {
        let offset: TextSelection = (14, 16).into();
        assert_eq!(offset.0.start_index, 14);
        assert_eq!(offset.0.length, 16);
    }
}
