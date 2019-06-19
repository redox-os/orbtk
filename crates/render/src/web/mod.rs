use stdweb::{
    _js_impl, js,
    traits::*,
    unstable::TryInto,
    web::{
        self, document, event,
        html_element::{CanvasElement, ImageElement},
        window, CanvasRenderingContext2d, FillRule,
    },
};

use crate::{utils::*, TextMetrics};

// todo define platform specific image object (web version image storage on document)
pub struct Image {

}

pub struct RenderContext2D {
    canvas_render_context_2D: CanvasRenderingContext2d
}

impl RenderContext2D {
    pub fn new(canvas_render_context_2D: CanvasRenderingContext2d) -> Self {
        // handle image store
        js!(
            @{&canvas_render_context_2D}.images = {};
        );

        RenderContext2D {
            canvas_render_context_2D
        }
    }
    
    pub fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        self.canvas_render_context_2D.fill_rect(x, y, width, height);
    }

    pub fn stroke_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        self.canvas_render_context_2D.stroke_rect(x, y, width, height);
    }

    pub fn fill_text(&self, text: &str, x: f64, y: f64, max_width: Option<f64>) {
        self.canvas_render_context_2D.fill_text(text, x, y, max_width);
    }

    pub fn stroke_text(&self, text: &str, x: f64, y: f64, max_width: Option<f64>) {
        self.canvas_render_context_2D.stroke_text(text, x, y, max_width);
    }

    pub fn measure_text(&self, text: &str) -> TextMetrics {
        TextMetrics {
            width: self.canvas_render_context_2D.measure_text(text).unwrap().get_width()
        }
    }

    pub fn fill(&self) {
        self.canvas_render_context_2D.fill(FillRule::default());
    }

    pub fn stroke(&self) {
        self.canvas_render_context_2D.stroke();
    }

    pub fn begin_path(&self) {
        self.canvas_render_context_2D.begin_path();
    }

    pub fn close_path(&self) {
        self.canvas_render_context_2D.close_path();
    }

    pub fn rect(&self, x: f64, y: f64, width: f64, height: f64) {
        self.canvas_render_context_2D.rect(x, y, width, height);
    }

    pub fn arc(&self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64, anticlockwise: bool) {
        self.canvas_render_context_2D.arc(x, y, radius, start_angle, end_angle, anticlockwise);
    }

    pub fn move_to(&self, x: f64, y: f64) {
        self.canvas_render_context_2D.move_to(x, y);
    }

    pub fn line_to(&self, x: f64, y: f64) {
        self.canvas_render_context_2D.line_to(x, y);
    }

    pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.canvas_render_context_2D.quadratic_curve_to(cpx, cpy, x, y);
    }

    pub fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.canvas_render_context_2D.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    pub fn draw_image(&self, src: &str, dx: f64, dy: f64) {
        let src = src.to_string();
        js!(
            if(@{&src} in @{&self.canvas_render_context_2D}.images) {
                @{&self.canvas_render_context_2D}.draw_image(@{&self.canvas_render_context_2D}.images[@{&src}], @{&dx}, @{&dy})
            }
        );

        js!(
           if(!(@{&src} in @{&self.canvas_render_context_2D}.images)) {
                var img = new Image();
                img.src = @{&src};
               
                img.onload = function() {
                    @{&self.canvas_render_context_2D}.images[@{&src}] = img;
                    @{&self.canvas_render_context_2D}.drawImage(img, @{&dx}, @{&dy});
                };
            }
        );
    }

    pub fn draw_image_with_size(&self, src: &str, x: f64, y: f64, width: f64, height: f64) {
        let src = src.to_string();
        js!(
            if(@{&src} in @{&self.canvas_render_context_2D}.images) {
                @{&self.canvas_render_context_2D}.draw_image(@{&self.canvas_render_context_2D}.images[@{&src}], @{x}, @{&y})
            }
        );

        js!(
           if(!(@{&src} in @{&self.canvas_render_context_2D}.images)) {
                var img = new Image();
                img.src = @{&src};             
                img.onload = function() {
                    @{&self.canvas_render_context_2D}.images[@{&src}] = img;
                    @{&self.canvas_render_context_2D}.drawImage(img, @{&x}, @{&y}, @{&height}, @{&height});
                };
            }
        );
    }

    pub fn draw_image_with_clip_and_size(&self, path: &str, x: f64, y: f64, width: f64, height: f64) {
        unimplemented!()
    }

    pub fn set_line_width(&self, clip_x: f64, clip_y: f64, clip_width: f64, clip_height: f64, x: f64, y: f64, width: f64, height: f64) {
        self.set_line_width(clip_x, clip_y, clip_width, clip_height, x, y, width, height);
    }

    pub fn set_font_family(&self, family: &str) {
        unimplemented!()
    }

    pub fn set_font_size(&self, size: f64) {
        unimplemented!()
    }

    pub fn set_text_align(&self, alignment: TextAlignment) {
        unimplemented!()
    }

    pub fn set_fill_style(&self, brush: Brush) {
        match brush {
            Brush::SolidColor(color) => {
                self.canvas_render_context_2D.set_fill_style_color(&color.to_string());
            },
            _ => ()
        }
    }

    pub fn set_stroke_style(&self, brush: Brush) {
        match brush {
            Brush::SolidColor(color) => {
                self.canvas_render_context_2D.set_stroke_style_color(&color.to_string());
            },
            _ => ()
        }
    }

    pub fn set_shadow_color(&self, color: Color) {
        self.canvas_render_context_2D.set_shadow_color(&color.to_string());
    }

    pub fn set_shadow_offset(&self, x: f64, y: f64) {
        self.canvas_render_context_2D.set_shadow_offset_x(x);
        self.canvas_render_context_2D.set_shadow_offset_y(y);
    }

    pub fn transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.canvas_render_context_2D.transform(a, b, c, d, e, f);
    }

    pub fn set_transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.canvas_render_context_2D.set_transform(a, b, c, d, e, f);
    }

    pub fn save(&self) {
        self.canvas_render_context_2D.save();
    }

    pub fn restore(&self) {
        self.canvas_render_context_2D.restore();
    }
}