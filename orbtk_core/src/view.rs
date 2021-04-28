use crate::*;

pub trait View {
    fn view(&mut self) -> BuildContext;
}
