use std::collections::HashMap;
use std::sync::Arc;
use structs::Rect;
use theme::Selector;
use {
    Backend, BoxConstraints, ComponentBox, Drawable, Entity, EntityComponentManager, Layout,
    LayoutResult, Widget, Property, Theme
};


// Extract draw and layout
// check to use property vec / hashmap of entity and its children instead of ecm as parameter
// draw(entity: Vec<Components>, children: HashMap<Entity, Vec<Components>) ???

pub struct Label(String);

pub struct TextBlock {
    pub label: String,
    pub class: String,
}

impl Default for TextBlock {
    fn default() -> TextBlock {
        TextBlock {
            label: String::from("TextBlock"),
            class: String::from("textblock"),
        }
    }
}

impl Widget for TextBlock {
    fn properties(&self) -> Vec<Property> {
        vec![
            ComponentBox::new(Label (self.label.clone())),
            ComponentBox::new(Selector::new(Some(self.class.clone()))),
            ComponentBox::new(Drawable::new(Box::new(
                |entity: Entity, ecm: &EntityComponentManager, renderer: &mut Box<Backend>| {
                    if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
                        if let Ok(bounds) = ecm.borrow_component::<Rect>(entity) {
                            if let Ok(label) = ecm.borrow_component::<Label>(entity) {
                                renderer.render_text(&label.0, bounds, selector);
                            }
                        }
                    }
                },
            ))),
            ComponentBox::new(Layout::new(Box::new(
                |entity: Entity,
                 ecm: &EntityComponentManager,
                 bc: &BoxConstraints,
                 _children: &[Entity],
                 _children_pos: &mut HashMap<Entity, (i32, i32)>,
                 _size: Option<(u32, u32)>,
                 _theme: &Arc<Theme> | {
                     if let Ok(label) = ecm.borrow_component::<Label>(entity) {
                         return LayoutResult::Size(bc.constrain((label.0.len() as u32 * 8 + 2, 18)))
                     }
                    
                    LayoutResult::Size((bc.min_width, bc.min_height))
                },
            ))),
        ]
    }
}
