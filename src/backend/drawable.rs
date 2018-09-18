
use {Backend, EntityComponentManager, Entity};

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
