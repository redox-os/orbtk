use orbgl_api::Canvas;

use crate::{
    backend::Renderer,
    properties::{Bounds, Text, WaterMark, Foreground},
    render_object::RenderObject,
    structs::{Point, Color},
    theme::Selector,
    widget::Context,
};

pub struct TextRenderObject;

impl Into<Box<dyn RenderObject>> for TextRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for TextRenderObject {
    fn render(
        &self,
        canvas: &mut Canvas,
        renderer: &mut dyn Renderer,
        context: &mut Context<'_>,
        global_position: &Point,
    ) {
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

        let foreground = widget.get_property::<Foreground>();

        if let Ok(selector) = widget.borrow_property::<Selector>() {
            if let Ok(bounds) = widget.borrow_property::<Bounds>() {
                if let Ok(text) = widget.borrow_property::<Text>() {
                    if !text.0.is_empty() {
                        renderer.render_text(
                            &text.0,
                            bounds,
                            &parent_bounds,
                            global_position,
                            theme.uint("font-size", selector),
                            foreground.into(),
                            &theme.string("font-family", selector),
                        );
                    } 
                    // else if let Ok(text) = widget.borrow_property::<WaterMark>() {
                    //     renderer.render_text(
                    //         &text.0,
                    //         bounds,
                    //         &parent_bounds,
                    //         global_position,
                    //         theme.uint("font-size", selector),
                    //         theme.brush("color", selector).into(),
                    //         &theme.string("font-family", selector),
                    //     );
                    // }
                }
            }
        }
    }
}
