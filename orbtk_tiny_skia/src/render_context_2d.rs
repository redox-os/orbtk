use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

use atomic_refcell::*;

use orbtk_core::*;

use crate::*;

pub struct TinySkiaRenderContext2D<'a> {
    pix_map: Pixmap,
    font_loader: &'a FontLoader,
}

impl<'a> TinySkiaRenderContext2D<'a> {
    pub fn new(
        width: u32,
        height: u32,
        font_loader: &'a FontLoader,
    ) -> Result<Self, crate::error::Error> {
        Ok(TinySkiaRenderContext2D {
            pix_map: Pixmap::new(width, height)
                .ok_or(crate::error::Error::CannotCreateTinySkiaDisplay)?,
            font_loader,
        })
    }
}

impl<'a> orbtk_core::RenderContext2D for TinySkiaRenderContext2D<'a> {
    fn draw_text(&mut self, text: &str, position: Point, font_size: u32, font_family: &str) {
        if let Some(font) = self.font_loader.font(font_family) {
            let scale = rusttype::Scale::uniform(font_size as f32);

            // The origin of a line of text is at the baseline (roughly where non-descending letters sit).
            // We don't want to clip the text, so we shift it down with an offset when laying it out.
            // v_metrics.ascent is the distance between the baseline and the highest edge of any glyph in
            // the font. That's enough to guarantee that there's no clipping.
            let v_metrics = font.v_metrics(scale);
            let offset = rusttype::point(0.0, v_metrics.ascent);

            let glyphs: Vec<rusttype::PositionedGlyph> = font.layout(text, scale, offset).collect();

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
                gpos.x += position.x() as f32;
                gpos.y += position.y() as f32;
                glyph_tracer.position = gpos;
                g.build_outline(&mut glyph_tracer);
            }

            if let Some(path) = glyph_tracer.path_builder.finish() {
                self.pix_map.fill_path(
                    &path,
                    // todo correct
                    &Paint {
                        shader: tiny_skia::Shader::SolidColor(tiny_skia::Color::WHITE),
                        blend_mode: tiny_skia::BlendMode::default(),
                        anti_alias: true,
                        force_hq_pipeline: false,
                    },
                    FillRule::Winding,
                    Transform::identity(),
                    None,
                );
            }
        }
    }

    fn data(&self) -> &[u8] {
        self.pix_map.data()
    }

    fn data_mut(&mut self) -> &mut [u8] {
        self.pix_map.data_mut()
    }

    fn data_u8_mut(&mut self) -> &mut [u8] {
        self.pix_map.data_mut()
    }
}
