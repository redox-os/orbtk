/// The struct `Pressed` represents the current pressed (touch / mouse) state of a widget.
#[derive(Default, Copy, Clone)]
pub struct Pressed(pub bool);

property!(Pressed, PressedProperty, pressed, shared_pressed);

impl From<bool> for Pressed {
    fn from(t: bool) -> Self {
        Pressed(t)
    }
}
