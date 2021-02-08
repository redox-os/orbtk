use std::f64::consts::PI;

use crate::{
    proc_macros::IntoRenderObject,
    render::RenderContext2D,
    render_object::*,
    utils,
    utils::{Brush, Point, Rectangle, Thickness},
};

#[derive(Debug, IntoRenderObject)]
pub struct RectangleRenderObject;

impl RectangleRenderObject {
    // Renders rectangle with border and without radius.
    fn render_bordered_rect_path(
        &self,
        rtx: &mut RenderContext2D,
        rect: Rectangle,
        brush: utils::Brush,
        border_brush: utils::Brush,
        border_thickness: Thickness,
    ) {
        rtx.rect(rect.x(), rect.y(), rect.width(), rect.height());

        if !brush.is_transparent() {
            rtx.set_fill_style(brush);
            rtx.fill();
        }

        if !border_brush.is_transparent() {
            rtx.set_line_width(border_thickness.left());
            rtx.set_stroke_style(border_brush);
            rtx.stroke();
        }
    }

    // Builds rectangle path with radius and without border.
    fn render_rounded_rect_path(
        &self,
        rtx: &mut RenderContext2D,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        radius: f64,
    ) {
        let r = x + width;
        let b = y + height;
        rtx.move_to(x + radius, y);
        rtx.line_to(r - radius, y);
        rtx.quadratic_curve_to(r, y, r, y + radius);
        rtx.line_to(r, y + height - radius);
        rtx.quadratic_curve_to(r, b, r - radius, b);
        rtx.line_to(x + radius, b);
        rtx.quadratic_curve_to(x, b, x, b - radius);
        rtx.line_to(x, y + radius);
        rtx.quadratic_curve_to(x, y, x + radius, y);
        rtx.close_path();
    }

    fn render_circle(
        &self,
        rtx: &mut RenderContext2D,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        radius: f64,
    ) {
        rtx.arc(x + width / 2.0, y + height / 2.0, radius, 0., 2. * PI);
        rtx.close_path();
    }

    fn render_bordered_circle(
        &self,
        rtx: &mut RenderContext2D,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        radius: f64,
        brush: utils::Brush,
        border_brush: utils::Brush,
        border_thickness: Thickness,
    ) {
        self.render_circle(rtx, x, y, width, height, radius);

        if !brush.is_transparent() {
            rtx.set_fill_style(brush);
            rtx.fill();
        }

        if !border_brush.is_transparent() {
            rtx.set_line_width(border_thickness.left());
            rtx.set_stroke_style(border_brush);
            rtx.stroke();
        }
    }

    // Renders rectangle with border and radius.
    fn render_rounded_bordered_rect_path(
        &self,
        rtx: &mut RenderContext2D,
        rect: Rectangle,
        radius: f64,
        brush: utils::Brush,
        border_brush: utils::Brush,
        border_thickness: Thickness,
    ) {
        self.render_rounded_rect_path(rtx, rect.x(), rect.y(), rect.width(), rect.height(), radius);

        if !brush.is_transparent() {
            rtx.set_fill_style(brush);
            rtx.fill();
        }

        if !border_brush.is_transparent() {
            rtx.set_line_width(border_thickness.left());
            rtx.set_stroke_style(border_brush);
            rtx.stroke();
        }
    }
}

impl RenderObject for RectangleRenderObject {
    fn render_self(&self, ctx: &mut Context, global_position: &Point, rtx: &mut RenderContext2D) {
        let (bounds, background, border_radius, border_thickness, border_brush) = {
            let widget = ctx.widget();
            (
                widget.clone::<Rectangle>("bounds"),
                widget.get::<Brush>("background").clone(),
                widget.clone_or_default::<f64>("border_radius"),
                widget.clone_or_default::<Thickness>("border_width"),
                widget.clone_or_default::<Brush>("border_brush"),
            )
        };

        if (bounds.width() == 0.0
            || bounds.height() == 0.0
            || (background.is_transparent() && border_brush.is_transparent()))
            && (border_thickness.left == 0.0
                && border_thickness.top == 0.0
                && border_thickness.right == 0.0
                && border_thickness.bottom == 0.0)
        {
            return;
        }

        let has_thickness = border_thickness.left > 0.0
            || border_thickness.top > 0.0
            || border_thickness.right > 0.0
            || border_thickness.bottom > 0.0;

        rtx.begin_path();

        if (bounds.width() - bounds.height()).abs() < f64::EPSILON
            && border_radius >= bounds.width() / 2.0
        {
            if !has_thickness {
                self.render_circle(
                    rtx,
                    global_position.x() + bounds.x(),
                    global_position.y() + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                    border_radius,
                );
                rtx.set_fill_style(background);
                rtx.fill();
            } else {
                self.render_bordered_circle(
                    rtx,
                    global_position.x() + bounds.x(),
                    global_position.y() + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                    border_radius,
                    background,
                    border_brush,
                    border_thickness,
                );
            }
        } else if border_radius > 0. && has_thickness {
            self.render_rounded_bordered_rect_path(
                rtx,
                Rectangle::new(*global_position + bounds.position(), bounds.size()),
                border_radius,
                background,
                border_brush,
                border_thickness,
            );
        } else if border_radius > 0. {
            self.render_rounded_rect_path(
                rtx,
                global_position.x() + bounds.x(),
                global_position.y() + bounds.y(),
                bounds.width(),
                bounds.height(),
                border_radius,
            );

            rtx.set_fill_style(background);
            rtx.fill();
        } else if has_thickness {
            self.render_bordered_rect_path(
                rtx,
                Rectangle::new(*global_position + bounds.position(), bounds.size()),
                background,
                border_brush,
                border_thickness,
            );
        } else {
            rtx.set_fill_style(background);
            rtx.fill_rect(
                global_position.x() + bounds.x(),
                global_position.y() + bounds.y(),
                bounds.width(),
                bounds.height(),
            );
        }
    }
}
