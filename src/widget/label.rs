use structs::Rect;
use theme::Selector;
use {Backend, ComponentBox, Drawable, Entity, EntityComponentManager, Widget};

pub struct Label {
    pub selector: ComponentBox,
}

impl Label {
    pub fn new(selector: Selector) -> Self {
        Label {
            selector: ComponentBox::new(selector),
        }
    }
}

impl Widget for Label {
    fn components(&self) -> Vec<ComponentBox> {
        vec![
            ComponentBox::new(String::from("Label")),
            ComponentBox::new(Selector::new(Some("button"))),
            ComponentBox::new(Drawable::new(Box::new(
                |entity: Entity, ecm: &EntityComponentManager, renderer: &mut Box<Backend>| {
                    if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
                        if let Ok(bounds) = ecm.borrow_component::<Rect>(entity) {
                            if let Ok(text) = ecm.borrow_component::<String>(entity) {
                                renderer.render_text(text, bounds, selector);
                            }
                        }
                    }
                },
            ))),
        ]
    }
}
