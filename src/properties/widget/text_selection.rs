/// The `TextSelection` property is used to mark the selection of a text.
#[derive(Default, Copy, Clone, PartialEq)]
pub struct TextSelection {
    pub start_index: usize,
    pub length: usize,
}

property!(
    TextSelection,
    TextSelectionProperty,
    text_selection,
    shared_text_selection
);

impl From<(usize, usize)> for TextSelection {
    fn from(t: (usize, usize)) -> Self {
        TextSelection {
            start_index: t.0,
            length: t.1,
        }
    }
}
