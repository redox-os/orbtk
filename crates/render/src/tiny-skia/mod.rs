use smallvec::SmallVec;
use std::{
    collections::HashMap,
    f64::consts::{FRAC_PI_2, PI},
    ptr, slice,
};
use tiny_skia::{
    Canvas, FillRule, Paint, PathBuilder, Pixmap, PixmapPaint, Shader, Stroke, Transform,
};

use crate::{common::*, utils::*, PipelineTrait, RenderConfig, RenderTarget, TextMetrics};

pub use self::font::*;
pub use self::image::Image;

mod font;
mod image;

#[derive(Debug)]
struct State {
    config: RenderConfig,
    path_rect: PathRect,
    clips_count: usize,
    transform: Transform,
}

type StatesOnStack = [State; 2];

pub struct RenderContext2D {
    canvas: Canvas,
    config: RenderConfig,
    saved_states: SmallVec<StatesOnStack>,
    fonts: HashMap<String, Font>,
    path_builder: PathBuilder,
    path_rect: PathRect,
    clips_count: usize,

    background: Color,
    fill_paint: Paint<'static>,
    stroke_paint: Paint<'static>,
}

impl RenderContext2D {
    fn paint_from_brush(brush: &Brush, frame: Rectangle, global_alpha: f32) -> Paint<'static> {
        let shader = match brush {
            Brush::SolidColor(color) => {
                let mut color =
                    tiny_skia::Color::from_rgba8(color.b(), color.g(), color.r(), color.a());
                color.set_alpha(color.alpha() * global_alpha);
                Shader::SolidColor(color)
            }
            Brush::Gradient(Gradient {
                kind: GradientKind::Linear(coords),
                stops,
                repeat,
            }) => {
                let spread = match repeat {
                    true => tiny_skia::SpreadMode::Repeat,
                    false => tiny_skia::SpreadMode::Pad,
                };
                let (start, end) = match coords {
                    LinearGradientCoords::Ends { start, end } => {
                        (*start + frame.position(), *end + frame.position())
                    }
                    LinearGradientCoords::Angle {
                        angle,
                        displacement,
                    } => {
                        let z = linear_gradient_ends_from_angle(*angle, frame.size());
                        let disp = displacement.pixels(frame.size());
                        let start = frame.position() + frame.size() / 2.0 + -z + disp;
                        let end = frame.position() + frame.size() / 2.0 + z + disp;
                        (start, end)
                    }
                    LinearGradientCoords::Direction {
                        direction,
                        displacement,
                    } => {
                        let width = frame.width();
                        let height = frame.height();
                        let (mut start, mut end) = direction.cross(width, height);
                        let displacement = displacement.pixels(frame.size());
                        start = start + frame.position() + displacement;
                        end = end + frame.position() + displacement;
                        (start, end)
                    }
                };
                let g_stops = build_unit_percent_gradient(&stops, end.distance(start), |p, c| {
                    let mut color = tiny_skia::Color::from_rgba8(c.b(), c.g(), c.r(), c.a());
                    color.set_alpha(color.alpha() * global_alpha);
                    tiny_skia::GradientStop::new(p as f32, color)
                });
                let tstart = tiny_skia::Point::from_xy(start.x() as f32, start.y() as f32);
                let tend = tiny_skia::Point::from_xy(end.x() as f32, end.y() as f32);
                tiny_skia::LinearGradient::new(
                    tstart,
                    tend,
                    g_stops,
                    spread,
                    tiny_skia::Transform::identity(),
                )
                .unwrap_or(Shader::SolidColor(tiny_skia::Color::WHITE))
            }
        };
        Paint {
            shader,
            anti_alias: true,
            ..Default::default()
        }
    }

    /// Creates a new render ctx 2d.
    pub fn new(width: f64, height: f64) -> Self {
        let pixmap = Pixmap::new(width as u32, height as u32).unwrap();
        let canvas = Canvas::from(pixmap);
        RenderContext2D {
            canvas,
            config: RenderConfig::default(),
            saved_states: SmallVec::<StatesOnStack>::new(),
            fonts: HashMap::new(),
            path_builder: PathBuilder::new(),
            path_rect: PathRect::new(None),
            clips_count: 0,
            background: Color::default(),
            fill_paint: Self::paint_from_brush(
                &Brush::default(),
                Rectangle::new(Point::new(0.0, 0.0), Size::new(0.0, 0.0)),
                1.0,
            ),
            stroke_paint: Self::paint_from_brush(
                &Brush::default(),
                Rectangle::new(Point::new(0.0, 0.0), Size::new(0.0, 0.0)),
                1.0,
            ),
        }
    }

    /// Set the background of the render context.
    pub fn set_background(&mut self, background: Color) {
        self.background = background;
    }

    pub fn resize(&mut self, width: f64, height: f64) {
        let pixmap = Pixmap::new(width as u32, height as u32).unwrap();
        let canvas = Canvas::from(pixmap);
        self.canvas = canvas;
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
        let rect = match self.path_rect.get_rect() {
            Some(rect) => rect,
            None => return, // The path is empty, do nothing
        };
        self.fill_paint =
            Self::paint_from_brush(&self.config.fill_style, rect, self.config.alpha as f32);
        self.canvas.fill_rect(
            tiny_skia::Rect::from_xywh(x as f32, y as f32, width as f32, height as f32).unwrap(),
            &self.fill_paint,
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

        let tm = self.measure_text(text);
        let rect = Rectangle::new(Point::new(x, y), Size::new(tm.width, tm.height));
        self.fill_paint =
            Self::paint_from_brush(&self.config.fill_style, rect, self.config.alpha as f32);

        if let Some(font) = self.fonts.get(&self.config.font_config.family) {
            let width = self.canvas.pixmap.width() as f64;
            let height = self.canvas.pixmap.height() as f64;

            if let Some(rect) = self.path_rect.get_clip() {
                font.render_text_clipped(
                    text,
                    &mut self.canvas,
                    width,
                    height,
                    self.config.font_config.font_size,
                    &self.fill_paint,
                    (x, y),
                    rect,
                );
            } else {
                font.render_text(
                    text,
                    &mut self.canvas,
                    width,
                    height,
                    self.config.font_config.font_size,
                    &self.fill_paint,
                    (x, y),
                );
            }
        }
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
        let rect = match self.path_rect.get_rect() {
            Some(rect) => rect,
            None => return, // The path is empty, do nothing
        };
        self.fill_paint =
            Self::paint_from_brush(&self.config.fill_style, rect, self.config.alpha as f32);
        if let Some(path) = self.path_builder.clone().finish() {
            self.canvas
                .fill_path(&path, &self.fill_paint, FillRule::EvenOdd);
        }
    }

    /// Strokes {outlines} the current or given path with the current stroke style.
    pub fn stroke(&mut self) {
        let rect = match self.path_rect.get_rect() {
            Some(rect) => rect,
            None => return, // The path is empty, do nothing
        };
        self.stroke_paint =
            Self::paint_from_brush(&self.config.stroke_style, rect, self.config.alpha as f32);
        if let Some(path) = self.path_builder.clone().finish() {
            self.canvas.stroke_path(
                &path,
                &self.stroke_paint,
                &Stroke {
                    width: self.config.line_width as f32,
                    ..Default::default()
                },
            );
        }
    }

    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    pub fn begin_path(&mut self) {
        self.path_builder = PathBuilder::new();
        self.path_rect.rebirth();
    }

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    pub fn close_path(&mut self) {
        self.path_builder.close();
        self.path_rect.record_path_close();
    }

    /// Adds a rectangle to the current path.
    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.path_rect.record_rect(x, y, width, height);
        let x = x as f32;
        let y = y as f32;
        let cx = x + width as f32;
        let cy = y + height as f32;
        self.path_builder.move_to(x, y);
        self.path_builder.line_to(cx, y);
        self.path_builder.line_to(cx, cy);
        self.path_builder.line_to(x, cy);
        self.path_builder.line_to(x, y);
    }

    /// Intern function to draw an arc segment minor or equal to 90°
    fn arc_fragment(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) {
        let (end_sin, end_cos) = end_angle.sin_cos();
        let end_x = x + end_cos * radius;
        let end_y = y + end_sin * radius;
        let (start_sin, start_cos) = start_angle.sin_cos();
        let start_x = x + start_cos * radius;
        let start_y = y + start_sin * radius;

        let t1x = y - start_y;
        let t1y = start_x - x;
        let t2x = end_y - y;
        let t2y = x - end_x;
        let dx = (start_x + end_x) / 2.0 - x;
        let dy = (start_y + end_y) / 2.0 - y;
        let tx = 3.0 / 8.0 * (t1x + t2x);
        let ty = 3.0 / 8.0 * (t1y + t2y);
        let a = tx * tx + ty * ty;
        let b = dx * tx + dy * ty;
        let c = dx * dx + dy * dy - radius * radius;
        let d = b * b - a * c;
        if d > 0.0 {
            let k = (d.sqrt() - b) / a;
            self.path_builder.cubic_to(
                (start_x + k * t1x) as f32,
                (start_y + k * t1y) as f32,
                (end_x + k * t2x) as f32,
                (end_y + k * t2y) as f32,
                (x + end_cos * radius) as f32,
                (y + end_sin * radius) as f32,
            );
        }
    }

    /// Creates a circular arc centered at (x, y) with a radius of radius. The path starts at startAngle and ends at endAngle.
    pub fn arc(&mut self, x: f64, y: f64, radius: f64, mut start_angle: f64, mut end_angle: f64) {
        self.path_rect
            .record_arc(x, y, radius, start_angle, end_angle);
        if start_angle.is_negative() {
            start_angle = TAU - -start_angle;
        }
        if end_angle.is_negative() {
            end_angle = TAU - -end_angle;
        }
        let premult_k = 0.552284749831 * radius;
        let (start_sin, start_cos) = start_angle.sin_cos();
        if end_angle - start_angle < TAU {
            self.path_builder.move_to(x as f32, y as f32);
            self.path_builder.line_to(
                (x + start_cos * radius) as f32,
                (y + start_sin * radius) as f32,
            );
        } else {
            self.path_builder.move_to(
                (x + start_cos * radius) as f32,
                (y + start_sin * radius) as f32,
            );
        }
        if end_angle - start_angle < FRAC_PI_2 {
            self.arc_fragment(x, y, radius, start_angle, end_angle);
            self.path_builder.line_to(x as f32, y as f32);
            return;
        } else if start_angle % FRAC_PI_2 > f64::EPSILON {
            self.arc_fragment(
                x,
                y,
                radius,
                start_angle,
                start_angle + FRAC_PI_2 - start_angle % FRAC_PI_2,
            );
        }
        // Build the four arc quadrants if they are in the range between start_angle and end_angle
        if start_angle <= 0.0 && end_angle >= FRAC_PI_2 {
            self.path_builder.cubic_to(
                (x + radius) as f32,
                (y + premult_k) as f32,
                (x + premult_k) as f32,
                (y + radius) as f32,
                x as f32,
                (y + radius) as f32,
            );
        }
        if start_angle <= FRAC_PI_2 && end_angle >= PI {
            self.path_builder.cubic_to(
                (x - premult_k) as f32,
                (y + radius) as f32,
                (x - radius) as f32,
                (y + premult_k) as f32,
                (x - radius) as f32,
                y as f32,
            );
        }
        if start_angle <= PI && end_angle >= PI + FRAC_PI_2 {
            self.path_builder.cubic_to(
                (x - radius) as f32,
                (y - premult_k) as f32,
                (x - premult_k) as f32,
                (y - radius) as f32,
                x as f32,
                (y - radius) as f32,
            );
        }
        if start_angle <= PI + FRAC_PI_2 && end_angle >= TAU {
            self.path_builder.cubic_to(
                (x + premult_k) as f32,
                (y - radius) as f32,
                (x + radius) as f32,
                (y - premult_k) as f32,
                (x + radius) as f32,
                y as f32,
            );
        }
        self.arc_fragment(x, y, radius, end_angle - end_angle % FRAC_PI_2, end_angle);
        if end_angle - start_angle < TAU {
            self.path_builder.line_to(x as f32, y as f32);
        }
    }

    /// Begins a new sub-path at the point specified by the given {x, y} coordinates.
    pub fn move_to(&mut self, x: f64, y: f64) {
        self.path_builder.move_to(x as f32, y as f32);
        self.path_rect.record_move_to(x, y);
    }

    /// Adds a straight line to the current sub-path by connecting the sub-path's last point to the specified {x, y} coordinates.
    pub fn line_to(&mut self, x: f64, y: f64) {
        self.path_builder.line_to(x as f32, y as f32);
        self.path_rect.record_line_to(x, y);
    }

    /// Adds a quadratic Bézier curve to the current sub-path.
    pub fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.path_builder
            .quad_to(cpx as f32, cpy as f32, x as f32, y as f32);
        self.path_rect.record_quadratic_curve_to(cpx, cpy, x, y);
    }

    /// Draws a render target.
    pub fn draw_render_target(&mut self, render_target: &RenderTarget, x: f64, y: f64) {
        let mut pixmap =
            Pixmap::new(render_target.width() as u32, render_target.height() as u32).unwrap();
        unsafe {
            ptr::copy_nonoverlapping(
                render_target.data().as_ptr() as *const u8,
                pixmap.data_mut().as_mut_ptr(),
                render_target.data().len() * 4,
            )
        };
        self.canvas
            .draw_pixmap(x as i32, y as i32, &pixmap, &PixmapPaint::default());
    }

    /// Draws the image.
    pub fn draw_image(&mut self, image: &Image, x: f64, y: f64) {
        let mut pixmap = Pixmap::new(image.width() as u32, image.height() as u32).unwrap();
        unsafe {
            ptr::copy_nonoverlapping(
                image.data().as_ptr() as *const u8,
                pixmap.data_mut().as_mut_ptr(),
                image.data().len() * 4,
            )
        };
        self.canvas
            .draw_pixmap(x as i32, y as i32, &pixmap, &PixmapPaint::default());
    }

    pub fn draw_pipeline(
        &mut self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        pipeline: Box<dyn PipelineTrait>,
    ) {
        let mut render_target = RenderTarget::new(width as u32, height as u32);
        pipeline.draw_pipeline(&mut render_target);
        self.draw_render_target(&render_target, x, y);
    }

    /// Creates a clipping path from the current sub-paths. Everything drawn after clip() is called appears inside the clipping path only.
    pub fn clip(&mut self) {
        // FIXME
        if let Some(path) = self.path_builder.clone().finish() {
            self.canvas.set_clip_path(&path, FillRule::EvenOdd, true);
        }
        self.path_rect.record_clip();
        self.clips_count += 1;
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
    pub fn set_fill_style(&mut self, fill_style: impl Into<Brush>) {
        self.config.fill_style = fill_style.into();
    }

    /// Specifies the fill stroke to use inside shapes.
    pub fn set_stroke_style(&mut self, stroke_style: impl Into<Brush>) {
        self.config.stroke_style = stroke_style.into();
    }

    // Canvas states

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    pub fn save(&mut self) {
        self.saved_states.push(State {
            config: self.config.clone(),
            path_rect: self.path_rect,
            clips_count: self.clips_count,
            transform: self.canvas.get_transform(),
        });
    }

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack.
    /// If there is no saved state, this method does nothing.
    pub fn restore(&mut self) {
        if let Some(State {
            config,
            path_rect,
            clips_count: former_clips_count,
            transform,
        }) = self.saved_states.pop()
        {
            self.config = config;
            self.path_rect = path_rect;
            // FIXME
            /*for _ in former_clips_count..self.clips_count {
                self.canvas.pop_clip();
            }*/
            self.canvas.reset_clip();
            self.canvas.set_transform(transform);
            self.clips_count = former_clips_count;
        }
    }

    pub fn clear(&mut self, brush: &Brush) {
        let paint = Self::paint_from_brush(
            brush,
            Rectangle::new(
                Point::new(0., 0.),
                Size::new(
                    self.canvas.pixmap.width() as f64,
                    self.canvas.pixmap.height() as f64,
                ),
            ),
            1.0,
        );
        self.canvas.fill_rect(
            tiny_skia::Rect::from_xywh(
                0.,
                0.,
                self.canvas.pixmap.width() as f32,
                self.canvas.pixmap.height() as f32,
            )
            .unwrap(),
            &paint,
        );
    }

    pub fn data(&self) -> &[u32] {
        unsafe {
            slice::from_raw_parts(
                self.canvas.pixmap.data().as_ptr() as *const u32,
                self.canvas.pixmap.data().len() / 4,
            )
        }
    }

    pub fn start(&mut self) {
        self.canvas.pixmap.fill(tiny_skia::Color::from_rgba8(
            self.background.b(),
            self.background.g(),
            self.background.r(),
            self.background.a(),
        ));
    }

    pub fn finish(&mut self) {}
}
