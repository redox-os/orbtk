use std::{cmp, collections::HashMap};

use raqote;

use crate::{utils::*, Pipeline, RenderConfig, RenderTarget, TextMetrics};

pub use self::font::*;
pub use self::image::Image;

mod font;
mod image;

/// The RenderContext2D trait, provides the rendering ctx. It is used for drawing shapes, text, images, and other objects.
pub struct RenderContext2D {
    draw_target: raqote::DrawTarget,
    path: raqote::Path,
    config: RenderConfig,
    saved_config: Option<RenderConfig>,
    fonts: HashMap<String, Font>,

    // hack / work around for faster text clipping
    clip: bool,
    last_rect: Rectangle,
    clip_rect: Option<Rectangle>,

    background: Color,
}

impl RenderContext2D {
    /// Creates a new render ctx 2d.
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
            clip: false,
            last_rect: Rectangle::new(0.0, 0.0, width, height),
            clip_rect: None,
            background: Color::default(),
        }
    }

    /// Set the background of the render context.
    pub fn set_background(&mut self, background: Color) {
        self.background = background;
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
        self.draw_target.fill_rect(
            x as f32,
            y as f32,
            width as f32,
            height as f32,
            &brush_to_source(&self.config.fill_style),
            &raqote::DrawOptions {
                alpha: self.config.alpha,
                ..Default::default()
            },
        );
    }

    /// Draws a rectangle that is stroked (outlined) according to the current strokeStyle and other ctx settings.
    pub fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.rect(x, y, width, height);
        self.stroke();
    }

    // Text

    /// Draws (fills) a given text at the given (x, y) position.
    pub fn fill_text(&mut self, text: &str, x: f64, y: f64) {
        if text.is_empty() {
            return;
        }

        let color = match self.config.fill_style {
            Brush::SolidColor(color) => color,
            _ => Color::from("#000000"),
        };

        if color.a() == 0 || self.config.alpha == 0.0 {
            return;
        }

        if let Some(font) = self.fonts.get(&self.config.font_config.family) {
            let width = self.draw_target.width() as f64;

            if self.clip {
                if let Some(rect) = self.clip_rect {
                    font.render_text_clipped(
                        text,
                        self.draw_target.get_data_mut(),
                        width,
                        (self.config.font_config.font_size, color, self.config.alpha),
                        (x, y),
                        rect,
                    );
                } else {
                    font.render_text(
                        text,
                        self.draw_target.get_data_mut(),
                        width,
                        (self.config.font_config.font_size, color, self.config.alpha),
                        (x, y),
                    );
                }
            } else {
                font.render_text(
                    text,
                    self.draw_target.get_data_mut(),
                    width,
                    (self.config.font_config.font_size, color, self.config.alpha),
                    (x, y),
                );
            }
        }
    }

    /// Returns a TextMetrics object.
    pub fn measure_text(&mut self, text: &str) -> TextMetrics {
        let mut text_metrics = TextMetrics::default();

        if text.is_empty() {
            return text_metrics;
        }

        if let Some(font) = self.fonts.get(&self.config.font_config.family) {
            let (width, height) = font.measure_text(text, self.config.font_config.font_size);

            text_metrics.width = width;
            text_metrics.height = height;
        }

        text_metrics
    }

    /// Fills the current or given path with the current file style.
    pub fn fill(&mut self) {
        self.draw_target.fill(
            &self.path,
            &brush_to_source(&self.config.fill_style),
            &raqote::DrawOptions {
                alpha: self.config.alpha,
                ..Default::default()
            },
        );
    }

    /// Strokes {outlines} the current or given path with the current stroke style.
    pub fn stroke(&mut self) {
        self.draw_target.stroke(
            &self.path,
            &brush_to_source(&self.config.stroke_style),
            &raqote::StrokeStyle {
                width: self.config.line_width as f32,
                ..Default::default()
            },
            &raqote::DrawOptions {
                alpha: self.config.alpha,
                ..Default::default()
            },
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
        self.last_rect = Rectangle::new(x, y, width, height);
        let mut path_builder = raqote::PathBuilder::from(self.path.clone());
        path_builder.rect(x as f32, y as f32, width as f32, height as f32);
        self.path = path_builder.finish();
    }

    /// Creates a circular arc centered at (x, y) with a radius of radius. The path starts at startAngle and ends at endAngle.
    pub fn arc(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) {
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

    /// Adds a cubic Bézier curve to the current sub-path.
    /// It requires three points: the first two are control points and the third one is the end point.
    /// The starting point is the latest point in the current path, which can be changed using MoveTo{} before creating the Bézier curve.
    pub fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        let mut path_builder = raqote::PathBuilder::from(self.path.clone());
        path_builder.cubic_to(
            cp1x as f32,
            cp1y as f32,
            cp2x as f32,
            cp2y as f32,
            x as f32,
            y as f32,
        );
    }

    /// Draws a render target.
    pub fn draw_render_target(&mut self, render_target: &RenderTarget, x: f64, y: f64) {
        self.draw_target.draw_image_at(
            x as f32,
            y as f32,
            &raqote::Image {
                data: &render_target.data(),
                width: render_target.width() as i32,
                height: render_target.height() as i32,
            },
            &raqote::DrawOptions {
                alpha: self.config.alpha,
                ..Default::default()
            },
        );
    }

    /// Draws the image.
    pub fn draw_image(&mut self, image: &Image, x: f64, y: f64) {
        self.draw_target.draw_image_at(
            x as f32,
            y as f32,
            &raqote::Image {
                data: &image.data(),
                width: image.width() as i32,
                height: image.height() as i32,
            },
            &raqote::DrawOptions {
                alpha: self.config.alpha,
                ..Default::default()
            },
        );
    }

    /// Draws the given part of the image.
    pub fn draw_image_with_clip(&mut self, image: &Image, clip: Rectangle, x: f64, y: f64) {
        let mut y = y as i32;
        let stride = image.width();
        let mut offset = clip.y.mul_add(stride, clip.x) as usize;
        let last_offset = cmp::min(
            ((clip.y + clip.height).mul_add(stride, clip.x)) as usize,
            image.data().len(),
        );
        while offset < last_offset {
            let next_offset = offset + stride as usize;

            self.draw_target.draw_image_at(
                x as f32,
                y as f32,
                &raqote::Image {
                    data: &image.data()[offset..],
                    width: clip.width as i32,
                    height: 1,
                },
                &raqote::DrawOptions {
                    alpha: self.config.alpha,
                    ..Default::default()
                },
            );
            offset = next_offset;
            y += 1;
        }
    }

    pub fn draw_pipeline(
        &mut self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        pipeline: Box<dyn Pipeline>,
    ) {
        let mut render_target = RenderTarget::new(width as u32, height as u32);
        pipeline.draw_pipeline(&mut render_target);
        self.draw_render_target(&render_target, x, y);
    }

    /// Creates a clipping path from the current sub-paths. Everything drawn after clip() is called appears inside the clipping path only.
    pub fn clip(&mut self) {
        self.clip_rect = Some(self.last_rect);
        self.clip = true;
        self.draw_target.push_clip(&self.path);
    }

    // Line styles

    /// Sets the thickness of lines.
    pub fn set_line_width(&mut self, line_width: f64) {
        self.config.line_width = line_width;
    }

    /// Sets the alpha value,
    pub fn set_alpha(&mut self, alpha: f32) {
        self.config.alpha = alpha;
    }

    /// Specifies the font family.
    pub fn set_font_family(&mut self, family: impl Into<String>) {
        self.config.font_config.family = family.into();
    }

    /// Specifies the font size.
    pub fn set_font_size(&mut self, size: f64) {
        self.config.font_config.font_size = size + 4.0;
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

    /// Sets the transformation.
    pub fn set_transform(
        &mut self,
        h_scaling: f64,
        h_skewing: f64,
        v_skewing: f64,
        v_scaling: f64,
        h_moving: f64,
        v_moving: f64,
    ) {
        self.draw_target
            .set_transform(&raqote::Transform::row_major(
                h_scaling as f32,
                h_skewing as f32,
                v_skewing as f32,
                v_scaling as f32,
                h_moving as f32,
                v_moving as f32,
            ));
    }

    // Canvas states

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    pub fn save(&mut self) {
        self.saved_config = Some(self.config.clone());
    }

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack.
    /// If there is no saved state, this method does nothing.
    pub fn restore(&mut self) {
        self.clip = false;
        self.clip_rect = None;
        self.draw_target.pop_clip();
        if let Some(config) = &self.saved_config {
            self.config = config.clone();
        }

        self.saved_config = None;
    }

    pub fn clear(&mut self, brush: &Brush) {
        let solid = match *brush {
            Brush::SolidColor(color) => raqote::SolidSource {
                r: color.r(),
                g: color.g(),
                b: color.b(),
                a: color.a(),
            },

            _ => raqote::SolidSource {
                r: 0x0,
                g: 0x0,
                b: 0x80,
                a: 0x80,
            },
        };

        self.draw_target.clear(solid);
    }

    pub fn data(&self) -> &[u32] {
        self.draw_target.get_data()
    }

    pub fn data_mut(&mut self) -> &mut [u32] {
        self.draw_target.get_data_mut()
    }

    pub fn data_u8_mut(&mut self) -> &mut [u8] {
        self.draw_target.get_data_u8_mut()
    }

    pub fn start(&mut self) {
        self.clear(&Brush::from(self.background));
    }
    pub fn finish(&mut self) {}
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
    match brush {
        Brush::SolidColor(color) => raqote::Source::Solid(raqote::SolidSource {
            r: color.r(),
            g: color.g(),
            b: color.b(),
            a: color.a(),
        }),
        Brush::LinearGradient { start, end, stops } => {
            let g_stops = stops
                .iter()
                .map(|stop| raqote::GradientStop {
                    position: stop.position as f32,
                    color: raqote::Color::new(
                        stop.color.a(),
                        stop.color.r(),
                        stop.color.g(),
                        stop.color.b(),
                    ),
                })
                .collect();

            raqote::Source::new_linear_gradient(
                raqote::Gradient { stops: g_stops },
                raqote::Point::new(start.x as f32, start.y as f32),
                raqote::Point::new(end.x as f32, start.y as f32),
                raqote::Spread::Pad,
            )
        }
    }
}
