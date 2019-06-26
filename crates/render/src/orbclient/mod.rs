use std::collections::HashMap;
use std::fmt;

use orbclient::Renderer;
use orbfont::*;
use orbgl::prelude::{CairoRenderEngine, FramebufferSurface};
use orbgl_api::{Canvas, Font};

use crate::{utils::*, FontConfig, TextMetrics};

#[derive(Clone)]
pub struct Image {
    source: String,
    inner: orbgl::prelude::Image,
}

impl Default for Image {
    fn default() -> Self {
        Image {
            source: String::default(),
            inner: orbgl::prelude::Image::new(0, 0),
        }
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Image ( source: {})", self.source)
    }
}

impl std::cmp::PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
    }
}

impl Image {
    /// Constructs a new image with the given source.
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();

        Image {
            inner: orbgl::prelude::Image::from_path(source.clone()).unwrap(),
            source,
        }
    }

    /// Gets the width.
    pub fn width(&self) -> f64 {
        self.inner.width() as f64
    }

    /// Gets the height.
    pub fn height(&self) -> f64 {
        self.inner.height() as f64
    }

    pub fn inner(&mut self) -> &mut orbgl::prelude::Image {
        &mut self.inner
    }
}

/// The RenderContext2D trait, provides the 2D rendering context. It is used for drawing shapes, text, images, and other objects.
pub struct RenderContext2D {
    font_config: FontConfig,
    canvas: Canvas,
    fill_color: orbclient::Color,
    clip_rect: Rect,
    fonts: HashMap<String, Font>,
    clip: bool,
    pub window: orbclient::Window,
}

impl RenderContext2D {
    /// Creates a new render context 2d.
    pub fn new(window: orbclient::Window) -> Self {
        let mut window = window;

        let surface = FramebufferSurface::new(
            window.width(),
            window.height(),
            window.data_mut().as_mut_ptr() as *mut u8,
        );

        let render_engine = CairoRenderEngine::new(surface.clone());

        let canvas = Canvas::new(render_engine.clone());

        RenderContext2D {
            font_config: FontConfig::default(),
            fonts: HashMap::new(),
            fill_color: orbclient::Color::rgb(0, 0, 0),
            canvas,
            window,
            clip: false,
            clip_rect: Rect::default(),
        }
    }

    pub fn register_font(&mut self, family: &str, font_file: &[u8]) {

    }

    // Rectangles

    /// Draws a filled rectangle whose starting point is at the coordinates {x, y} with the specified width and height and whose style is determined by the fillStyle attribute.
    pub fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.canvas.fill_rect(x, y, width, height);
    }

    /// Draws a rectangle that is stroked (outlined) according to the current strokeStyle and other context settings.
    pub fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.canvas.stroke_rect(x, y, width, height);
    }

    // Text

    /// Draws (fills) a given text at the given (x, y) position.
    pub fn fill_text(&mut self, text: &str, x: f64, y: f64, max_width: Option<f64>) {
        if let Some(font) = self.fonts.get(&self.font_config.family) {
            let line = font.render(text, self.font_config.font_size as f32);
            if self.clip {
                line.draw_clipped(
                    &mut self.window,
                    x as i32,
                    y as i32,
                    self.clip_rect.x as i32,
                    self.clip_rect.width as u32,
                    self.fill_color,
                );
            } else {
                 line.draw(
                    &mut self.window,
                    x as i32,
                    y as i32,
                    self.fill_color,
                );
            }
        }
    }

    /// Draws (strokes) a given text at the given (x, y) position.
    pub fn stroke_text(&mut self, text: &str, x: f64, y: f64, max_width: Option<f64>) {}

    /// Returns a TextMetrics object.
    pub fn measure_text(&mut self, text: &str) -> TextMetrics {
        if let Some(font) = self.fonts.get(&self.font_config.family) {
            let text = font.render(text, self.font_config.font_size as f32);
            return TextMetrics {
                width: text.width() as f64,
                height: text.height() as f64,
            };
        }
        TextMetrics {
            width: 0.0,
            height: 0.0,
        }
    }

    /// Fills the current or given path with the current file style.
    pub fn fill(&mut self) {
        self.canvas.fill();
    }

    /// Strokes {outlines} the current or given path with the current stroke style.
    pub fn stroke(&mut self) {
        self.canvas.stroke();
    }

    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    pub fn begin_path(&mut self) {
        self.canvas.stroke();
    }

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    pub fn close_path(&mut self) {
        self.clip = false;
        self.canvas.close_path();
    }

    /// Adds a rectangle to the current path.
    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.canvas.rect(x, y, width, height);
        self.clip_rect = Rect::new(x, y, width, height);
    }

    /// Creates a circular arc centered at (x, y) with a radius of radius. The path starts at startAngle and ends at endAngle.
    pub fn arc(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64, _: bool) {
        self.canvas.arc(x, y, radius, start_angle, end_angle);
    }

    /// Begins a new sub-path at the point specified by the given {x, y} coordinates.

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.canvas.move_to(x, y);
    }

    /// Adds a straight line to the current sub-path by connecting the sub-path's last point to the specified {x, y} coordinates.
    pub fn line_to(&mut self, x: f64, y: f64) {
        self.canvas.line_to(x, y);
    }

    /// Adds a quadratic Bézier curve to the current sub-path.
    pub fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.canvas.quadratic_curve_to(cpx, cpy, x, y);
    }

    /// Adds a cubic Bézier curve to the current sub-path. It requires three points: the first two are control points and the third one is the end point. The starting point is the latest point in the current path, which can be changed using MoveTo{} before creating the Bézier curve.
    pub fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.canvas.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    // Draw image

    /// Draws the image.
    pub fn draw_image(&mut self, image: &mut Image, x: f64, y: f64) {
        self.canvas.draw_image(image.inner(), x, y);
    }

    /// Draws the image with the given size.
    pub fn draw_image_with_size(
        &mut self,
        image: &mut Image,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        self.canvas
            .draw_image_with_size(image.inner(), x, y, width, height);
    }

    /// Draws the given part of the image.
    pub fn draw_image_with_clip_and_size(
        &mut self,
        image: &mut Image,
        clip_x: f64,
        clip_y: f64,
        clip_width: f64,
        clip_height: f64,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        self.canvas.draw_image_with_clip_and_size(
            image.inner(),
            clip_x,
            clip_y,
            clip_width,
            clip_height,
            x,
            y,
            width,
            height,
        );
    }

    /// Creates a clipping path from the current sub-paths. Everything drawn after clip() is called appears inside the clipping path only.
    pub fn clip(&mut self) {
        self.clip = true;
    }

    // Line styles

    /// Sets the thickness of lines.
    pub fn set_line_width(&mut self, line_width: f64) {
        self.canvas.set_line_width(line_width);
    }

    /// Specific the font family.
    pub fn set_font_family(&mut self, family: impl Into<String>) {}

    /// Specifies the font size.
    pub fn set_font_size(&mut self, size: f64) {}

    /// Specifies the text alignment.
    pub fn set_text_align(&mut self, alignment: TextAlignment) {}

    /// Baseline alignment setting.
    pub fn set_text_baseline(&mut self, text_baseline: TextBaseline) {}

    // Fill and stroke style

    /// Specifies the fill color to use inside shapes.
    pub fn set_fill_style(&mut self, brush: Brush) {
        match brush {
            Brush::SolidColor(color) => {
                self.fill_color = orbclient::Color { data: color.data };
                self.canvas
                    .set_fill_style(orbgl::prelude::Color { data: color.data });
            }
            _ => (),
        }
    }

    /// Specifies the fill stroke to use inside shapes.
    pub fn set_stroke_style(&mut self, brush: Brush) {
        match brush {
            Brush::SolidColor(color) => self
                .canvas
                .set_stroke_style(orbgl::prelude::Color { data: color.data }),
            _ => (),
        }
    }

    // Shadows

    pub fn set_shadow_color(&mut self, color: Color) {}

    pub fn set_shadow_offset(&mut self, x: f64, y: f64) {}

    // Transformations

    /// Multiplies the current transformation with the matrix described by the arguments of this method. You are able to scale, rotate, move and skew the context.
    pub fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.canvas.transform(a, b, c, d, e, f);
    }

    /// Sets the tranformation.
    pub fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.canvas.set_transform(a, b, c, d, e, f);
    }

    // Canvas states

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    pub fn save(&mut self) {
        self.canvas.save();
    }

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. If there is no saved state, this method does nothing.
    pub fn restore(&mut self) {
        self.canvas.restore();
    }
}
