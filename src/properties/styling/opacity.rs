/// Represents the opacity of a widget.
#[derive(Default, Clone, Copy)]
pub struct Opacity(pub f64);

property!(
    Opacity,
    OpacityProperty,
    opacity,
    shared_opacity
);


impl From<f64> for Opacity {
    fn from(t: f64) -> Self {
        Opacity(t)
    }
}
