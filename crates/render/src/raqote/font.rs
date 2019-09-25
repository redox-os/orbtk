use rusttype;

use crate::utils::{Brush, Color};

use super::Image;

#[derive(Debug, Clone)]
pub struct Font {
    inner: rusttype::Font<'static>,
}

impl Font {
    pub fn from_bytes(bytes: &'static [u8]) -> Result<Self, String> {
        if let Ok(font) = rusttype::Font::from_bytes(bytes) {
            return Ok(Font { inner: font });
        }

        Err("Could not load font from bytes".to_string())
    }

    pub fn measure_text(&self, text: &str, size: f64) -> (f64, f64) {
        (0.0, 0.0)
    }

    pub fn render_text(&self, text: &str, size: f64, data: &mut [u32], brush: &Brush, width: f64) {
        let scale = rusttype::Scale::uniform(size as f32);

        // The origin of a line of text is at the baseline (roughly where non-descending letters sit).
        // We don't want to clip the text, so we shift it down with an offset when laying it out.
        // v_metrics.ascent is the distance between the baseline and the highest edge of any glyph in
        // the font. That's enough to guarantee that there's no clipping.
        let v_metrics = self.inner.v_metrics(scale);
        let offset = rusttype::point(0.0, v_metrics.ascent);

        // Glyphs to draw for "RustType". Feel free to try other strings.
        let glyphs: Vec<rusttype::PositionedGlyph> =
            self.inner.layout(text, scale, offset).collect();

        let col = match brush {
            Brush::SolidColor(color) => color.clone(),
            _ => Color::from("#000000"),
        };

        for g in glyphs.iter() {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|off_x, off_y, v| {
                    let off_x = off_x as i32 + bb.min.x;
                    let off_y = off_y as i32 + bb.min.y;

                    // Alpha blending from orbclient
                    {
                        let alpha = (v * 255.0) as u32;
                        let new = (alpha << 24) | (col.data & 0xFFFFFF);
                        let old = &mut data[(off_y * width as i32 + off_x) as usize];

                        if alpha >= 255 {
                            *old = new;
                        } else if alpha > 0 {
                            let n_alpha = 255 - alpha;
                            let rb = ((n_alpha * (*old & 0x00FF00FF)) + (alpha * (new & 0x00FF00FF))) >> 8;
                            let ag = (n_alpha * ((*old & 0xFF00FF00) >> 8))
                                + (alpha * (0x01000000 | ((new & 0x0000FF00) >> 8)));

                            *old = (rb & 0x00FF00FF) | (ag & 0xFF00FF00);
                        }
                    }
                });
            }
        }
    }
}
