use {Entity, EntityComponentManager, Label, Rect, RenderObject, Renderer, Selector, Theme};

pub struct TextRenderObject;

impl RenderObject for TextRenderObject {
    fn render(
        &self,
        entity: Entity,
        ecm: &EntityComponentManager,
        renderer: &mut Renderer,
        theme: &Theme,
        offset: (i32, i32),
    ) {
        if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
            if let Ok(bounds) = ecm.borrow_component::<Rect>(entity) {
                if let Ok(label) = ecm.borrow_component::<Label>(entity) {
                    renderer.render_text(theme, &label.0, bounds, selector, offset);
                }
            }
        }
    }
}
