use crate::utils::{Brush, Point, Rectangle, Size};
use raqote::{DrawTarget, PathBuilder};
use rusttype::OutlineBuilder;

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

    /*    pub fn render_text(
        &self,
        text: &str,
        data: &mut [u32],
        transform: &raqote::Transform,
        width: f64,
        height: f64,
        config: (f64, Color, f32),
        position: (f64, f64),
    ) {
        self.render_text_clipped(
            text,
            data,
            transform,
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
        data: &mut [u32],
        transform: &raqote::Transform,
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

        for g in glyphs.iter() {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|off_x, off_y, v| {
                    let off_x = off_x as i32 + bb.min.x;
                    let off_y = off_y as i32 + bb.min.y;

                    if off_x >= 0
                        && off_x < pixel_width
                        && off_y >= 0
                        && off_y < pixel_height
                        && position.0 + off_x as f64 >= clip.x()
                        && position.0 + off_x as f64 <= clip.x() + clip.width()
                        && position.1 + off_y as f64 >= clip.y()
                        && position.1 + off_y as f64 <= clip.y() + clip.height()
                        && position.0 + (off_x as f64) < width
                        && position.1 + (off_y as f64) < height
                    {
                        // Alpha blending from orbclient
                        let alpha = (config.2 * v * 255.0) as u32;
                        let new = (alpha << 24) | (config.1.data & 0x00FF_FFFF);

                        let absolute_position = transform.transform_point(raqote::Point::new(
                            (position.0 + off_x as f64) as f32,
                            (position.1 + off_y as f64) as f32,
                        ));
                        let index = (absolute_position.y as i32 * width as i32
                            + absolute_position.x as i32)
                            as usize;
                        if index >= data.len() {
                            return;
                        }
                        let old = &mut data[index];
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
    }*/

    pub fn render_text(
        &self,
        text: &str,
        draw_target: &mut DrawTarget,
        font_size: f64,
        brush: &Brush,
        global_alpha: f32,
        position: (f64, f64),
    ) {
        let scale = rusttype::Scale::uniform(font_size as f32);

        // The origin of a line of text is at the baseline (roughly where non-descending letters sit).
        // We don't want to clip the text, so we shift it down with an offset when laying it out.
        // v_metrics.ascent is the distance between the baseline and the highest edge of any glyph in
        // the font. That's enough to guarantee that there's no clipping.
        let v_metrics = self.inner.v_metrics(scale);
        let offset = rusttype::point(0.0, v_metrics.ascent);

        // Glyphs to draw for "RustType". Feel free to try other strings.
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
        let width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0)
            .ceil() as f64;
        draw_target.fill(
            &glyph_tracer.path_builder.finish(),
            &super::brush_to_source(
                &brush,
                Rectangle::new(
                    Point::new(position.0, position.1),
                    Size::new(width, font_size.ceil()),
                ),
            ),
            &raqote::DrawOptions {
                alpha: global_alpha,
                ..Default::default()
            },
        );
    }
}
