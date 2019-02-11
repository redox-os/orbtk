/// Represents the font size of an text element.
pub struct FontSize(pub f64);

property!(
    FontSize,
    FontSizeProperty,
    font_size,
    shared_font_size
);


impl From<f64> for FontSize {
    fn from(t: f64) -> Self {
        FontSize(t)
    }
}
