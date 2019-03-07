/// Represents the degree to which the corners of a Border are rounded.
#[derive(Default, Clone, Copy)]
pub struct BorderRadius(pub f64);

property!(
    BorderRadius,
    BorderRadiusProperty,
    border_radius,
    border_radius_prop
);

impl From<f64> for BorderRadius {
    fn from(t: f64) -> Self {
        BorderRadius(t)
    }
}
