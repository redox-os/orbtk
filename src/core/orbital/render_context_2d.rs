use std::collections::HashMap;
use std::sync::Arc;

use orbclient::{Color, Renderer as OrbRenderer, Window as OrbWindow};
use orbfont::Font;
use orbgl::Canvas;
use orbimage::Image;

use crate::{
    core::{
     
        Renderer,
    },
    properties::{Bounds, Point},
    theme::{material_font_icons::MATERIAL_ICONS_REGULAR_FONT, Theme, ROBOTO_REGULAR_FONT, Selector},
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
