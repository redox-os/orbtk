use rusttype;

use crate::utils::Color;

#[derive(Debug, Clone)]
pub struct Font {
    inner: rusttype::Font<'static>,
}

impl Font {
    pub fn from_bytes(bytes: &'static [u8]) -> Result<Self, &'static str> {
        rusttype::Font::from_bytes(bytes)
            .map(|font| Font { inner: font })
            .map_err(|_| "Could not load font from bytes")
    }

    pub fn measure_text(&self, text: &str, size: f64) -> (f64, f64) {
        let scale = rusttype::Scale::uniform(size as f32);
        let v_metrics = self.inner.v_metrics(scale);
        let offset = rusttype::point(0.0, v_metrics.ascent);

        let pixel_height = size.ceil();

        // Glyphs to draw for "RustType". Feel free to try other strings.
        let glyphs: Vec<rusttype::PositionedGlyph> =
            self.inner.layout(text, scale, offset).collect();

        let width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0)
            .ceil() as f64;

        (width, pixel_height)
    }

    pub fn render_text(
        &self,
        text: &str,
        size: f64,
        data: &mut [u32],
        col: Color,
        alpha: f32,
        width: f64,
        x: f64,
        y: f64,
    ) {
        self.render_text_clipped(
            text,
            size,
            data,
            col,
            alpha,
            width,
            x,
            y,
            (0.0, 0.0, width, std::f64::MAX),
        );
    }

    pub fn render_text_clipped(
        &self,
        text: &str,
        size: f64,
        data: &mut [u32],
        col: Color,
        alpha: f32,
        width: f64,
        x: f64,
        y: f64,
        clip_rect: (f64, f64, f64, f64),
    ) {
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

        let pixel_width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0)
            .ceil() as i32;

        let pixel_height = size.ceil() as i32;

        for g in glyphs.iter() {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|off_x, off_y, v| {
                    let off_x = off_x as i32 + bb.min.x;
                    let off_y = off_y as i32 + bb.min.y;

                    if off_x >= 0
                        && off_x < pixel_width
                        && off_y >= 0
                        && off_y < pixel_height
                        && x + off_x as f64 >= clip_rect.0
                        && x + off_x as f64 <= clip_rect.0 + clip_rect.2
                        && y + off_y as f64 >= clip_rect.1
                        && y + off_y as f64 <= clip_rect.1 + clip_rect.3
                    {
                        // Alpha blending from orbclient
                        let alpha = (alpha * v * 255.0) as u32;
                        let new = (alpha << 24) | (col.data & 0x00FF_FFFF);
                        let old = &mut data
                            [((y as i32 + off_y) * width as i32 + x as i32 + off_x) as usize];

                        if alpha >= 255 {
                            *old = new;
                        } else if alpha > 0 {
                            let n_alpha = 255 - alpha;
                            let rb = ((n_alpha * (*old & 0x00FF_00FF))
                                + (alpha * (new & 0x00FF_00FF)))
                                >> 8;
                            let ag = (n_alpha * ((*old & 0xFF00_FF00) >> 8))
                                + (alpha * (0x0100_0000 | ((new & 0x0000_FF00) >> 8)));

                            *old = (rb & 0x00FF_00FF) | (ag & 0xFF00_FF00);
                        }
                    }
                });
            }
        }
    }
}
