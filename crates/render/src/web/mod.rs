use stdweb::{
    js,
    unstable::TryInto,
    web::{document, html_element::CanvasElement, CanvasRenderingContext2d, FillRule},
};

// pub use crate::image::Image as InnerImage;
use crate::{utils::*, FontConfig, Pipeline, RenderConfig, RenderTarget, TextMetrics};

pub use self::image::*;

mod image;

/// The RenderContext2D trait, provides the rendering ctx. It is used for drawing shapes, text, images, and other objects.
pub struct RenderContext2D {
    canvas_render_context_2_d: CanvasRenderingContext2d,
    font_config: FontConfig,
    config: RenderConfig,
    saved_config: Option<RenderConfig>,
    export_data: Vec<u32>,
}

impl RenderContext2D {
    /// Creates a new render ctx with the given width and height.
    pub fn new(width: f64, height: f64) -> Self {
        let canvas: CanvasElement = document()
            .create_element("canvas")
            .unwrap()
            .try_into()
            .unwrap();

        canvas.set_width(width as u32);
        canvas.set_height(height as u32);

        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

        let export_data = vec![0; (width * height) as usize];
        ctx.set_text_baseline(stdweb::web::TextBaseline::Middle);
        RenderContext2D {
            config: RenderConfig::default(),
            saved_config: None,
            canvas_render_context_2_d: ctx,
            font_config: FontConfig::default(),
            export_data,
        }
    }

    /// Creates a new render ctx 2d.
    pub fn from_context(canvas_render_context_2_d: CanvasRenderingContext2d) -> Self {
        let export_data = vec![
            0;
            (canvas_render_context_2_d.get_canvas().width()
                * canvas_render_context_2_d.get_canvas().height())
                as usize
        ];
        canvas_render_context_2_d.set_text_baseline(stdweb::web::TextBaseline::Middle);
        RenderContext2D {
            config: RenderConfig::default(),
            saved_config: None,
            canvas_render_context_2_d,
            font_config: FontConfig::default(),
            export_data,
        }
    }

    // Rectangles

    /// Draws a filled rectangle whose starting point is at the coordinates {x, y} with the specified width and height and whose style is determined by the fillStyle attribute.
    pub fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.fill_style(&self.config.fill_style);
        self.canvas_render_context_2_d
            .fill_rect(x, y, width, height);
    }

    /// Draws a rectangle that is stroked (outlined) according to the current strokeStyle and other ctx settings.
    pub fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.stroke_style(&self.config.fill_style);
        self.canvas_render_context_2_d
            .stroke_rect(x, y, width, height);
    }

    // Text

    /// Draws (fills) a given text at the given (x, y) position.
    pub fn fill_text(&mut self, text: &str, x: f64, y: f64) {
        self.fill_style(&self.config.fill_style);
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
        self.fill_style(&self.config.fill_style);
        self.canvas_render_context_2_d.fill(FillRule::default());
    }

    /// Strokes {outlines} the current or given path with the current stroke style.
    pub fn stroke(&mut self) {
        self.stroke_style(&self.config.stroke_style);
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

    /// Draws a render target.
    pub fn draw_render_target(&mut self, render_target: &RenderTarget, x: f64, y: f64) {
        // todo
    }

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

    /// Draws the given part of the image.
    pub fn draw_image_with_clip(&mut self, image: &Image, clip: Rectangle, x: f64, y: f64) {
        js!(
            var img = document.image_store.image(@{&image.source});

            if(img == null) {
                img = document.image_store.load_image(@{&image.source});
                img.then(
                    function(i) {
                         @{&self.canvas_render_context_2_d}.drawImage(img, @{&clip.x}, @{&clip.y}, @{&clip.width}, @{&clip.height}, @{&x}, @{&y}, @{&clip.width}, @{&clip.height});
                    }
                )
            } else {
                 @{&self.canvas_render_context_2_d}.drawImage(img, @{&x}, @{&y});
            }
        );
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

        let image_data = self
            .canvas_render_context_2_d
            .create_image_data(width, height)
            .unwrap();

        for i in 0..(render_target.data.len() - 1) {
            let pixel = render_target.data.get(i).unwrap();
            let r = ((pixel & 0x00FF0000) >> 16) as u8;
            let g = ((pixel & 0x0000FF00) >> 8) as u8;
            let b = (pixel & 0x000000FF) as u8;
            let a = ((pixel & 0xFF000000) >> 24) as u8;

            let index = i as u32 * 4;
            js!(
                @{&image_data}.data[@{index} + 0] = @{r};  // R value
                @{&image_data}.data[@{index} + 1] = @{g};    // G value
                @{&image_data}.data[@{index} + 2] = @{b};  // B value
                @{&image_data}.data[@{index} + 3] = @{a};  // A value
            );
        }

        let canvas: CanvasElement = document()
            .create_element("canvas")
            .unwrap()
            .try_into()
            .unwrap();

        canvas.set_width(width as u32);
        canvas.set_height(height as u32);

        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();
        ctx.put_image_data(image_data, 0.0, 0.0)
            .expect("Could no draw pipeline.");

        // todo: use await after stdweb futures are stable
        js!(
            // use the tempCanvas.toDataURL to create an img object
            var img = new Image();

            img.onload = function () {
                @{&self.canvas_render_context_2_d}.drawImage(img,@{&x},@{&y});
            };

            img.src = @{&canvas}.toDataURL();
        );
    }

    /// Creates a clipping path from the current sub-paths. Everything drawn after clip() is called appears inside the clipping path only.
    pub fn clip(&mut self) {
        self.canvas_render_context_2_d.clip(FillRule::EvenOdd);
    }

    // Line styles

    /// Sets the thickness of lines.
    pub fn set_line_width(&mut self, line_width: f64) {
        self.config.line_width = line_width;
        self.canvas_render_context_2_d.set_line_width(line_width);
    }

    /// Sets the alpha value,
    pub fn set_alpha(&mut self, alpha: f32) {
        self.canvas_render_context_2_d
            .set_global_alpha(alpha as f64);
    }

    /// Specific the font family.
    pub fn set_font_family(&mut self, family: impl Into<String>) {
        self.font_config.family = family.into();
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
    pub fn set_fill_style(&mut self, fill_style: Brush) {
        self.config.fill_style = fill_style;
    }

    /// Specifies the fill stroke to use inside shapes.
    pub fn set_stroke_style(&mut self, stroke_style: Brush) {
        self.config.stroke_style = stroke_style;
    }

    // Transformations

    /// Sets the tranformation.
    pub fn set_transform(
        &mut self,
        h_scaling: f64,
        h_skewing: f64,
        v_skewing: f64,
        v_scaling: f64,
        h_moving: f64,
        v_moving: f64,
    ) {
        self.canvas_render_context_2_d.set_transform(
            h_scaling, h_skewing, v_skewing, v_scaling, h_moving, v_moving,
        );
    }

    // Canvas states

    /// Saves the entire state of the canvas by pushing the current state onto a stack.
    pub fn save(&mut self) {
        self.saved_config = Some(self.config.clone());
        self.canvas_render_context_2_d.save();
    }

    /// Restores the most recently saved canvas state by popping the top entry in the drawing state stack. If there is no saved state, this method does nothing.
    pub fn restore(&mut self) {
        self.canvas_render_context_2_d.restore();
        if let Some(config) = &self.saved_config {
            self.config = config.clone();
        }

        self.saved_config = None;
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

    pub fn data(&mut self) -> &[u32] {
        let width = self.canvas_render_context_2_d.get_canvas().width();
        let height = self.canvas_render_context_2_d.get_canvas().height();

        // self.canvas_render_context_2_d.set_fill_style_color("#000000");
        // self.canvas_render_context_2_d.fill_rect(0.0, 0.0, 10.0, height as f64 / 8.0);

        let image_data = self
            .canvas_render_context_2_d
            .get_image_data(0.0, 0.0, width as f64, height as f64)
            .unwrap();

        js!(
            console.log(@{&image_data});
        );

        for i in 0..(self.export_data.len() - 1) {
            let index = i as u32 * 4;
            let r: u8 = js!(
                return @{&image_data}.data[@{index}];
            )
            .try_into()
            .unwrap();

            let g: u8 = js!(
                return @{&image_data}.data[@{index} + 1];
            )
            .try_into()
            .unwrap();

            let b: u8 = js!(
                return @{&image_data}.data[@{index} + 2];
            )
            .try_into()
            .unwrap();

            let a: u8 = js!(
                return @{&image_data}.data[@{index} + 3];
            )
            .try_into()
            .unwrap();

            js!(
                if(@{&g} != 0) {
                    console.log(@{&g});
                }

            );

            self.export_data[i] =
                ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        }

        &self.export_data
    }

    pub fn start(&mut self) {}
    pub fn finish(&mut self) {}

    fn fill_style<'a>(&self, brush: &Brush) {
        match brush {
            Brush::SolidColor(color) => {
                self.canvas_render_context_2_d
                    .set_fill_style_color(&color.to_string());
            }
            Brush::LinearGradient { start, end, stops } => {
                let web_gradient = self
                    .canvas_render_context_2_d
                    .create_linear_gradient(start.x, start.y, end.x, end.y);

                for stop in stops {
                    web_gradient
                        .add_color_stop(stop.position, stop.color.to_string().as_str())
                        .unwrap();
                }

                self.canvas_render_context_2_d
                    .set_fill_style_gradient(&web_gradient);
            }
        }
    }

    fn stroke_style<'a>(&self, brush: &Brush) {
        match brush {
            Brush::SolidColor(color) => {
                self.canvas_render_context_2_d
                    .set_stroke_style_color(&color.to_string());
            }
            Brush::LinearGradient { start, end, stops } => {
                let web_gradient = self
                    .canvas_render_context_2_d
                    .create_linear_gradient(start.x, start.y, end.x, end.y);

                for stop in stops {
                    web_gradient
                        .add_color_stop(stop.position, stop.color.to_string().as_str())
                        .unwrap();
                }

                self.canvas_render_context_2_d
                    .set_stroke_style_gradient(&web_gradient);
            }
        }
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
