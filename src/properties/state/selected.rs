/// The struct `Selected` represents the current selected state of a widget.
#[derive(Default, Copy, Clone)]
pub struct Selected(pub bool);

property!(Selected, SelectedProperty, selected, shared_selected);

impl From<bool> for Selected {
    fn from(t: bool) -> Self {
        Selected(t)
    }
}
