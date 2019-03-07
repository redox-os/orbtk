/// Represents the font size of a text element.
#[derive(Default, Clone, Copy)]
pub struct FontSize(pub f64);

property!(FontSize, FontSizeProperty, font_size, font_size_prop);

impl From<f64> for FontSize {
    fn from(t: f64) -> Self {
        FontSize(t)
    }
}
