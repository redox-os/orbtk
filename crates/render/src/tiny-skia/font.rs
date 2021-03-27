use rusttype::OutlineBuilder;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

#[derive(Debug)]
struct GlyphTracer {
    path_builder: PathBuilder,
    position: rusttype::Point<f32>,
}

impl GlyphTracer {
    #[inline(always)]
    fn map_point(&self, x: f32, y: f32) -> (f32, f32) {
        (self.position.x + x, self.position.y + y)
    }
}

impl OutlineBuilder for GlyphTracer {
    fn move_to(&mut self, x: f32, y: f32) {
        let (x, y) = self.map_point(x, y);
        self.path_builder.move_to(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let (x, y) = self.map_point(x, y);
        self.path_builder.line_to(x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let (x, y) = self.map_point(x, y);
        let (x1, y1) = self.map_point(x1, y1);
        self.path_builder.quad_to(x1, y1, x, y);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let (x, y) = self.map_point(x, y);
        let (x1, y1) = self.map_point(x1, y1);
        let (x2, y2) = self.map_point(x2, y2);
        self.path_builder.cubic_to(x1, y1, x2, y2, x, y);
    }

    fn close(&mut self) {
        self.path_builder.close();
    }
}

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
        font_size: f64,
        paint: &Paint,
        pixmap: &mut Pixmap,
        position: (f64, f64),
        text: &str,
    ) {
        let scale = rusttype::Scale::uniform(font_size as f32);

        // The origin of a line of text is at the baseline (roughly where non-descending letters sit).
        // We don't want to clip the text, so we shift it down with an offset when laying it out.
        // v_metrics.ascent is the distance between the baseline and the highest edge of any glyph in
        // the font. That's enough to guarantee that there's no clipping.
        let v_metrics = self.inner.v_metrics(scale);
        let offset = rusttype::point(0.0, v_metrics.ascent);

        let glyphs: Vec<rusttype::PositionedGlyph> =
            self.inner.layout(text, scale, offset).collect();

        let mut glyph_tracer = GlyphTracer {
            path_builder: PathBuilder::new(),
            position: rusttype::point(0.0, 0.0),
        };
        for g in glyphs.iter() {
            let mut gpos = match g.pixel_bounding_box() {
                Some(bbox) => rusttype::point(bbox.min.x as f32, bbox.min.y as f32),
                None => {
                    continue;
                }
            };
            gpos.x += position.0 as f32;
            gpos.y += position.1 as f32;
            glyph_tracer.position = gpos;
            g.build_outline(&mut glyph_tracer);
        }
        if let Some(path) = glyph_tracer.path_builder.finish() {
            pixmap.fill_path(&path, paint, FillRule::Winding, Transform::identity(), None);
        }
    }
}
