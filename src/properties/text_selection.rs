/// The `TextSeleciton` property is used to mark the selection of a text.
#[derive(Default, Copy, Clone)]
pub struct TextSelection {
    pub start_index: usize,
    pub end_index: usize
}