/// The struct `Focused` represents the current focused state of a widget.
#[derive(Default, Copy, Clone)]
pub struct Focused(pub bool);

property!(Focused, FocusedProperty, focused, shared_focused);

impl From<bool> for Focused {
    fn from(t: bool) -> Self {
       Focused(t)
    }
}