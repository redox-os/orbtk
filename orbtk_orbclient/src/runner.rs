use crate::*;

pub trait Runner {
    fn run(&mut self) -> Result<bool, Error>;
}
