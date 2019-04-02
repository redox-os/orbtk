use crate::prelude::*;

/// Is used to mark the selection of a text.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct TextSelectionValue {
    pub start_index: usize,
    pub length: usize,
}

property!(
    // The `TextSelection` property is used to mark the selection of a text.
    TextSelection(TextSelectionValue)
);

impl From<(usize, usize)> for TextSelection {
    fn from(t: (usize, usize)) -> Self {
        TextSelection(TextSelectionValue {
            start_index: t.0,
            length: t.1,
        })
    }
}
