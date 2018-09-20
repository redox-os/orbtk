use std::any::Any;
use std::sync::Arc;

use dces::ComponentBox;

use Rect;

pub use self::button::*;
pub use self::container::*;
pub use self::flex::*;
pub use self::label::*;

mod button;
mod container;
mod flex;
mod label;

pub enum Template {
    Empty,
    Single(Arc<Widget>),
    Mutli(Vec<Arc<Widget>>),
}

pub trait Widget: Any {
    fn template(&self) -> Template {
        Template::Empty
    }
    fn components(&self) -> Vec<ComponentBox> {
        vec![]
    }
    fn properties(&self) -> ComponentBox {
        ComponentBox::new(Rect::default())
    }

}
