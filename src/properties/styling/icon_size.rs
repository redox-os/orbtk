/// Represents the size of an icon element.
#[derive(Default, Clone, Copy)]
pub struct IconSize(pub f64);

property!(
    IconSize,
    IconSizeProperty,
    icon_sie,
    shared_icon_size
);

impl From<f64> for IconSize {
    fn from(t: f64) -> Self {
        IconSize(t)
    }
}
