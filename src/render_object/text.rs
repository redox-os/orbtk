use backend::Renderer;
use render_object::RenderObject;
use properties::{Label, Point, Rect, WaterMark};
use theme::{Selector, Theme};
use widget::WidgetContainer;

pub struct TextRenderObject;

impl Into<Box<RenderObject>> for TextRenderObject {
    fn into(self) -> Box<RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for TextRenderObject {
    fn render(
        &self,
        renderer: &mut Renderer,
        widget: &WidgetContainer,
        theme: &Theme,
        global_position: &Point,
    ) {
        if let Ok(selector) = widget.borrow_property::<Selector>() {
            if let Ok(bounds) = widget.borrow_property::<Rect>() {
                if let Ok(parent_bounds) = widget.borrow_parent_property::<Rect>() {
                    if let Ok(label) = widget.borrow_property::<Label>() {
                        if !label.0.is_empty() {
                            renderer.render_text(
                                &label.0,
                                bounds,
                                parent_bounds,
                                global_position,
                                theme.uint("font-size", selector),
                                theme.color("color", selector),
                                &theme.string("font-family", selector),
                            );
                        } else if let Ok(label) = widget.borrow_property::<WaterMark>() {
                            renderer.render_text(
                                &label.0,
                                bounds,
                                parent_bounds,
                                global_position,
                                theme.uint("font-size", selector),
                                theme.color("color", selector),
                                &theme.string("font-family", selector)
                            );
                        }
                    }
                }
            }
        }
    }
}
