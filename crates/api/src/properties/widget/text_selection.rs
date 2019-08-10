use crate::prelude::*;

/// Is used to mark the selection of a text.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct TextSelectionValue {
    pub start_index: usize,
    pub length: usize,
}

impl From<(usize, usize)> for TextSelectionValue {
    fn from(t: (usize, usize)) -> Self {
        TextSelectionValue {
            start_index: t.0,
            length: t.1,
        }
    }
}

property!(
    // The `TextSelection` property is used to mark the selection of a text.
    TextSelection(TextSelectionValue): (usize, usize)
);
