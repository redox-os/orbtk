use legion::World;

use crate::*;

pub trait RenderObject: std::marker::Send + std::marker::Sync {
    fn draw(&self, world: &World, rtx: &mut dyn RenderContext2D);
}
