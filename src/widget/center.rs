use std::collections::HashMap;
use std::sync::Arc;

use backend::Backend;
use structs::Rect;
use theme::Selector;
use {
    BoxConstraints, ComponentBox, Drawable, Entity, EntityComponentManager, Layout, LayoutResult,
    Template, Thickness, Widget, Property
};

#[derive(Default)]
pub struct Center {
    child: Option<Arc<Widget>>,
}

impl Center {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn child(&mut self, child: Arc<Widget>) {
        self.child = Some(child);
    }
}

impl Widget for Center {
    fn template(&self) -> Template {
        if let Some(child) = &self.child {
            Template::Single(child.clone())
        } else {
            Template::Empty
        }
    }

    fn properties(&self) -> Vec<Property> {
        vec![
            ComponentBox::new(Selector::new(Some("center"))),
            // ComponentBox::new(Layout::new(Box::new(
            //     |entity: Entity,
            //      ecm: &EntityComponentManager,
            //      bc: &BoxConstraints,
            //      children: &[Entity],
            //      children_pos: &mut HashMap<Entity, (i32, i32)>,
            //      size: Option<(u32, u32)>| {

            //          let padding = {
            //              let mut padding = Thickness::new(0, 0, 0, 0);
            //              if let Ok(pad) = ecm.borrow_component::<Thickness>(entity) {
            //                 padding = *pad;
            //              };

            //              padding
            //          };

            //         if let Some(size) = size {
            //             children_pos.insert(children[0], (padding.left, padding.top));
            //             LayoutResult::Size((
            //                 size.0 + padding.left as u32 + padding.right as u32,
            //                 size.1 + padding.top as u32 + padding.bottom as u32,
            //             ))
            //         } else {
            //             let child_bc = BoxConstraints {
            //                 min_width: (bc.min_width as i32 -  (padding.left + padding.right)).max(0) as u32,
            //                 max_width: (bc.max_width as i32 - (padding.left + padding.right)).max(0) as u32,
            //                 min_height: (bc.min_height as i32 - (padding.top + padding.bottom)).max(0) as u32,
            //                 max_height: (bc.max_height as i32 - (padding.top + padding.bottom)).max(0) as u32,
            //             };
            //             LayoutResult::RequestChild(children[0], child_bc)
            //         }
            //     },
            // ))),
        ]
    }
}
