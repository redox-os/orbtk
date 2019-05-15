use std::collections::HashMap;
use std::sync::Arc;

use orbclient::{Color, Window as OrbWindow};
use orbfont::Font;

use crate::{prelude::*, backend::Renderer};

pub struct OrbFontRenderer {
    pub fonts: HashMap<&'static str, Font>,
}

impl OrbFontRenderer {
    fn render(
        &self,
        text: &str,
        bounds: &Bounds,
        parent_bounds: &Bounds,
        global_position: &Point,
        renderer: &mut OrbWindow,
        font_size: f32,
        color: Color,
        font: &Font,
    ) {
        if font_size > 0.0 {
            let line = font.render(text, font_size);
            line.draw_clipped(
                renderer,
                (global_position.x + bounds.x()) as i32,
                (global_position.y + bounds.y()) as i32,
                global_position.x as i32,
                parent_bounds.width() as u32,
                color,
            );
        }
    }
}

lazy_static! {
    pub static ref FONT_RENDERER: Arc<OrbFontRenderer> = {
        let mut fonts = HashMap::new();

        if let Ok(font) = Font::from_data(fonts::ROBOTO_REGULAR_FONT.to_vec().into_boxed_slice()) {
            fonts.insert("Roboto Regular", font);
        }

        if let Ok(font) = Font::from_data(fonts::MATERIAL_ICONS_REGULAR_FONT.to_vec().into_boxed_slice()) {
            fonts.insert("Material Icons Regular", font);
        }

        Arc::new(OrbFontRenderer { fonts })
    };
}

impl Renderer for OrbWindow {
    fn render_text(
        &mut self,
        text: &str,
        bounds: &Bounds,
        parent_bounds: &Bounds,
        global_position: &Point,
        font_size: u32,
        color: Color,
        font: &Font,
    ) {
        let alpha = (color.data >> 24) & 0xFF;

        if alpha == 0 {
            return;
        }

        FONT_RENDERER.render(
            text,
            bounds,
            parent_bounds,
            global_position,
            self,
            font_size as f32,
            color,
            font,
        );
    }
}
