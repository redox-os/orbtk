use std::marker::*;

use legion::*;

use crate::Widget;

pub struct Ui<S>
where
    S: Default + Clone + PartialEq,
{
    world: World,
    resources: Resources,
    state: S,
    view_builder: Option<Box<dyn Fn(&mut S) -> Box<dyn Widget + 'static>>>,
}

impl<S> Ui<S>
where
    S: Default + Clone + PartialEq,
{
    pub fn new(state: S) -> Self {
        Self {
            world: World::default(),
            resources: Resources::default(),
            state,
            view_builder: None,
        }
    }

    fn layout(&mut self) {}

    fn draw(&mut self) {}

    fn event(&mut self) {}

    pub fn set_view<F>(&mut self, view_builder: F)
    where
        F: Fn(&mut S) -> Box<dyn Widget + 'static> + 'static,
    {
        self.view_builder = Some(Box::new(view_builder));
    }

    pub fn run(&mut self) {}
}
