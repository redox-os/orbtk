
/// Update is used to call the registered update function in a loop
/// that depends on the used platform.
pub struct Update {
    pub update: Box<FnMut() -> bool + Send>,
}

impl Update {
    pub fn new(update: Box<FnMut() -> bool + Send>) -> Self {
        Update {
            update
        }
    }
}

/// This trait is used to define an update loop.
pub trait Runner {
    fn run(&mut self);
}