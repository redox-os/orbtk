use orbgl_api::Canvas;

use crate::{prelude::*, backend::Renderer};

pub struct RectangleRenderObject;

impl RectangleRenderObject {
    // Renders rectangle without border and radius.
    fn render_rect_path(
        &self,
        canvas: &mut Canvas,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        brush: Brush,
    ) {
        match brush {
            Brush::SolidColor(color) => canvas.set_fill_style(color),
            _ => {} // todo: gradient
        }

        canvas.fill_rect(x, y, width, height);
    }

    // Renders rectangle with border and without radius.
    fn render_bordered_rect_path(
        &self,
        canvas: &mut Canvas,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        brush: Brush,
        border_brush: Brush,
        border_thickness: Thickness,
    ) {
        // border
        self.render_rect_path(canvas, x, y, width, height, border_brush);

        // content
        self.render_rect_path(
            canvas,
            x + border_thickness.left,
            y + border_thickness.top,
            width - border_thickness.left - border_thickness.right,
            height - border_thickness.top - border_thickness.bottom,
            brush,
        );
    }

    // Builds rectangle path with radius and without border.
    fn render_rounded_rect_path(
        &self,
        canvas: &mut Canvas,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        radius: f64,
        brush: Brush,
    ) {
        let m_pi = 3.14159265;
        let degrees = m_pi / 180.0;

        // canvas.begin_path();
        canvas.arc(
            x + width - radius,
            y + radius,
            radius,
            -90.0 * degrees,
            0.0 * degrees,
        );
        canvas.arc(
            x + width - radius,
            y + height - radius,
            radius,
            0.0 * degrees,
            90.0 * degrees,
        );
        canvas.arc(
            x + radius,
            y + height - radius,
            radius,
            90.0 * degrees,
            180.0 * degrees,
        );
        canvas.arc(
            x + radius,
            y + radius,
            radius,
            180.0 * degrees,
            270.0 * degrees,
        );

        match brush {
            Brush::SolidColor(color) => canvas.set_fill_style(color),
            _ => {} // todo: gradient
        }

        canvas.fill();
    }

    // Renders rectangle with border and radius.
    fn render_rounded_bordered_rect_path(
        &self,
        canvas: &mut Canvas,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        radius: f64,
        brush: Brush,
        border_brush: Brush,
        border_thickness: Thickness,
    ) {
        // border
        self.render_rounded_rect_path(canvas, x, y, width, height, radius, border_brush);

        // content
        self.render_rounded_rect_path(
            canvas,
            x + border_thickness.left,
            y + border_thickness.top,
            width - border_thickness.left - border_thickness.right,
            height - border_thickness.top - border_thickness.right,
            radius,
            brush,
        );
    }
}

impl Into<Box<dyn RenderObject>> for RectangleRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for RectangleRenderObject {
    fn render(
        &self,
        canvas: &mut Canvas,
        _renderer: &mut dyn Renderer,
        context: &mut Context<'_>,
        global_position: &Point,
    ) {
        let widget = context.widget();

        let bounds = widget.get::<Bounds>();
        let background = widget.clone::<Background>();
        let border_radius = widget.clone_or_default::<BorderRadius>();
        let border_thickness = widget.clone_or_default::<BorderThickness>();
        let border_brush = widget.clone_or_default::<BorderBrush>();

        let has_thickness = border_thickness.0.left > 0.0
            || border_thickness.0.top > 0.0
            || border_thickness.0.right > 0.0
            || border_thickness.0.bottom > 0.0;

        if border_radius.0 > 0.0 {
            if has_thickness {
                self.render_rounded_bordered_rect_path(
                    canvas,
                    global_position.x + bounds.x(),
                    global_position.y + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                    border_radius.0,
                    background.0,
                    border_brush.0,
                    border_thickness.0,
                );
            } else {
                self.render_rounded_rect_path(
                    canvas,
                    global_position.x + bounds.x(),
                    global_position.y + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                    border_radius.0,
                    background.0,
                );
            }
        } else {
            if has_thickness {
                self.render_bordered_rect_path(
                    canvas,
                    global_position.x + bounds.x(),
                    global_position.y + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                    background.0,
                    border_brush.0,
                    border_thickness.0,
                );
            } else {
                self.render_rect_path(
                    canvas,
                    global_position.x + bounds.x(),
                    global_position.y + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                    background.0,
                );
            }
        }
    }
}
