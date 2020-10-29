use tiny_skia::Canvas;

use crate::utils::{Color, Rectangle};

#[derive(Debug, Clone)]
pub struct Font {
    inner: rusttype::Font<'static>,
}

impl Font {
    pub fn from_bytes(bytes: &'static [u8]) -> Result<Self, &'static str> {
        rusttype::Font::try_from_bytes(bytes)
            .map(|font| Font { inner: font })
            .ok_or("Could not load font from bytes")
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
        canvas: &mut Canvas,
        width: f64,
        height: f64,
        config: (f64, Color, f32),
        position: (f64, f64),
    ) {
        self.render_text_clipped(
            text,
            canvas,
            width,
            height,
            config,
            position,
            Rectangle::new((0.0, 0.0), (width, std::f64::MAX)),
        );
    }

    pub fn render_text_clipped(
        &self,
        text: &str,
        canvas: &mut Canvas,
        width: f64,
        height: f64,
        config: (f64, Color, f32),
        position: (f64, f64),
        clip: Rectangle,
    ) {
        let scale = rusttype::Scale::uniform(config.0 as f32);

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

        let pixel_height = config.0.ceil() as i32;

        for g in glyphs.iter() {}
    }
}
