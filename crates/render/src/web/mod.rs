use stdweb::{
    js,
    web::{CanvasRenderingContext2d, FillRule},
};

use crate::{utils::*, FontConfig, TextMetrics};

pub use self::image::*;

mod image;

/// The RenderContext2D trait, provides the 2D rendering context. It is used for drawing shapes, text, images, and other objects.
pub struct RenderContext2D {
    canvas_render_context_2_d: CanvasRenderingContext2d,
    font_config: FontConfig,
}

impl RenderContext2D {
    /// Creates a new render context 2d.
    pub fn new(canvas_render_context_2_d: CanvasRenderingContext2d) -> Self {
        canvas_render_context_2_d.set_text_baseline(stdweb::web::TextBaseline::Middle);
        RenderContext2D {
            canvas_render_context_2_d,
            font_config: FontConfig::default(),
        }
    }

    // Rectangles

    /// Draws a filled rectangle whose starting point is at the coordinates {x, y} with the specified width and height and whose style is determined by the fillStyle attribute.
    pub fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.canvas_render_context_2_d
            .fill_rect(x, y, width, height);
    }

    /// Draws a rectangle that is stroked (outlined) according to the current strokeStyle and other context settings.
    pub fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.canvas_render_context_2_d
            .stroke_rect(x, y, width, height);
    }

    // Text

    /// Draws (fills) a given text at the given (x, y) position.
    pub fn fill_text(&mut self, text: &str, x: f64, y: f64) {
        self.canvas_render_context_2_d
            .set_text_baseline(stdweb::web::TextBaseline::Middle);
        self.canvas_render_context_2_d.fill_text(
            text,
            x,
            y + self.font_config.font_size.ceil() / 2.0,
            None,
        );
    }

    pub fn measure(
        &mut self,
        text: &str,
        font_size: f64,
        family: impl Into<String>,
    ) -> TextMetrics {
        self.set_font_family(family);
        self.set_font_size(font_size);
        self.measure_text(text)
    }

    /// Returns a TextMetrics object.
    pub fn measure_text(&mut self, text: &str) -> TextMetrics {
        TextMetrics {
            width: self
                .canvas_render_context_2_d
                .measure_text(text)
                .unwrap()
                .get_width(),
            height: self.font_config.font_size.ceil(),
        }
    }

    /// Fills the current or given path with the current file style.
    pub fn fill(&mut self) {
        self.canvas_render_context_2_d.fill(FillRule::default());
    }

    /// Strokes {outlines} the current or given path with the current stroke style.
    pub fn stroke(&mut self) {
        self.canvas_render_context_2_d.stroke();
    }

    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    pub fn begin_path(&mut self) {
        self.canvas_render_context_2_d.begin_path();
    }

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    pub fn close_path(&mut self) {
        self.canvas_render_context_2_d.close_path();
    }

    /// Adds a rectangle to the current path.
    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.canvas_render_context_2_d.rect(x, y, width, height);
    }

    /// Creates a circular arc centered at (x, y) with a radius of radius. The path starts at startAngle and ends at endAngle.
    pub fn arc(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) {
        self.canvas_render_context_2_d
            .arc(x, y, radius, start_angle, end_angle, false);
    }

    /// Begins a new sub-path at the point specified by the given {x, y} coordinates.

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.canvas_render_context_2_d.move_to(x, y);
    }

    /// Adds a straight line to the current sub-path by connecting the sub-path's last point to the specified {x, y} coordinates.
    pub fn line_to(&mut self, x: f64, y: f64) {
        self.canvas_render_context_2_d.line_to(x, y);
    }

    /// Adds a quadratic Bézier curve to the current sub-path.
    pub fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.canvas_render_context_2_d
            .quadratic_curve_to(cpx, cpy, x, y);
    }

    /// Adds a cubic Bézier curve to the current sub-path. It requires three points: the first two are control points and the third one is the end point. The starting point is the latest point in the current path, which can be changed using MoveTo{} before creating the Bézier curve.
    pub fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.canvas_render_context_2_d
            .bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    // Draw image

    /// Draws the image.
    pub fn draw_image(&mut self, image: &Image, x: f64, y: f64) {
        js!(
            var img = document.image_store.image(@{&image.source});

            if(img == null) {
                img = document.image_store.load_image(@{&image.source});
                img.then(
                    function(i) {
                         @{&self.canvas_render_context_2_d}.drawImage(i, @{&x}, @{&y});
                    }
                )
            } else {
                //  @{&self.canvas_render_context_2_d}.drawImage(img, @{&x}, @{&y});
            }
        );
    }

    /// Draws the image with the given size.
    pub fn draw_image_with_size(&mut self, image: &Image, x: f64, y: f64, width: f64, height: f64) {
        js!(
            var img = document.image_store.image(@{&image.source});

            if(img == null) {
                img = document.image_store.load_image(@{&image.source});

                img.then(
                    function(i) {
                         @{&self.canvas_render_context_2_d}.drawImage(i, @{&x}, @{&y}, @{&width}, @{&height});
                    }
                )
            } else {
                 @{&self.canvas_render_context_2_d}.drawImage(img, @{&x}, @{&y});
            }
        );
    }

    /// Draws the given part of the image.
    pub fn draw_image_with_clip_and_size(
        &mut self,
        image: &Image,
        clip_x: f64,
        clip_y: f64,
        clip_width: f64,
        clip_height: f64,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        js!(
            var img = document.image_store.image(@{&image.source});

            if(img == null) {
                img = document.image_store.load_image(@{&image.source});
                img.then(
                    function(i) {
                         @{&self.canvas_render_context_2_d}.drawImage(img, @{&clip_x}, @{&clip_y}, @{&clip_width}, @{&clip_height}, @{&x}, @{&y}, @{&width}, @{&height});
                    }
                )
            } else {
                 @{&self.canvas_render_context_2_d}.drawImage(img, @{&x}, @{&y});
            }
        );
    }

    /// Creates a clipping path from the current sub-paths. Everything drawn after clip() is called appears inside the clipping path only.
    pub fn clip(&mut self) {
        self.canvas_render_context_2_d.clip(FillRule::EvenOdd);
    }

    // Line styles

    /// Sets the thickness of lines.
    pub fn set_line_width(&mut self, line_width: f64) {
        self.canvas_render_context_2_d.set_line_width(line_width);
    }

    /// Specific the font family.
    pub fn set_font_family(&mut self, family: impl Into<String>) {
        self.font_config.family = family.into().replace(" Regular", "");
        self.canvas_render_context_2_d
            .set_font(&self.font_config.to_string());
    }

    /// Specifies the font size.
    pub fn set_font_size(&mut self, size: f64) {
        self.font_config.font_size = size;
        self.canvas_render_context_2_d
            .set_font(&self.font_config.to_string());
    }

    // Fill and stroke style

    /// Specifies the fill color to use inside shapes.
    pub fn set_fill_style(&mut self, brush: Brush) {
        match brush {
            Brush::SolidColor(color) => {
                self.canvas_render_context_2_d
                    .set_fill_style_color(&color.to_string());
            }
            _ => (),
        }
    }

    /// Specifies the fill stroke to use inside shapes.
    pub fn set_stroke_style(&mut self, brush: Brush) {
        match brush {
            Brush::SolidColor(color) => {
                self.canvas_render_context_2_d
                    .set_stroke_style_color(&color.to_string());
            }
            _ => (),
        }
    }

    // Transformations

    /// Sets the tranformation.
    pub fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.canvas_render_context_2_d
            .set_transform(a, b, c, d, e, f);
    }

    // Canvas states

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    pub fn save(&mut self) {
        self.canvas_render_context_2_d.save();
    }

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. If there is no saved state, this method does nothing.
    pub fn restore(&mut self) {
        self.canvas_render_context_2_d.restore();
    }

    pub fn clear(&mut self, brush: &Brush) {
        let color = match brush {
            Brush::SolidColor(color) => color.to_string(),
            _ => Color::rgba(0, 0, 0, 0).to_string(),
        };

        self.save();
        self.canvas_render_context_2_d
            .set_fill_style_color(color.as_str());
        let canvas = self.canvas_render_context_2_d.get_canvas();
        self.canvas_render_context_2_d.fill_rect(
            0.0,
            0.0,
            canvas.width() as f64,
            canvas.height() as f64,
        );
        self.restore();
    }

    pub fn set_canvas_render_context_2d(
        &mut self,
        canvas_render_context_2_d: CanvasRenderingContext2d,
    ) {
        self.canvas_render_context_2_d = canvas_render_context_2_d;
    }

    pub fn start(&mut self) {}
    pub fn finish(&mut self) {}
}

// --- Conversions ---

impl From<&str> for Image {
    fn from(s: &str) -> Image {
        Image::new(s)
    }
}

impl From<String> for Image {
    fn from(s: String) -> Image {
        Image::new(s)
    }
}
