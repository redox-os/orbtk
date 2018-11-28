use std::collections::HashMap;
use std::sync::Arc;

use orbclient::{Color, Renderer as OrbRenderer, Window as OrbWindow};
use orbfont::Font;

use backend::Renderer;
use structs::{Point, Rect};
use theme::{ROBOTO_REGULAR_FONT, material_font_icons::MATERIAL_ICONS_REGULAR_FONT};

pub struct OrbFontRenderer {
    pub fonts: HashMap<&'static str, Font>,
}

impl OrbFontRenderer {
    fn render(
        &self,
        text: &str,
        bounds: &Rect,
        parent_bounds: &Rect,
        offset: &Point,
        global_position: &Point,
        renderer: &mut OrbWindow,
        font_size: f32,
        color: Color,
        font: &str,
    ) {
        if let Some(font) = &self.fonts.get(font) {
            let line = font.render(text, font_size);
            line.draw_clipped(
                renderer,
                global_position.x + bounds.x + offset.x,
                global_position.y + bounds.y + offset.y,
                global_position.x,
                parent_bounds.width,
                color,
            );
        } else {
            let rect = Rect::new(
                global_position.x + bounds.x + offset.x,
                global_position.y + bounds.y + offset.y,
                bounds.width,
                bounds.height,
            );
            let mut current_rect = Rect::new(rect.x, rect.y, rect.width, rect.height);
            let x = rect.x;

            for c in text.chars() {
                if c == '\n' {
                    current_rect.x = x;
                    current_rect.y += 16;
                } else {
                    if current_rect.x + 8 >= global_position.x
                        && current_rect.y + 16 >= global_position.y
                        && current_rect.x + 8 < global_position.x + parent_bounds.width as i32
                        && current_rect.y < global_position.y + parent_bounds.height as i32
                    {
                        renderer.char(current_rect.x, current_rect.y, c, color);
                    }
                    current_rect.x += 8;
                }
            }
        }
    }
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

        Arc::new(OrbFontRenderer {
            fonts
        })
    };
}

impl Renderer for OrbWindow {
    fn render(&mut self, background: Color) {
        // render window background
        self.set(background);
    }

    fn render_rectangle(
        &mut self,
        bounds: &Rect,
        parent_bounds: &Rect,
        offset: &Point,
        global_position: &Point,
        border_radius: u32,
        background: Color,
        border_width: u32,
        border_color: Color,
    ) {
        let x = (bounds.x + global_position.x + offset.x).max(parent_bounds.x);
        let y = (bounds.y + global_position.y + offset.y).max(parent_bounds.y);
        let width = (bounds.width as i32 + offset.x).min(parent_bounds.width as i32) as u32;
        let height = (bounds.height as i32 + offset.y).min(parent_bounds.height as i32) as u32;

        self.rounded_rect(x, y, width, height, border_radius, true, background);

        if border_width > 0 {
            self.rounded_rect(x, y, width, height, border_radius, false, border_color);
        }
    }

    fn render_text(
        &mut self,
        text: &str,
        bounds: &Rect,
        parent_bounds: &Rect,
        offset: &Point,
        global_position: &Point,
        font_size: u32,
        color: Color,
        font: &str,
    ) {
        FONT_RENDERER.render(
            text,
            bounds,
            parent_bounds,
            offset,
            global_position,
            self,
            font_size as f32,
            color,
            font,
        );
    }
}
