use std::sync::Arc;

use backend::Backend;
use structs::Rect;
use theme::Selector;
use {ComponentBox, Drawable, Entity, EntityComponentManager, Template, Widget};

#[derive(Default)]
pub struct Container {
    child: Option<Arc<Widget>>,
}

impl Container {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn child(&mut self, child: Arc<Widget>) {
        self.child = Some(child);
    }
}

impl Widget for Container {
    fn template(&self) -> Template {
        if let Some(child) = &self.child {
            Template::Single(child.clone())
        } else {
            Template::Empty
        }
    }

    fn components(&self) -> Vec<ComponentBox> {
        vec![
            ComponentBox::new(Selector::new(Some("border"))),
            ComponentBox::new(Drawable::new(Box::new(
                |entity: Entity, ecm: &EntityComponentManager, renderer: &mut Box<Backend>| {
                    if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
                        if let Ok(bounds) = ecm.borrow_component::<Rect>(entity) {
                            renderer.render_rectangle(bounds, selector);
                        }
                    }
                },
            ))),
        ]
    }
}
