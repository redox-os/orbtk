use stdweb::{
    _js_impl, js,
    unstable::TryInto,
    web::{CanvasRenderingContext2d, FillRule, TextAlign},
};

use crate::{utils::*, FontConfig, TextMetrics};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Image {
    source: String,
}

impl Image {
    /// Constructs a new image with the given source.
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();

        // Register image store if not registered.
        js!(
            if(!document.hasOwnProperty("image_store")) {
                document.image_store = {
                    images: {}
                };

                document.image_store.load_image = function (src) {
                    var img = new Image();

                    var d = new Promise(function (resolve, reject) {
                        img.onload = function () {
                            this.images[src] = img;
                            resolve(img);
                        }.bind(this);

                        img.onerror = function () {
                            reject("Could not load image: " + src);
                        };
                    }.bind(this));

                    img.src = src;
                    return d;
                };
            }
        );

        js!(
            document.image_store.image = function (src) {
                return (src in this.images) ? this.images[src] : null;
            };
        );

        // load the image
        js!(
            document.image_store.load_image(@{&source});
        );

        Image { source }
    }

    /// Gets the width.
    pub fn width(&self) -> f64 {
        let width: u64 = js!(
            var image = document.image_store.image(@{&self.source});

            if(image == null) {
                return 0;
            }

            return image.width;
        )
        .try_into()
        .unwrap();
        width as f64
    }

    /// Gets the height.
    pub fn height(&self) -> f64 {
        let height: u64 = js!(
            var image = document.image_store.image(@{&self.source});

            if(image == null) {
                return 0;
            }

            return image.height;
        )
        .try_into()
        .unwrap();

        height as f64
    }
}

/// The RenderContext2D trait, provides the 2D rendering context. It is used for drawing shapes, text, images, and other objects.
pub struct RenderContext2D {
    canvas_render_context_2_d: CanvasRenderingContext2d,
    font_config: FontConfig,
}

impl RenderContext2D {
    /// Creates a new render context 2d.
    pub fn new(canvas_render_context_2_d: CanvasRenderingContext2d) -> Self {
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
    pub fn fill_text(&mut self, text: &str, x: f64, y: f64, max_width: Option<f64>) {
        self.canvas_render_context_2_d
            .fill_text(text, x, y, max_width);
    }

    /// Draws (strokes) a given text at the given (x, y) position.
    pub fn stroke_text(&mut self, text: &str, x: f64, y: f64, max_width: Option<f64>) {
        self.canvas_render_context_2_d
            .stroke_text(text, x, y, max_width);
    }

    /// Returns a TextMetrics object.
    pub fn measure_text(&mut self, text: &str) -> TextMetrics {
        TextMetrics {
            width: self
                .canvas_render_context_2_d
                .measure_text(text)
                .unwrap()
                .get_width(),
            height: self.font_config.font_size,
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
    pub fn arc(
        &mut self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) {
        self.canvas_render_context_2_d
            .arc(x, y, radius, start_angle, end_angle, anticlockwise);
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
    pub fn draw_image(&mut self, image: &mut Image, x: f64, y: f64) {
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
    pub fn draw_image_with_size(
        &mut self,
        image: &mut Image,
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
        js!(
            var img = document.image_store.image(@{&image.source});

            if(img == null) {
                img = document.image_store.load_image(@{&image.source});
                img.then(
                    function(i) {
                         @{&self.canvas_render_context_2_d}.drawImage(i,, @{&clip_x}, @{&clip_y}, @{&clip_width}, @{&clip_height}, @{&x}, @{&y}, @{&width}, @{&height});
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

    /// Specifies the text alignment.
    pub fn set_text_align(&mut self, alignment: TextAlignment) {
        let text_alignment = match alignment {
            TextAlignment::Left => TextAlign::Left,
            TextAlignment::Right => TextAlign::Right,
            TextAlignment::Center => TextAlign::Center,
            TextAlignment::Start => TextAlign::Start,
            TextAlignment::End => TextAlign::End,
        };
        self.canvas_render_context_2_d
            .set_text_align(text_alignment);
    }

    /// Baseline alignment setting.
    pub fn set_text_baseline(&mut self, text_baseline: TextBaseline) {
        let text_baseline = match text_baseline {
            TextBaseline::Top => stdweb::web::TextBaseline::Top,
            TextBaseline::Hanging => stdweb::web::TextBaseline::Hanging,
            TextBaseline::Middle => stdweb::web::TextBaseline::Middle,
            TextBaseline::Alphabetic => stdweb::web::TextBaseline::Alphabetic,
            TextBaseline::Ideographic => stdweb::web::TextBaseline::Ideographic,
            TextBaseline::Bottom => stdweb::web::TextBaseline::Bottom,
        };

        self.canvas_render_context_2_d
            .set_text_baseline(text_baseline);
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

    // Shadows

    pub fn set_shadow_color(&mut self, color: Color) {
        self.canvas_render_context_2_d
            .set_shadow_color(&color.to_string());
    }

    pub fn set_shadow_offset(&mut self, x: f64, y: f64) {
        self.canvas_render_context_2_d.set_shadow_offset_x(x);
        self.canvas_render_context_2_d.set_shadow_offset_y(y);
    }

    // Transformations

    /// Multiplies the current transformation with the matrix described by the arguments of this method. You are able to scale, rotate, move and skew the context.
    pub fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.canvas_render_context_2_d.transform(a, b, c, d, e, f);
    }

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

    pub fn set_canvas_render_context_2d(
        &mut self,
        canvas_render_context_2_d: CanvasRenderingContext2d,
    ) {
        self.canvas_render_context_2_d = canvas_render_context_2_d;
    }
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
