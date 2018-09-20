use std::collections::HashMap;
use structs::Rect;
use theme::Selector;
use {
    Backend, BoxConstraints, ComponentBox, Drawable, Entity, EntityComponentManager, Layout,
    LayoutResult, Widget,
};

pub struct LabelProperties {
    pub label: String,
    pub selector: Selector,
}

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
    fn properties(&self) -> ComponentBox {
        ComponentBox::new(LabelProperties {
            label: String::from("Label"),
            selector: Selector::new(Some("button")),
        })
    }

    fn components(&self) -> Vec<ComponentBox> {
        vec![
            ComponentBox::new(Drawable::new(Box::new(
                |entity: Entity, ecm: &EntityComponentManager, renderer: &mut Box<Backend>| {
                    if let Ok(props) = ecm.borrow_component::<LabelProperties>(entity) {
                        if let Ok(bounds) = ecm.borrow_component::<Rect>(entity) {
                             renderer.render_text(&props.label, bounds, &props.selector);
                        }
                    }
                },
            ))),
            ComponentBox::new(Layout::new(Box::new(
                |_entity: Entity,
                 _ecm: &EntityComponentManager,
                 bc: &BoxConstraints,
                 _children: &[Entity],
                 _children_pos: &mut HashMap<Entity, (i32, i32)>,
                 _size: Option<(u32, u32)>| {
                    LayoutResult::Size(bc.constrain((100, 17)))
                },
            ))),
        ]
    }
}
