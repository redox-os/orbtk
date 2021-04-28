use rusttype::OutlineBuilder;
use tiny_skia::PathBuilder;

#[derive(Debug)]
pub struct GlyphTracer {
    pub path_builder: PathBuilder,
    pub position: rusttype::Point<f32>,
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
