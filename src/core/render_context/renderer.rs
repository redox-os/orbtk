use orbgl::{Canvas, Color};

use crate::core::{Brush, PathSegment, Shape};

pub trait Renderer {
    fn render_shape(&mut self, shape: &mut Shape) {
        self.render_path(shape.path());
    }

    fn render_path(&mut self, path: &mut [PathSegment]);
}

impl Renderer for Canvas {
    fn render_path(&mut self, path: &mut [PathSegment]) {
        for instruction in path {
            match instruction {
                PathSegment::Arc {
                    x,
                    y,
                    radius,
                    start_angle,
                    end_engle,
                } => self.arc(*x, *y, *radius, *start_angle, *end_engle),
                PathSegment::ArcTo {
                    x1,
                    y1,
                    x2,
                    y2,
                    radius,
                } => println!("Renderer: 'ArcTo' is not implemented."),
                PathSegment::BeginPath() => self.begin_path(),
                PathSegment::BezierCurveTo {
                    cp1x,
                    cp1y,
                    cp2x,
                    cp2y,
                    x,
                    y,
                } => self.bezier_curve_to(*cp1x, *cp1y, *cp2x, *cp2y, *x, *y),
                PathSegment::ClearRect {
                    x,
                    y,
                    width,
                    height,
                } => self.clear_rect(*x, *y, *width, *height),
                PathSegment::ClosePath() => self.close_path(),
                PathSegment::DrawImage { image, x, y } => {
                    self.draw_image(image, *x, *y);
                }
                PathSegment::DrawImageWithSize {
                    image,
                    x,
                    y,
                    width,
                    height,
                } => self.draw_image_with_size(image, *x, *y, *width, *height),
                PathSegment::DrawImageWithClipAndSize {
                    image,
                    clip_x,
                    clip_y,
                    clip_width,
                    clip_height,
                    x,
                    y,
                    width,
                    height,
                } => self.draw_image_with_clip_and_size(
                    image,
                    *clip_x,
                    *clip_y,
                    *clip_width,
                    *clip_height,
                    *x,
                    *y,
                    *width,
                    *height,
                ),
                PathSegment::Fill() => self.fill(),
                PathSegment::FillRect {
                    x,
                    y,
                    width,
                    height,
                } => self.fill_rect(*x, *y, *width, *height),
                PathSegment::FillText {
                    text,
                    x,
                    y,
                    max_width,
                } => println!("Renderer: 'FillText' is not implemented."),
                PathSegment::LineTo { x, y } => self.line_to(*x, *y),
                PathSegment::MoveTo { x, y } => self.move_to(*x, *y),
                PathSegment::Restore() => self.restore(),
                PathSegment::Rotate { angle } => self.rotate(*angle),
                PathSegment::SetFillStyleBrush { brush } => match brush {
                    Brush::SolidColor(color) => self.set_fill_style(get_color(color)),
                    Brush::Gradient(gradient) => {}
                },
                PathSegment::SetStrokeStyleColor { color } => {
                    println!("Renderer: 'SetStrokeStyleColor' is not implemented.")
                }
                PathSegment::SetFont { font: _ } => {
                    println!("Renderer: 'SetFont' is not implemented.")
                }
                PathSegment::SetLineWidth { width } => {
                    println!("Renderer: 'SetLineWidth' is not implemented.")
                }
                PathSegment::SetShadowBlur { blur } => {
                    println!("Renderer: 'SetShadowBlur' is not implemented.")
                }
                PathSegment::SetShadowColor { color } => {
                    println!("Renderer: 'SetShadowColor' is not implemented.")
                }
                PathSegment::SetShadowOffsetX { x } => {
                    println!("Renderer: 'SetShadowOffsetX' is not implemented.")
                }
                PathSegment::SetShadowOffsetY { y } => {
                    println!("Renderer: 'SetShadowOffsetY' is not implemented.")
                }
                PathSegment::Save() => self.save(),
                PathSegment::Scale { x, y } => self.scale(*x, *y),
                PathSegment::Stroke() => self.stroke(),
                PathSegment::Transform { a, b, c, d, e, f } => {
                    self.transform(*a, *b, *c, *d, *e, *f)
                }
                PathSegment::Translate { x, y } => self.translate(*x, *y),
                _ => {}
            }
        }
    }
}

fn get_color(hex: &str) -> Color {
    let clean_hex = hex.trim_start_matches("#");
    match clean_hex.len() {
        6 | 8 => {
            let mut x = match u32::from_str_radix(&clean_hex, 16) {
                Ok(x) => x,
                Err(_) => 0,
            };

            if clean_hex.len() == 6 {
                x |= 0xFF_000_000;
            }

            Color { data: x }
        }
        _ => Color { data: 0 },
    }
}