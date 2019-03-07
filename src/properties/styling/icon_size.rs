/// Represents the size of an icon element.
#[derive(Default, Clone, Copy)]
pub struct IconSize(pub f64);

property!(IconSize, IconSizeProperty, icon_size, icon_size_prop);

impl From<f64> for IconSize {
    fn from(t: f64) -> Self {
        IconSize(t)
    }
}
