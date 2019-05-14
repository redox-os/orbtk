use std::collections::HashMap;
use std::sync::Arc;

use orbclient::{Color, Renderer as OrbRenderer, Window as OrbWindow};
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

        let x = (bounds.x() + global_position.x).max(parent_bounds.x()) as i32;
        let y = (bounds.y() + global_position.y).max(parent_bounds.y()) as i32;
        let width = bounds.width() as u32; //(bounds.width() as i32).min(parent_bounds.width() as i32) as u32;
        let height = bounds.height() as u32; // (bounds.height() as i32).min(parent_bounds.height() as i32) as u32;

        self.rounded_rect(
            x,
            y,
            bounds.width() as u32,
            height,
            border_radius,
            true,
            background,
        );

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

    fn render_image(
        &mut self,
        image: &[Color],
        bounds: &Bounds,
        parent_bounds: &Bounds,
        global_position: &Point,
    ) {
        let x = (bounds.x() + global_position.x).max(parent_bounds.x()) as i32;
        let y = (bounds.y() + global_position.y).max(parent_bounds.y()) as i32;
        let width = (bounds.width() as i32).min(parent_bounds.width() as i32) as u32;
        let height = (bounds.height() as i32).min(parent_bounds.height() as i32) as u32;

        self.image_fast(x, y, width, height, image);
    }
}
