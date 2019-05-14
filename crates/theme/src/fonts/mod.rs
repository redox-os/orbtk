use orbfont::Font;

/// Reference to the material icon font.
pub static MATERIAL_ICONS_REGULAR_FONT: &'static [u8; 128180] =
    include_bytes!("MaterialIcons-Regular.ttf");
pub static ROBOTO_REGULAR_FONT: &'static [u8; 145348] = include_bytes!("Roboto-Regular.ttf");

pub fn font_by_key(key: &str) -> Option<Font> {
    match key {
        "Roboto Regular" => {
            return Some(Font::from_data(font_into_box(ROBOTO_REGULAR_FONT)).unwrap());
        }
        "Material Icons Regular" => {
            return Some(Font::from_data(font_into_box(MATERIAL_ICONS_REGULAR_FONT)).unwrap());
        }
        _ => return None,
    }
}

pub fn font_into_box(font: &[u8]) -> Box<[u8]> {
    font.to_vec().into_boxed_slice()
}

// font sizes
pub static FONT_SIZE_12: f64 = 12.0;

// icon sizes
pub static ICON_FONT_SIZE_12: f64 = 12.0;
