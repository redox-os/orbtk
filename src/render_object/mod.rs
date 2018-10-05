use std::sync::Arc;

use {Entity, EntityComponentManager, Renderer, Theme};

pub use self::rectangle::*;
pub use self::text::*;

mod rectangle;
mod text;

pub trait RenderObject {
    fn render(
        &self,
        entity: Entity,
        ecm: &EntityComponentManager,
        renderer: &mut Renderer,
        theme: &Arc<Theme>,
        offset: (i32, i32),
    );
}