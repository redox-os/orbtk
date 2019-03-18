// /// The struct `MouseOver` represents the current mouse over state of a widget.
// #[derive(Default, Copy, Clone)]
// pub struct MouseOver(pub bool);

// property!(MouseOver, MouseOverProperty, mouse_over, shared_mouse_over);

// impl From<bool> for MouseOver {
//     fn from(t: bool) -> Self {
//         MouseOver(t)
//     }
// }

property!(
    /// `MouseOver` describes the mouse over state of a widget.
    MouseOver(bool)
);