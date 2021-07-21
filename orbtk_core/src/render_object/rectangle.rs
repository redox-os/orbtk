use std::f64::consts::PI;

use crate::{
    proc_macros::IntoRenderObject,
    render::RenderContext2D,
    render_object::*,
    utils,
    utils::{Brush, Point, Rectangle, Thickness},
};

/// Structure that defines a rectangle for a render object.
#[derive(Debug, IntoRenderObject)]
pub struct RectangleRenderObject;

impl RectangleRenderObject {
    // Renders rectangle with border and without radius.
    fn render_bordered_rect_path(
        &self,
        render_context_2_d: &mut RenderContext2D,
        rect: Rectangle,
        brush: utils::Brush,
        border_brush: utils::Brush,
        border_thickness: Thickness,
    ) {
        render_context_2_d.rect(rect.x(), rect.y(), rect.width(), rect.height());

        if !brush.is_transparent() {
            render_context_2_d.set_fill_style(brush);
            render_context_2_d.fill();
        }

        if !border_brush.is_transparent() {
            render_context_2_d.set_line_width(border_thickness.left());
            render_context_2_d.set_stroke_style(border_brush);
            render_context_2_d.stroke();
        }
    }

    // Builds rectangle path with radius and without border.
    fn render_rounded_rect_path(
        &self,
        render_context_2_d: &mut RenderContext2D,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        radius: f64,
    ) {
        let r = x + width;
        let b = y + height;
        render_context_2_d.move_to(x + radius, y);
        render_context_2_d.line_to(r - radius, y);
        render_context_2_d.quadratic_curve_to(r, y, r, y + radius);
        render_context_2_d.line_to(r, y + height - radius);
        render_context_2_d.quadratic_curve_to(r, b, r - radius, b);
        render_context_2_d.line_to(x + radius, b);
        render_context_2_d.quadratic_curve_to(x, b, x, b - radius);
        render_context_2_d.line_to(x, y + radius);
        render_context_2_d.quadratic_curve_to(x, y, x + radius, y);
        render_context_2_d.close_path();
    }

    fn render_circle(
        &self,
        render_context_2_d: &mut RenderContext2D,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        radius: f64,
    ) {
        render_context_2_d.arc(x + width / 2.0, y + height / 2.0, radius, 0., 2. * PI);
        render_context_2_d.close_path();
    }

    fn render_bordered_circle(
        &self,
        render_context_2_d: &mut RenderContext2D,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        radius: f64,
        brush: utils::Brush,
        border_brush: utils::Brush,
        border_thickness: Thickness,
    ) {
        self.render_circle(render_context_2_d, x, y, width, height, radius);

        if !brush.is_transparent() {
            render_context_2_d.set_fill_style(brush);
            render_context_2_d.fill();
        }

        if !border_brush.is_transparent() {
            render_context_2_d.set_line_width(border_thickness.left());
            render_context_2_d.set_stroke_style(border_brush);
            render_context_2_d.stroke();
        }
    }

    // Renders rectangle with border and radius.
    fn render_rounded_bordered_rect_path(
        &self,
        render_context_2_d: &mut RenderContext2D,
        rect: Rectangle,
        radius: f64,
        brush: utils::Brush,
        border_brush: utils::Brush,
        border_thickness: Thickness,
    ) {
        self.render_rounded_rect_path(
            render_context_2_d,
            rect.x(),
            rect.y(),
            rect.width(),
            rect.height(),
            radius,
        );

        if !brush.is_transparent() {
            render_context_2_d.set_fill_style(brush);
            render_context_2_d.fill();
        }

        if !border_brush.is_transparent() {
            render_context_2_d.set_line_width(border_thickness.left());
            render_context_2_d.set_stroke_style(border_brush);
            render_context_2_d.stroke();
        }
    }
}

impl RenderObject for RectangleRenderObject {
    fn render_self(&self, ctx: &mut Context, global_position: &Point) {
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

        ctx.render_context_2_d().begin_path();

        if (bounds.width() - bounds.height()).abs() < f64::EPSILON
            && border_radius >= bounds.width() / 2.0
        {
            if !has_thickness {
                self.render_circle(
                    ctx.render_context_2_d(),
                    global_position.x() + bounds.x(),
                    global_position.y() + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                    border_radius,
                );
                ctx.render_context_2_d().set_fill_style(background);
                ctx.render_context_2_d().fill();
            } else {
                self.render_bordered_circle(
                    ctx.render_context_2_d(),
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
                ctx.render_context_2_d(),
                Rectangle::new(*global_position + bounds.position(), bounds.size()),
                border_radius,
                background,
                border_brush,
                border_thickness,
            );
        } else if border_radius > 0. {
            self.render_rounded_rect_path(
                ctx.render_context_2_d(),
                global_position.x() + bounds.x(),
                global_position.y() + bounds.y(),
                bounds.width(),
                bounds.height(),
                border_radius,
            );

            ctx.render_context_2_d().set_fill_style(background);
            ctx.render_context_2_d().fill();
        } else if has_thickness {
            self.render_bordered_rect_path(
                ctx.render_context_2_d(),
                Rectangle::new(*global_position + bounds.position(), bounds.size()),
                background,
                border_brush,
                border_thickness,
            );
        } else {
            ctx.render_context_2_d().set_fill_style(background);
            ctx.render_context_2_d().fill_rect(
                global_position.x() + bounds.x(),
                global_position.y() + bounds.y(),
                bounds.width(),
                bounds.height(),
            );
        }
    }
}
