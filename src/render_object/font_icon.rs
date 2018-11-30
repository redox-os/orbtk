use backend::Renderer;
use render_object::RenderObject;
use properties::{FontIcon, Point, Rect};
use theme::{Selector, Theme};
use widget::WidgetContainer;

pub struct FontIconRenderObject;

impl Into<Box<RenderObject>> for FontIconRenderObject {
    fn into(self) -> Box<RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for FontIconRenderObject {
    fn render(
        &self,
        renderer: &mut Renderer,
        widget: &WidgetContainer,
        theme: &Theme,
        offset: &Point,
        global_position: &Point,
    ) {
        if let Ok(selector) = widget.borrow_property::<Selector>() {
            if let Ok(bounds) = widget.borrow_property::<Rect>() {
                if let Ok(parent_bounds) = widget.borrow_parent_property::<Rect>() {
                    if let Ok(icon) = widget.borrow_property::<FontIcon>() {
                        if !icon.0.is_empty() {
                            renderer.render_text(
                                &icon.0,
                                bounds,
                                parent_bounds,
                                offset,
                                global_position,
                                theme.uint("icon-size", selector),
                                theme.color("icon-color", selector),
                                &theme.string("icon-font-family", selector)
                            );
                        }
                    }
                }
            }
        }
    }
}
