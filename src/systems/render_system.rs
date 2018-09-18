use std::cell::RefCell;

use dces::{Entity, EntityComponentManager, System};

use {Backend, Drawable, Rect, Selector};

pub struct RenderSystem {
    pub renderer: RefCell<Box<Backend>>,
}

impl System for RenderSystem {
    fn run(&self, entities: &Vec<Entity>, ecm: &mut EntityComponentManager) {
        self.renderer.borrow_mut().render();
        for entity in entities {
            if let Ok(drawable) = ecm.borrow_component::<Drawable>(*entity) {
                if let Ok(selector) = ecm.borrow_component::<Selector>(*entity) {
                    if let Ok(bounds) = ecm.borrow_component::<Rect>(*entity) {
                        drawable.draw(bounds, selector, &mut *self.renderer.borrow_mut());
                    }
                }
            }

            if let Ok(selector) = ecm.borrow_component::<Selector>(*entity) {
                println!("{:?}", selector);
            } else {
                println!("No {}", entity);
            }
        }

        self.renderer.borrow_mut().update();
    }
}
