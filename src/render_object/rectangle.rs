use std::sync::Arc;

use {Entity, EntityComponentManager, Rect, RenderObject, Renderer, Selector, Theme};

pub struct RectangleRenderObject;

impl RenderObject for RectangleRenderObject {
    fn render(
        &self,
        entity: Entity,
        ecm: &EntityComponentManager,
        renderer: &mut Renderer,
        theme: &Arc<Theme>,
        offset: (i32, i32),
    ) {
        if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
            if let Ok(bounds) = ecm.borrow_component::<Rect>(entity) {
                renderer.render_rectangle(theme, bounds, selector, offset);
            }
        }
    }
}
