use std::marker::*;

use legion::*;

use crate::*;

pub struct Ui<S>
where
    S: Default + Clone + PartialEq,
{
    world: World,
    resources: Resources,
    state: S,
    view_builder: Option<Box<dyn Fn(&mut S) -> BuildContext>>,
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

    fn build(&mut self) {
        if let Some(view_builder) = &self.view_builder {
            let (world, resources) = view_builder(&mut self.state).consume();
            // println!("{:?}", world);
            self.world = world;
            self.resources = resources;
        }
    }

    pub fn set_view<F>(&mut self, view_builder: F)
    where
        F: Fn(&mut S) -> BuildContext + 'static,
    {
        self.view_builder = Some(Box::new(view_builder));
        self.build();
    }

    pub fn run(&mut self) {
        let old_state = self.state.clone();

        self.event();

        if self.state != old_state {
            self.build();
        }
    }
}
