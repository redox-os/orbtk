use legion::*;

#[derive(Debug)]
pub struct Ui<S>
where
    S: Default + Clone + PartialEq,
{
    world: World,
    state: S,
}

impl<S> Ui<S>
where
    S: Default + Clone + PartialEq,
{
    pub fn new(state: S) -> Self {
        Self {
            world: World::default(),
            state,
        }
    }

    fn layout(&mut self) {}

    fn draw(&mut self) {}

    fn event(&mut self) {}

    pub fn run(&mut self) {}
}
