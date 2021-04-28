use std::marker::*;

use legion::*;

use atomic_refcell::*;

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

    fn draw(&mut self) {
        let mut render_query = <&components::RenderComponent>::query();
        // let mut rtx = Ren

        // for render in render_query.iter(&world) {
        //     render.draw(world, rtx)
        // }

        // for bounds in render_query.iter(&self.world) {
        //     println!("{:?}", bounds);

        //     let mut test_query = <&components::TypeComponent>::query();

        //     for type_com in test_query.iter(&self.world) {}
        // }
    }

    fn event(&mut self) {}

    fn build(&mut self) {
        if let Some(view_builder) = &self.view_builder {
            let (world, resources) = view_builder(&mut self.state).consume();
            // println!("{:?}", world);
            self.world = world;
            self.resources = resources;
        }
    }

    /// Inserts a global resource.
    pub fn insert_resource<T: 'static>(&mut self, resource: T) {
        self.resources.insert(resource);
    }

    pub fn resource<T: 'static>(&self) -> Option<AtomicRef<'_, T>> {
        self.resources.get::<T>()
    }
    pub fn mut_resource<T: 'static>(&mut self) -> Option<AtomicRefMut<'_, T>> {
        self.resources.get_mut::<T>()
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

        self.draw();
    }
}
