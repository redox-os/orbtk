use std::collections::HashMap;
use std::sync::Arc;

use orbclient::{Color, Renderer as OrbRenderer, Window as OrbWindow};
use orbfont::Font;

use crate::backend::Renderer;
use crate::properties::{Bounds, Point};
use crate::styling::fonts::*;

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
        font: &str,
    ) {
        if let Some(font) = &self.fonts.get(font) {
            let line = font.render(text, font_size);
            line.draw_clipped(
                renderer,
                global_position.x + bounds.x,
                global_position.y + bounds.y,
                global_position.x,
                parent_bounds.width,
                color,
            );
        } else {
            let rect = Bounds::new(
                global_position.x + bounds.x,
                global_position.y + bounds.y,
                bounds.width,
                bounds.height,
            );
            let mut current_rect = Bounds::new(rect.x, rect.y, rect.width, rect.height);
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

        Arc::new(OrbFontRenderer { fonts })
    };
}

impl Renderer for OrbWindow {
    fn render(&mut self, background: Color) {
        // render window background
        self.set(background);
    }

    fn render_rectangle(
        &mut self,
        bounds: &Bounds,
        parent_bounds: &Bounds,
        global_position: &Point,
        border_radius: u32,
        background: Color,
        border_width: u32,
        border_color: Color,
        opacity: f32,
    ) {
        let background = {
            if opacity < 1.0 {
                Color {
                    data: (((opacity * 255.0) as u32) << 24)
                        | ((background.r() as u32) << 16)
                        | ((background.g() as u32) << 8)
                        | (background.b() as u32),
                }
            } else {
                background
            }
        };

        let border_color = {
            if opacity < 1.0 {
                Color {
                    data: (((opacity * 255.0) as u32) << 24)
                        | ((border_color.r() as u32) << 16)
                        | ((border_color.g() as u32) << 8)
                        | (border_color.b() as u32),
                }
            } else {
                border_color
            }
        };

        let x = (bounds.x + global_position.x).max(parent_bounds.x);
        let y = (bounds.y + global_position.y).max(parent_bounds.y);
        let width = (bounds.width as i32).min(parent_bounds.width as i32) as u32;
        let height = (bounds.height as i32).min(parent_bounds.height as i32) as u32;

        self.rounded_rect(x, y, width, height, border_radius, true, background);

        if border_width > 0 {
            self.rounded_rect(x, y, width, height, border_radius, false, border_color);
        }
    }

    fn render_text(
        &mut self,
        text: &str,
        bounds: &Bounds,
        parent_bounds: &Bounds,
        global_position: &Point,
        font_size: u32,
        color: Color,
        font: &str,
    ) {
        // todo handle alpha by orbfong
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

    fn render_image(
        &mut self,
        image: &[Color],
        bounds: &Bounds,
        parent_bounds: &Bounds,
        global_position: &Point,
    ) {
        let x = (bounds.x + global_position.x).max(parent_bounds.x);
        let y = (bounds.y + global_position.y).max(parent_bounds.y);
        let width = (bounds.width as i32).min(parent_bounds.width as i32) as u32;
        let height = (bounds.height as i32).min(parent_bounds.height as i32) as u32;

        self.image_fast(x, y, width, height, image);
    }
}
