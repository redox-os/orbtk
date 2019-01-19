use std::collections::HashMap;
use std::sync::Arc;

use orbfont::Font;

use crate::{
    styling::{fonts::{MATERIAL_ICONS_REGULAR_FONT, ROBOTO_REGULAR_FONT}},
};

pub struct OrbFontRenderer {
    pub fonts: HashMap<&'static str, Font>,
}

lazy_static! {
    pub static ref FONT_RENDERER: Arc<OrbFontRenderer> = {
        let mut fonts = HashMap::new();

        if let Ok(font) = Font::from_data(ROBOTO_REGULAR_FONT.to_vec().into_boxed_slice()) {
            fonts.insert("Roboto Regular", font);
        }

        if let Ok(font) = Font::from_data(MATERIAL_ICONS_REGULAR_FONT.to_vec().into_boxed_slice()) {
            fonts.insert("Material Icons Regular", font);
        }

        Arc::new(OrbFontRenderer { fonts })
    };
}
