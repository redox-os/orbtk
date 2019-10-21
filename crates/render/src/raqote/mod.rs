use std::{cmp, collections::HashMap};

use raqote;

use crate::{utils::*, RenderConfig, RenderPipeline, RenderTarget, TextMetrics};

pub use self::font::*;
pub use self::image::Image;

mod font;
mod image;

/// The RenderContext2D trait, provides the rendering context. It is used for drawing shapes, text, images, and other objects.
pub struct RenderContext2D {
    draw_target: raqote::DrawTarget,
    path: raqote::Path,
    config: RenderConfig,
    saved_config: Option<RenderConfig>,
    fonts: HashMap<String, Font>,

    // hack / work around for faster text clipping
    clip: bool,
    last_rect: (f64, f64, f64, f64),
    clip_rect: Option<(f64, f64, f64, f64)>,
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
            clip: false,
            last_rect: (0.0, 0.0, width, height),
            clip_rect: None,
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
    pub fn fill_text(&mut self, text: &str, x: f64, y: f64) {
        if text.len() == 0 {
            return;
        }

        let color = match self.config.fill_style {
            Brush::SolidColor(color) => color.clone(),
            _ => Color::from("#000000"),
        };

        if color.a() == 0 {
            return;
        }

        if let Some(font) = self.fonts.get(&self.config.font_config.family) {
            let width = self.draw_target.width() as f64;

            if self.clip {
                if let Some(rect) = self.clip_rect {
                    font.render_text_clipped(
                        text,
                        self.config.font_config.font_size,
                        self.draw_target.get_data_mut(),
                        &color,
                        width,
                        x,
                        y,
                        rect,
                    );
                } else {
                    font.render_text(
                        text,
                        self.config.font_config.font_size,
                        self.draw_target.get_data_mut(),
                        &color,
                        width,
                        x,
                        y,
                    );
                }
            } else {
                font.render_text(
                    text,
                    self.config.font_config.font_size,
                    self.draw_target.get_data_mut(),
                    &color,
                    width,
                    x,
                    y,
                );
            }
        }
    }

    /// Returns a TextMetrics object.
    pub fn measure_text(&mut self, text: &str) -> TextMetrics {
        let mut text_metrics = TextMetrics::default();

        if text.len() == 0 {
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
            &raqote::DrawOptions::new(),
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
        self.last_rect = (x, y, width, height);
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

    /// Adds a cubic Bézier curve to the current sub-path. It requires three points: the first two are control points and the third one is the end point. The starting point is the latest point in the current path, which can be changed using MoveTo{} before creating the Bézier curve.
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

    // Draw image

    fn draw_render_target(&mut self, render_target: &RenderTarget, x: f64, y: f64) {
        self.draw_target.draw_image_at(
            x as f32,
            y as f32,
            &raqote::Image {
                data: &render_target.data(),
                width: render_target.width() as i32,
                height: render_target.height() as i32,
            },
            &raqote::DrawOptions::default(),
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
            &raqote::DrawOptions::default(),
        );
    }

    /// Draws the given part of the image.
    pub fn draw_image_with_clip(
        &mut self,
        image: &Image,
        clip_x: f64,
        clip_y: f64,
        clip_width: f64,
        clip_height: f64,
        x: f64,
        y: f64,
    ) {
        let mut y = y as i32;
        let stride = image.width();
        let mut offset = (clip_y * stride + clip_x) as usize;
        let last_offset = cmp::min(
            ((clip_y + clip_height) * stride + clip_x) as usize,
            image.data().len(),
        );
        while offset < last_offset {
            let next_offset = offset + stride as usize;

            self.draw_target.draw_image_at(
                x as f32,
                y as f32,
                &raqote::Image {
                    data: &image.data()[offset..],
                    width: clip_width as i32,
                    height: 1,
                },
                &raqote::DrawOptions::default(),
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
        pipeline: Box<dyn RenderPipeline>,
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

    /// Specific the font family.
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

    /// Sets the tranformation.
    pub fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.draw_target
            .set_transform(&raqote::Transform::row_major(
                a as f32, b as f32, c as f32, d as f32, e as f32, f as f32,
            ));
    }

    // Canvas states

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    pub fn save(&mut self) {
        self.saved_config = Some(self.config.clone());
    }

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. If there is no saved state, this method does nothing.
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

    pub fn start(&mut self) {}
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
        Brush::SolidColor(color) => {
            return raqote::Source::Solid(raqote::SolidSource {
                r: color.r(),
                g: color.g(),
                b: color.b(),
                a: color.a(),
            });
        }
        Brush::LinearGradient { start, end, stops } => {
            let mut g_stops = vec![];
            for stop in stops {
                g_stops.push(raqote::GradientStop {
                    position: stop.position as f32,
                    color: raqote::Color::new(
                        stop.color.a(),
                        stop.color.r(),
                        stop.color.g(),
                        stop.color.b(),
                    ),
                });
            }

            return raqote::Source::new_linear_gradient(
                raqote::Gradient { stops: g_stops },
                raqote::Point::new(start.x as f32, start.y as f32),
                raqote::Point::new(end.x as f32, start.y as f32),
                raqote::Spread::Pad,
            );
        }
    }
}
