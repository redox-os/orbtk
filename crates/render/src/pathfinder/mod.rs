use pathfinder_canvas::{CanvasFontContext, CanvasRenderingContext2D, FillStyle, Path2D};
use pathfinder_content::color::{ColorF, ColorU};
use pathfinder_content::outline::{ArcDirection, Contour, Outline};
use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::vector::{Vector2F, Vector2I};
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_gpu::resources::FilesystemResourceLoader;
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererOptions};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;

use crate::{utils::*, FontConfig, TextMetrics};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Image {
    source: String,
}

impl Image {
    /// Constructs a new image with the given source.
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();

        Image { source }
    }

    /// Gets the width.
    pub fn width(&self) -> f64 {
        0.0
    }

    /// Gets the height.
    pub fn height(&self) -> f64 {
        0.0
    }
}

/// The RenderContext2D trait, provides the 2D rendering context. It is used for drawing shapes, text, images, and other objects.
pub struct RenderContext2D {
    font_context: CanvasFontContext,
    renderer: Option<Renderer<GLDevice>>,
    canvas: Vec<CanvasRenderingContext2D>,
    scene: SceneProxy,
    path: Vec<Path2D>,
    font_config: FontConfig,
}

impl RenderContext2D {
    /// Creates a new render context 2d.
    pub fn new() -> Self {
        RenderContext2D {
            font_context: CanvasFontContext::from_system_source(),
            scene: SceneProxy::new(RayonExecutor),
            renderer: None,
            canvas: vec![],
            path: vec![],
            font_config: FontConfig::default(),
        }
    }

    pub fn refresh(&mut self, width: f64, height: f64) {
        self.begin_path();
        let window_size = Vector2I::new(width as i32, height as i32);

        if self.renderer.is_none() {
            // Create a Pathfinder renderer.
            self.renderer = Some(Renderer::new(
                GLDevice::new(GLVersion::GL3, 0),
                &FilesystemResourceLoader::locate(),
                DestFramebuffer::full_window(window_size),
                RendererOptions {
                    background_color: Some(ColorF::white()),
                },
            ));
        }

        self.canvas.push(CanvasRenderingContext2D::new(
            self.font_context.clone(),
            window_size.to_f32(),
        ));
    }

    pub fn render(&mut self) {
        let mut canvas = self.canvas.remove(0);

        if let Some(render) = &mut self.renderer {
            // Build and render scene.
            // Clear to background color.
            render.set_options(RendererOptions {
                background_color: Some(ColorF::new(0.0, 0.0, 0.0, 0.0)),
            });
            self.scene.replace_scene(canvas.into_scene());
            self.scene.build_and_render(render, BuildOptions::default());
        }
    }

    /// Registers a new font file.
    pub fn register_font(&mut self, family: &str, font_file: &[u8]) {}

    // Rectangles

    /// Draws a filled rectangle whose starting point is at the coordinates {x, y} with the specified width and height and whose style is determined by the fillStyle attribute.
    pub fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        if let Some(canvas) = &mut self.canvas.get_mut(0) {
            canvas.fill_rect(RectF::new(
                Vector2F::new(x as f32, y as f32),
                Vector2F::new(width as f32, height as f32),
            ));
        }
    }

    /// Draws a rectangle that is stroked (outlined) according to the current strokeStyle and other context settings.
    pub fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {}

    // Text

    /// Draws (fills) a given text at the given (x, y) position.
    pub fn fill_text(&mut self, text: &str, x: f64, y: f64, _: Option<f64>) {
         if let Some(canvas) = &mut self.canvas.get_mut(0) {
             canvas.fill_text(text, Vector2F::new(x as f32, y as f32));
         }
    }

    /// Draws (strokes) a given text at the given (x, y) position.
    pub fn stroke_text(&mut self, _: &str, _: f64, _: f64, _: Option<f64>) {}

    /// Returns a TextMetrics object.
    pub fn measure_text(&mut self, text: &str) -> TextMetrics {

        let mut text_metrics = TextMetrics {
            width: 0.0,
            height: 0.0,
        };

         if let Some(canvas) = &mut self.canvas.get(0) {
             text_metrics.width = canvas.measure_text(text).width as f64;
             text_metrics.height = self.font_config.font_size;
         }

        text_metrics
    }

    /// Fills the current or given path with the current file style.
    pub fn fill(&mut self) {
        if let Some(canvas) = &mut self.canvas.get_mut(0) {
            canvas.fill_path(self.path.remove(0));
        }
    }

    /// Strokes {outlines} the current or given path with the current stroke style.
    pub fn stroke(&mut self) {}

    /// Starts a new path by emptying the list of sub-paths. Call this when you want to create a new path.
    pub fn begin_path(&mut self) {
        if self.path.len() > 0 {
            self.path.remove(0);
        }

        self.path.push(Path2D::new());
    }

    /// Attempts to add a straight line from the current point to the start of the current sub-path. If the shape has already been closed or has only one point, this function does nothing.
    pub fn close_path(&mut self) {
        if let Some(path) = &mut self.path.get_mut(0) {
            path.close_path();
        }
    }

    /// Adds a rectangle to the current path.
    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        if let Some(path) = &mut self.path.get_mut(0) {
            path.rect(RectF::new(
                Vector2F::new(x as f32, y as f32),
                Vector2F::new(width as f32, y as f32),
            ));
        }
    }

    /// Creates a circular arc centered at (x, y) with a radius of radius. The path starts at startAngle and ends at endAngle.
    pub fn arc(&mut self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64, _: bool) {
        if let Some(path) = &mut self.path.get_mut(0) {
            path.arc(
                Vector2F::new(x as f32, y as f32),
                radius as f32,
                start_angle as f32,
                end_angle as f32,
                ArcDirection::CW,
            );
        }
    }

    /// Begins a new sub-path at the point specified by the given {x, y} coordinates.

    pub fn move_to(&mut self, x: f64, y: f64) {
        if let Some(path) = &mut self.path.get_mut(0) {
            path.move_to(Vector2F::new(x as f32, y as f32));
        }
    }

    /// Adds a straight line to the current sub-path by connecting the sub-path's last point to the specified {x, y} coordinates.
    pub fn line_to(&mut self, x: f64, y: f64) {
        if let Some(path) = &mut self.path.get_mut(0) {
            path.line_to(Vector2F::new(x as f32, y as f32));
        }
    }

    /// Adds a quadratic Bézier curve to the current sub-path.
    pub fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {}

    /// Adds a cubic Bézier curve to the current sub-path. It requires three points: the first two are control points and the third one is the end point. The starting point is the latest point in the current path, which can be changed using MoveTo{} before creating the Bézier curve.
    pub fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {}

    // Draw image

    /// Draws the image.
    pub fn draw_image(&mut self, image: &mut Image, x: f64, y: f64) {}

    /// Draws the image with the given size.
    pub fn draw_image_with_size(
        &mut self,
        image: &mut Image,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {

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
    pub fn set_line_width(&mut self, line_width: f64) {}

    /// Specific the font family.
    pub fn set_font_family(&mut self, family: impl Into<String>) {}

    /// Specifies the font size.
    pub fn set_font_size(&mut self, size: f64) {
        self.font_config.font_size = size;
         if let Some(canvas) = &mut self.canvas.get_mut(0) {
             canvas.set_font_size(size as f32);
         }
    }

    /// Specifies the text alignment.
    pub fn set_text_align(&mut self, _: TextAlignment) {}

    /// Baseline alignment setting.
    pub fn set_text_baseline(&mut self, _: TextBaseline) {}

    // Fill and stroke style

    /// Specifies the fill color to use inside shapes.
    pub fn set_fill_style(&mut self, brush: Brush) {
        if let Some(canvas) = &mut self.canvas.get_mut(0) {
            match brush {
                Brush::SolidColor(color) => canvas.set_fill_style(FillStyle::Color(ColorU {
                    r: color.r(),
                    g: color.g(),
                    b: color.b(),
                    a: color.a(),
                })),
                _ => (),
            }
        }
    }

    /// Specifies the fill stroke to use inside shapes.
    pub fn set_stroke_style(&mut self, brush: Brush) {
        if let Some(canvas) = &mut self.canvas.get_mut(0) {
            match brush {
                Brush::SolidColor(color) => canvas.set_stroke_style(FillStyle::Color(ColorU {
                    r: color.r(),
                    g: color.g(),
                    b: color.b(),
                    a: color.a(),
                })),
                _ => (),
            }
        }
    }

    // Shadows

    pub fn set_shadow_color(&mut self, _: Color) {}

    pub fn set_shadow_offset(&mut self, _: f64, _: f64) {}

    // Transformations

    /// Multiplies the current transformation with the matrix described by the arguments of this method. You are able to scale, rotate, move and skew the context.
    pub fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {}

    /// Sets the tranformation.
    pub fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {}

    // Canvas states

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    pub fn save(&mut self) {
        if let Some(canvas) = &mut self.canvas.get_mut(0) {
            canvas.save();
        }
    }

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. If there is no saved state, this method does nothing.
    pub fn restore(&mut self) {
        if let Some(canvas) = &mut self.canvas.get_mut(0) {
            canvas.restore();
        }
    }
}
