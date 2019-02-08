use crate::{
    backend::Renderer,
    properties::{Bounds, FontIcon, PrimaryFontIcon, SecondaryFontIcon},
    render_object::RenderObject,
    structs::Point,
    theme::Selector,
    widget::Context,
};

pub struct FontIconRenderObject;

impl Into<Box<dyn RenderObject>> for FontIconRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for FontIconRenderObject {
    fn render(
        &self,
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

        if let Ok(selector) = widget.borrow_property::<Selector>() {
            if let Ok(bounds) = widget.borrow_property::<Bounds>() {
                let icon = if let Ok(icon) = widget.borrow_property::<FontIcon>() {
                    Some(&icon.0)
                } else if let Ok(icon) = widget.borrow_property::<PrimaryFontIcon>() {
                    Some(&icon.0)
                } else if let Ok(icon) = widget.borrow_property::<SecondaryFontIcon>() {
                    Some(&icon.0)
                } else {
                    None
                };

                if let Some(icon) = icon {
                    if !icon.is_empty() {
                        renderer.render_text(
                            icon,
                            bounds,
                            &parent_bounds,
                            global_position,
                            theme.uint("icon-size", selector),
                            theme.brush("icon-color", selector).into(),
                            &theme.string("icon-family", selector),
                        );
                    }
                }
            }
        }
    }
}
