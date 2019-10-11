use crate::{prelude::*, render::RenderContext, utils::*};

pub struct RectangleRenderObject;

impl RectangleRenderObject {
    // Renders rectangle with border and without radius.
    fn render_bordered_rect_path(
        &self,
        render_context_2_d: &mut RenderContext,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        brush: Brush,
        border_brush: Brush,
        _border_thickness: Thickness,
    ) {
        render_context_2_d.rect(x, y, width, height);

        if !brush.is_transparent() {
            render_context_2_d.set_fill_style(brush);
            render_context_2_d.fill();
        }

        if !border_brush.is_transparent() {
            render_context_2_d.set_stroke_style(border_brush);
            render_context_2_d.stroke();
        }
    }

    // Builds rectangle path with radius and without border.
    fn render_rounded_rect_path(
        &self,
        render_context_2_d: &mut RenderContext,
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

    // Renders rectangle with border and radius.
    fn render_rounded_bordered_rect_path(
        &self,
        render_context_2_d: &mut RenderContext,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        radius: f64,
        brush: Brush,
        border_brush: Brush,
        _border_thickness: Thickness,
    ) {
        self.render_rounded_rect_path(render_context_2_d, x, y, width, height, radius);

        if !brush.is_transparent() {
            render_context_2_d.set_fill_style(brush);
            render_context_2_d.fill();
        }

        if !border_brush.is_transparent() {
            render_context_2_d.set_stroke_style(border_brush);
            render_context_2_d.stroke();
        }
    }
}

impl Into<Box<dyn RenderObject>> for RectangleRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for RectangleRenderObject {
    fn render_self(&self, context: &mut Context<'_>, global_position: &Point) {
        let (bounds, background, border_radius, border_thickness, border_brush) = {
            let widget = context.widget();
            (
                widget.clone::<Bounds>(),
                widget.get::<Background>().0.clone(),
                widget.clone_or_default::<BorderRadius>().0,
                widget.clone_or_default::<BorderThickness>().0,
                widget.clone_or_default::<BorderBrush>().0,
            )
        };

        // if context.entity.0 == 63 {
        //     println!("Yub");
        // }

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

        // let now = Instant::now();

        let has_thickness = border_thickness.left > 0.0
            || border_thickness.top > 0.0
            || border_thickness.right > 0.0
            || border_thickness.bottom > 0.0;

        context.render_context_2_d().begin_path();

        if border_radius > 0.0 {
            if has_thickness {
                self.render_rounded_bordered_rect_path(
                    context.render_context_2_d(),
                    global_position.x + bounds.x(),
                    global_position.y + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                    border_radius,
                    background,
                    border_brush,
                    border_thickness,
                );
            } else {
                self.render_rounded_rect_path(
                    context.render_context_2_d(),
                    global_position.x + bounds.x(),
                    global_position.y + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                    border_radius,
                );

                context.render_context_2_d().set_fill_style(background);
                context.render_context_2_d().fill();
            }
        } else {
            if has_thickness {
                self.render_bordered_rect_path(
                    context.render_context_2_d(),
                    global_position.x + bounds.x(),
                    global_position.y + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                    background,
                    border_brush,
                    border_thickness,
                );
            } else {
                context.render_context_2_d().rect(
                    global_position.x + bounds.x(),
                    global_position.y + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                );
                context.render_context_2_d().set_fill_style(background);
                context.render_context_2_d().fill();
            }
        }

        // println!(
        //     "Rectangle render mils: {}, id: {}",
        //     now.elapsed().as_millis(),
        //     context.entity.0
        // );
    }
}
