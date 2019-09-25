use std::{collections::HashMap, fmt, path::Path, sync::Arc};

use raqote;

use crate::{utils::*, RenderConfig, TextMetrics};

pub use self::font::*;
pub use self::image::*;

mod font;
mod image;

/// The RenderContext2D trait, provides the 2D rendering context. It is used for drawing shapes, text, images, and other objects.
pub struct RenderContext2D {
    draw_target: raqote::DrawTarget,
    path: raqote::Path,
    config: RenderConfig,
    saved_config: Option<RenderConfig>,
    fonts: HashMap<String, Font>,
}

impl RenderContext2D {
    /// Creates a new render context 2d.
    pub fn new(width: f64, height: f64) -> Self {
        RenderContext2D {
            draw_target: raqote::DrawTarget::new(width as i32, height as i32),
            path: raqote::Path {
                ops: Vec::new(),
                winding: raqote::Winding::NonZero,
            },
            config: RenderConfig::default(),
            saved_config: None,
            fonts: HashMap::new(),
        }
    }

    pub fn resize(&mut self, width: f64, height: f64) {
        self.draw_target = raqote::DrawTarget::new(width as i32, height as i32);
    }

    /// Registers a new font file.
    pub fn register_font(&mut self, family: &str, font_file: &'static [u8]) {
        if self.fonts.contains_key(family) {
            return;
        }

        if let Ok(font) = Font::from_bytes(font_file) {
            self.fonts.insert(family.to_string(), font);
        }
    }

    // Rectangles

    /// Draws a filled rectangle whose starting point is at the coordinates {x, y} with the specified width and height and whose style is determined by the fillStyle attribute.
    pub fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.rect(x, y, width, height);
        self.fill();
    }

    /// Draws a rectangle that is stroked (outlined) according to the current strokeStyle and other context settings.
    pub fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.rect(x, y, width, height);
        self.stroke();
    }

    // Text

    /// Draws (fills) a given text at the given (x, y) position.
    pub fn fill_text(&mut self, text: &str, x: f64, y: f64, _: Option<f64>) {
        if text.len() == 0 {
            return;
        }

        if let Some(font) = self.fonts.get(&self.config.font_config.family) {
            let mut image = Image::new(100, 100);

            font.render_text(text, self.config.font_config.font_size, image.data_mut(), &Brush::from("#ffffff"), 100.0);
            self.draw_image(&mut image, x, y);
        }
    }

    /// Draws (strokes) a given text at the given (x, y) position.
    pub fn stroke_text(&mut self, _: &str, _: f64, _: f64, _: Option<f64>) {}

    /// Returns a TextMetrics object.
    pub fn measure_text(&mut self, text: &str) -> TextMetrics {
        let mut text_metrics = TextMetrics::default();

        if text.len() == 0 {
            return text_metrics;
        }

        text_metrics
    }

    /// Fills the current or given path with the current file style.
    pub fn fill(&mut self) {
        self.draw_target.fill(
            &self.path,
            &brush_to_source(&self.config.fill_style),
            &raqote::DrawOptions::new(),
        );
    }

    /// Strokes {outlines} the current or given path with the current stroke style.
    pub fn stroke(&mut self) {
        self.draw_target.stroke(
            &self.path,
            &brush_to_source(&self.config.stroke_style),
            &raqote::StrokeStyle::default(),
            &raqote::DrawOptions::new(),
        );
    }

    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    pub fn begin_path(&mut self) {
        self.path = raqote::Path {
            ops: Vec::new(),
            winding: raqote::Winding::NonZero,
        };
    }

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    pub fn close_path(&mut self) {
        let mut path_builder = raqote::PathBuilder::from(self.path.clone());
        path_builder.close();
        self.path = path_builder.finish();
    }
    /// Adds a rectangle to the current path.
    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let mut path_builder = raqote::PathBuilder::from(self.path.clone());
        path_builder.rect(x as f32, y as f32, width as f32, height as f32);
        self.path = path_builder.finish();
    }

    /// Creates a circular arc centered at (x, y) with a radius of radius. The path starts at startAngle and ends at endAngle.
    pub fn arc(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64, _: bool) {
        let mut path_builder = raqote::PathBuilder::from(self.path.clone());
        path_builder.arc(
            x as f32,
            y as f32,
            radius as f32,
            start_angle as f32,
            end_angle as f32,
        );
        self.path = path_builder.finish();
    }

    /// Begins a new sub-path at the point specified by the given {x, y} coordinates.

    pub fn move_to(&mut self, x: f64, y: f64) {
        let mut path_builder = raqote::PathBuilder::from(self.path.clone());
        path_builder.move_to(x as f32, y as f32);
        self.path = path_builder.finish();
    }

    /// Adds a straight line to the current sub-path by connecting the sub-path's last point to the specified {x, y} coordinates.
    pub fn line_to(&mut self, x: f64, y: f64) {
        let mut path_builder = raqote::PathBuilder::from(self.path.clone());
        path_builder.line_to(x as f32, y as f32);
        self.path = path_builder.finish();
    }

    /// Adds a quadratic Bézier curve to the current sub-path.
    pub fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        let mut path_builder = raqote::PathBuilder::from(self.path.clone());
        path_builder.quad_to(cpx as f32, cpy as f32, x as f32, y as f32);
        self.path = path_builder.finish();
    }

    /// Adds a cubic Bézier curve to the current sub-path. It requires three points: the first two are control points and the third one is the end point. The starting point is the latest point in the current path, which can be changed using MoveTo{} before creating the Bézier curve.
    pub fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {}

    // Draw image

    /// Draws the image.
    pub fn draw_image(&mut self, image: &mut Image, x: f64, y: f64) {
        self.draw_target.draw_image_at(
            x as f32,
            y as f32,
            &raqote::Image {
                data: &image.data(),
                width: image.width() as i32,
                height: image.height() as i32,
            },
            &raqote::DrawOptions::default(),
        );
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
        self.draw_target.draw_image_with_size_at(
            x as f32,
            y as f32,
            width as f32,
            height as f32,
            &raqote::Image {
                data: image.data(),
                width: image.width() as i32,
                height: image.height() as i32,
            },
            &raqote::DrawOptions::default(),
        );
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
    }

    /// Creates a clipping path from the current sub-paths. Everything drawn after clip() is called appears inside the clipping path only.
    pub fn clip(&mut self) {}

    // Line styles

    /// Sets the thickness of lines.
    pub fn set_line_width(&mut self, line_width: f64) {
        self.config.line_width = line_width;
    }

    /// Specific the font family.
    pub fn set_font_family(&mut self, family: impl Into<String>) {
        self.config.font_config.family = family.into();
    }

    /// Specifies the font size.
    pub fn set_font_size(&mut self, size: f64) {
        self.config.font_config.font_size = size;
    }

    /// Specifies the text alignment.
    pub fn set_text_align(&mut self, _: TextAlignment) {}

    /// Baseline alignment setting.
    pub fn set_text_baseline(&mut self, _: TextBaseline) {
        // println!("fn set_text_baseline is not implemented for orbclient renderer");
    }

    // Fill and stroke style

    /// Specifies the fill color to use inside shapes.
    pub fn set_fill_style(&mut self, fill_style: Brush) {
        self.config.fill_style = fill_style;
    }

    /// Specifies the fill stroke to use inside shapes.
    pub fn set_stroke_style(&mut self, stroke_style: Brush) {
        self.config.stroke_style = stroke_style;
    }

    // Transformations

    /// Multiplies the current transformation with the matrix described by the arguments of this method. You are able to scale, rotate, move and skew the context.
    pub fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {}

    /// Sets the tranformation.
    pub fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {}

    // Canvas states

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    pub fn save(&mut self) {
        self.saved_config = Some(self.config.clone());
    }

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. If there is no saved state, this method does nothing.
    pub fn restore(&mut self) {
        if let Some(config) = &self.saved_config {
            self.config = config.clone();
        }

        self.saved_config = None;
    }

    pub fn data(&self) -> &[u32] {
        self.draw_target.get_data()
    }
}

// --- Conversions ---

impl From<&str> for Image {
    fn from(s: &str) -> Image {
        Image::from_path(s).unwrap()
    }
}

impl From<String> for Image {
    fn from(s: String) -> Image {
        Image::from_path(s).unwrap()
    }
}

// --- Conversions ---

fn brush_to_source<'a>(brush: &Brush) -> raqote::Source<'a> {
    match *brush {
        Brush::SolidColor(color) => {
            return raqote::Source::Solid(raqote::SolidSource {
                r: color.r(),
                g: color.g(),
                b: color.b(),
                a: color.a(),
            })
        }
        _ => {
            return raqote::Source::Solid(raqote::SolidSource {
                r: 0x0,
                g: 0x0,
                b: 0x80,
                a: 0x80,
            })
        }
    }
}
