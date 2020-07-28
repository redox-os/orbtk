use std::collections::HashSet;

/// `SelectedIndices` describes a list of selected indices.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct SelectedIndices(pub HashSet<usize>);

impl From<HashSet<usize>> for SelectedIndices {
    fn from(i: HashSet<usize>) -> Self {
        SelectedIndices(i)
    }
}
