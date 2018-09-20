use std::cell::RefCell;

use dces::{Entity, EntityComponentManager, System};

use Backend;

pub struct Drawable {
    pub draw_fn: Box<Fn(Entity, &EntityComponentManager, &mut Box<Backend>)>,
}

impl Drawable {
    pub fn new(draw_fn: Box<Fn(Entity, &EntityComponentManager, &mut Box<Backend>)>) -> Self {
        Drawable { draw_fn }
    }

    pub fn draw(&self, entity: Entity, ecm: &EntityComponentManager, renderer: &mut Box<Backend>) {
        (self.draw_fn)(entity, ecm, renderer)
    }
}

pub struct RenderSystem {
    pub renderer: RefCell<Box<Backend>>,
}

impl System for RenderSystem {
    fn run(&self, entities: &Vec<Entity>, ecm: &mut EntityComponentManager) {
        self.renderer.borrow_mut().render();
        for entity in entities {
            if let Ok(drawable) = ecm.borrow_component::<Drawable>(*entity) {
                drawable.draw(*entity, ecm, &mut *self.renderer.borrow_mut());
            }
        }

        self.renderer.borrow_mut().update();
    }
}
