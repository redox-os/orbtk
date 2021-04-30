use legion::world::Entry;

use crate::*;

pub trait RenderObject: std::marker::Send + std::marker::Sync {
    fn draw(&self, entry: Entry, rtx: &mut dyn RenderContext2D);
}
