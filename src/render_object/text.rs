use crate::backend::Renderer;
use crate::properties::{Label, Point, Bounds, WaterMark};
use crate::render_object::RenderObject;
use crate::theme::Selector;
use crate::widget::Context;

pub struct TextRenderObject;

impl Into<Box<dyn RenderObject>> for TextRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for TextRenderObject {
    fn render(&self, renderer: &mut dyn Renderer, context: &mut Context<'_>, global_position: &Point) {
        let parent_bounds = if let Some(parent) = context.parent_widget() {
            if let Ok(bounds) = parent.borrow_property::<Bounds>() {
                bounds.clone()
            } else {
                Bounds::default()
            }
        } else {
            Bounds::default()
        };

        let theme = context.theme;
        let widget = context.widget();

        if let Ok(selector) = widget.borrow_property::<Selector>() {
            if let Ok(bounds) = widget.borrow_property::<Bounds>() {
                if let Ok(label) = widget.borrow_property::<Label>() {
                    if !label.0.is_empty() {
                        renderer.render_text(
                            &label.0,
                            bounds,
                            &parent_bounds,
                            global_position,
                            theme.uint("font-size", selector),
                            theme.color("color", selector),
                            &theme.string("font-family", selector),
                        );
                    } else if let Ok(label) = widget.borrow_property::<WaterMark>() {
                        renderer.render_text(
                            &label.0,
                            bounds,
                            &parent_bounds,
                            global_position,
                            theme.uint("font-size", selector),
                            theme.color("color", selector),
                            &theme.string("font-family", selector),
                        );
                    }
                }
            }
        }
    }
}
