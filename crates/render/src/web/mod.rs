use stdweb::{
    _js_impl, js,
    traits::*,
    unstable::TryInto,
    web::{
        self, document, event,
        html_element::{CanvasElement, ImageElement},
        window, CanvasRenderingContext2d, FillRule, TextAlign,
    },
};

use crate::{utils::*, TextMetrics};

// Internal font helper.
#[derive(Default, Clone, PartialEq, Debug)]
struct FontConfig {
    family: String,
    font_size: f64,
}

impl ToString for FontConfig {
    fn to_string(&self) -> String {
        format!("{}px {}", self.font_size, self.family)
    }
}

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

            return image.width();
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

            return image.height();
        )
        .try_into()
        .unwrap();

        height as f64
    }
}

pub struct RenderContext2D {
    canvas_render_context_2_d: CanvasRenderingContext2d,
    font_config: FontConfig,
}

impl RenderContext2D {
    pub fn new(canvas_render_context_2_d: CanvasRenderingContext2d) -> Self {
        RenderContext2D {
            canvas_render_context_2_d,
            font_config: FontConfig::default(),
        }
    }

    pub fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.canvas_render_context_2_d.fill_rect(x, y, width, height);
    }

    pub fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.canvas_render_context_2_d
            .stroke_rect(x, y, width, height);
    }

    pub fn fill_text(&mut self, text: &str, x: f64, y: f64, max_width: Option<f64>) {
        self.canvas_render_context_2_d
            .fill_text(text, x, y + self.font_config.font_size / 1.125, max_width);
    }

    pub fn stroke_text(&mut self, text: &str, x: f64, y: f64, max_width: Option<f64>) {
        self.canvas_render_context_2_d
            .stroke_text(text, x, y, max_width);
    }

    pub fn measure_text(&mut self, text: &str) -> TextMetrics {
        TextMetrics {
            width: self
                .canvas_render_context_2_d
                .measure_text(text)
                .unwrap()
                .get_width(),
        }
    }

    pub fn fill(&mut self) {
        self.canvas_render_context_2_d.fill(FillRule::default());
    }

    pub fn stroke(&mut self) {
        self.canvas_render_context_2_d.stroke();
    }

    pub fn begin_path(&mut self) {
        self.canvas_render_context_2_d.begin_path();
    }

    pub fn close_path(&mut self) {
        self.canvas_render_context_2_d.close_path();
    }

    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.canvas_render_context_2_d.rect(x, y, width, height);
    }

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

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.canvas_render_context_2_d.move_to(x, y);
    }

    pub fn line_to(&mut self, x: f64, y: f64) {
        self.canvas_render_context_2_d.line_to(x, y);
    }

    pub fn quadratic_curve_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.canvas_render_context_2_d
            .quadratic_curve_to(cpx, cpy, x, y);
    }

    pub fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.canvas_render_context_2_d
            .bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

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
                 @{&self.canvas_render_context_2_d}.drawImage(img, @{&x}, @{&y});
            }
        );
    }

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
                         @{&self.canvas_render_context_2_d}.drawImage(i,, @{&clip_x}, @{&clip_y}, @{&clip_width}, @{&clip_height}, @{&x}, @{&y}, @{&width}, @{&height});
                    }
                )
            } else {
                 @{&self.canvas_render_context_2_d}.drawImage(img, @{&x}, @{&y});
            }
        );
    }

    pub fn clip(&mut self) {
        self.canvas_render_context_2_d.clip(FillRule::default());
    }

    pub fn set_line_width(
        &mut self,
        clip_x: f64,
        clip_y: f64,
        clip_width: f64,
        clip_height: f64,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) {
        self.set_line_width(clip_x, clip_y, clip_width, clip_height, x, y, width, height);
    }

    pub fn set_font_family(&mut self, family: impl Into<String>) {
        self.font_config.family = family.into();
        self.canvas_render_context_2_d
            .set_font(&self.font_config.to_string());
    }

    pub fn set_font_size(&mut self, size: f64) {
        self.font_config.font_size = size;
        self.canvas_render_context_2_d
            .set_font(&self.font_config.to_string());
    }

    pub fn set_text_align(&mut self, alignment: TextAlignment) {
        let text_alignment = match alignment {
            TextAlignment::Left => TextAlign::Left,
            TextAlignment::Right => TextAlign::Right,
            TextAlignment::Center => TextAlign::Center,
            TextAlignment::Start => TextAlign::Start,
            TextAlignment::End => TextAlign::End,
        };
        self.canvas_render_context_2_d.set_text_align(text_alignment);
    }

    pub fn set_fill_style(&mut self, brush: Brush) {
        match brush {
            Brush::SolidColor(color) => {
                self.canvas_render_context_2_d
                    .set_fill_style_color(&color.to_string());
            }
            _ => (),
        }
    }

    pub fn set_stroke_style(&mut self, brush: Brush) {
        match brush {
            Brush::SolidColor(color) => {
                self.canvas_render_context_2_d
                    .set_stroke_style_color(&color.to_string());
            }
            _ => (),
        }
    }

    pub fn set_shadow_color(&mut self, color: Color) {
        self.canvas_render_context_2_d
            .set_shadow_color(&color.to_string());
    }

    pub fn set_shadow_offset(&mut self, x: f64, y: f64) {
        self.canvas_render_context_2_d.set_shadow_offset_x(x);
        self.canvas_render_context_2_d.set_shadow_offset_y(y);
    }

    pub fn transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.canvas_render_context_2_d.transform(a, b, c, d, e, f);
    }

    pub fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.canvas_render_context_2_d
            .set_transform(a, b, c, d, e, f);
    }

    pub fn save(&mut self) {
        self.canvas_render_context_2_d.save();
    }

    pub fn restore(&mut self) {
        self.canvas_render_context_2_d.restore();
    }
}
